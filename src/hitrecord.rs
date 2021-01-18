use std::rc::Rc;

use crate::{
    material::{Empty, Material},
    ray::Ray,
    utility::Point3,
    vec3::Vec3,
};

#[derive(Clone)]
pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub material: Rc<dyn Material>,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = r.direction().dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal.clone()
        } else {
            -outward_normal.clone()
        }
    }
}

impl Default for HitRecord {
    fn default() -> Self {
        Self {
            point: Point3::default(),
            normal: Vec3::default(),
            material: Rc::new(Empty),
            t: 0.0,
            front_face: false,
        }
    }
}
