use nalgebra::{Point3, Quaternion, Scalar};
use num_traits::Zero;

pub trait Point {}

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq)]
pub enum PointType<T: Scalar + Zero> {
    XYZ(PointXYZ<T>),
    XYZ_RGBA(PointXYZRGBA<T>),
}

#[repr(align(16))]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct PointXYZ<T: Scalar + Zero> {
    inner: Point3<T>,
}

impl<T: Scalar + Zero> PointXYZ<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        let inner = Point3::new(x, y, z);
        Self { inner }
    }
}

impl<T: Scalar + Zero> Point for PointXYZ<T> {}

// TODO!: This is terrible, fix it
// impl From<&[f32]> for PointXYZ {
//     fn from(value: &[f32]) -> Self {
//         assert!(
//             value.len() >= 3,
//             "Creating a PointXYZ with less than 3 values is impossible"
//         );
//         Self {
//             x: value[0],
//             y: value[1],
//             z: value[2],
//         }
//     }
// }

#[repr(align(4))]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    pub fn r(&self) -> u8 {
        self.r
    }

    pub fn g(&self) -> u8 {
        self.g
    }

    pub fn b(&self) -> u8 {
        self.b
    }

    pub fn a(&self) -> u8 {
        self.a
    }
}

impl From<u32> for Color {
    fn from(value: u32) -> Self {
        Self {
            r: ((value >> 24) & 0xFF) as u8,
            g: ((value >> 16) & 0xFF) as u8,
            b: ((value >> 8) & 0xFF) as u8,
            a: (value & 0xFF) as u8,
        }
    }
}

impl From<f32> for Color {
    fn from(value: f32) -> Self {
        assert!(!value.is_nan(), "Trying to convert a NaN-float to a color");
        assert!(
            !value.is_subnormal(),
            "Trying to convert a subnormal-float to a color"
        );

        // Raw transmute instead of casting, we want to preserve the bits, not the numerical value
        // Check the documentation for f32::to_bits for more information
        let value = value.to_bits();
        Self::from(value)
    }
}

#[repr(align(16))]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct PointXYZRGBA<T: Scalar + Zero> {
    inner: Point3<T>,
    color: Color,
}

impl<T: Scalar + Zero> Point for PointXYZRGBA<T> {}

impl<T: Scalar + Zero> PointXYZRGBA<T> {
    pub fn new(x: T, y: T, z: T, rgba: u32) -> Self {
        let inner = Point3::new(x, y, z);
        Self {
            inner,
            color: Color::from(rgba),
        }
    }

    pub fn new_color_f32(x: T, y: T, z: T, rgba: f32) -> Self {
        let inner = Point3::new(x, y, z);
        Self {
            inner,
            color: Color::from(rgba),
        }
    }

    pub fn x(&self) -> &T {
        &self.inner.x
    }

    pub fn y(&self) -> &T {
        &self.inner.y
    }

    pub fn z(&self) -> &T {
        &self.inner.z
    }

    pub fn r(&self) -> u8 {
        self.color.r
    }

    pub fn g(&self) -> u8 {
        self.color.g
    }

    pub fn b(&self) -> u8 {
        self.color.b
    }

    pub fn a(&self) -> u8 {
        self.color.a
    }

    pub fn color(&self) -> &Color {
        &self.color
    }

    pub fn color_mut(&mut self) -> &mut Color {
        &mut self.color
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ViewPoint<T: Scalar + Zero> {
    point: Point3<T>,
    quaternion: Quaternion<T>,
}

impl<T: Scalar + Zero> ViewPoint<T> {
    pub fn new(point: Point3<T>, quaternion: Quaternion<T>) -> Self {
        Self { point, quaternion }
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum PointCloudType<T: Scalar + Zero> {
    XYZ(PointCloud<PointXYZ<T>>),
    XYZ_RGBA(PointCloud<PointXYZRGBA<T>>),
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct PointCloudHeader {
    sequence: u32,
    timestamp: u64,
    frame_id: String,
}

#[derive(Debug, Default, PartialEq)]
pub struct PointCloud<P>
where
    P: Point,
{
    header: Option<PointCloudHeader>,
    width: usize,
    height: usize,
    points: Vec<P>,
}

impl<P: Point> PointCloud<P> {
    pub fn new(points: Vec<P>, width: usize, height: usize) -> Self {
        Self {
            header: None,
            width,
            height,
            points,
        }
    }
}

#[cfg(test)]
mod tests {}
