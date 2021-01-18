use crate::vec3::Vec3;

pub type Point3 = Vec3;
pub type Color = Vec3;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * std::f64::consts::PI / 180.0
}