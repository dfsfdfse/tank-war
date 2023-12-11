use bevy::prelude::{Vec3, Vec2};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Boundary {
    pub max: Vec3,
    pub min: Vec3,
}

pub trait Vec3Ext {
    fn boundary(&mut self, boundary: &Boundary) -> &mut Self;
    fn scale_all(self, target: &Vec3) -> Vec3;
    fn scale_x(self, target: &Vec3) -> Vec3;
    fn scale_x_val(self, x: f32) -> Vec3;
}

pub trait Vec2Ext {
    fn to_vec3(self) -> Vec3;
}

impl Vec2Ext for Vec2 {
    fn to_vec3(self) -> Vec3 {
        Vec3::new(self.x, self.y, 1.)
    }
}

impl Vec3Ext for Vec3 {
    fn boundary(&mut self, boundary: &Boundary) -> &mut Self {
        if self.x < boundary.min.x {
            self.x = boundary.min.x;
        }
        if self.x > boundary.max.x {
            self.x = boundary.max.x;
        }
        if self.y < boundary.min.y {
            self.y = boundary.min.y;
        }
        if self.y > boundary.max.y {
            self.y = boundary.max.y;
        }
        self
    }

    fn scale_all(self, target: &Vec3) -> Vec3 {
        //获取到目标的缩放比例
        let mut scale = Vec3::new(1., 1., 1.);
        scale.x = target.x / self.x;
        scale.y = target.y / self.y;
        scale.z = target.z / self.z;
        scale
    }

    fn scale_x(self, target: &Vec3) -> Vec3 {
        let mut scale = Vec3::new(1., 1., 1.);
        scale.x = target.x / self.x;
        scale.y = scale.x;
        scale.z = scale.x;
        scale
    }

    fn scale_x_val(self, x: f32) -> Vec3 {
        let mut scale = Vec3::new(1., 1., 1.);
        scale.x = x / self.x;
        scale.y = scale.x;
        scale.z = scale.x;
        scale
    }
}

