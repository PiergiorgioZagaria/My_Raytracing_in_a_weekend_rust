use std::ops;
// use crate::mylib::{HitRecord,HitableList,Ray};

/// A struct that holds 3 coordinates as floats
#[derive(Debug,Clone,Copy,PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Vec3 {
    /// Create a new Vec3
    pub fn new(x: f32,y: f32, z:f32) -> Self{
        Self {x,y,z}
    }
    pub fn length(self) -> f32{
        (self.x*self.x + self.y*self.y + self.z*self.z).sqrt()
    }
    pub fn squared_len(self) -> f32{
        self.x*self.x + self.y*self.y + self.z*self.z
    }
    pub fn dot(self, t: Vec3) -> f32{
        self.x * t.x + self.y * t.y + self.z * t.z
    }
    pub fn cross(self, t:Vec3) -> Self{
        Self {
            x: self.y * t.z - self.z * t.y,
            y: self.z * t.x - self.x * t.z,
            z: self.x * t.y - self.y * t.x
        }
    }
    pub fn unit_vector(self) -> Self{
        self / self.length()
    }

}

impl ops::Add for Vec3 {
    type Output = Self;
    fn add(self, other: Self) -> Self{
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}
impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self,t: Vec3){
        *self = *self + t;
    }
}
impl ops::Sub for Vec3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self{
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}
impl ops::SubAssign for Vec3 {
    fn sub_assign(&mut self,t: Vec3){
        *self = *self - t;
    }
}
impl ops::Mul<Vec3> for Vec3 {
    type Output = Self;
    fn mul(self, other: Self) -> Self{
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}
impl ops::Mul<f32> for Vec3 {
    type Output = Self;
    fn mul(self, t: f32) -> Self{
        Self {
            x: self.x * t,
            y: self.y * t,
            z: self.z * t,
        }
    }
}
impl ops::Mul<Vec3> for f32 {
    type Output = Vec3;
    fn mul(self, v: Vec3) -> Vec3{
        v * self
    }
}
impl ops::MulAssign<Vec3> for Vec3 {
    fn mul_assign(&mut self,t: Vec3){
        *self = *self * t;
    }
}
impl ops::MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self,t: f32){
        *self = *self * t;
    }
}
impl ops::Div<Vec3> for Vec3 {
    type Output = Self;
    fn div(self, other: Self) -> Self{
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}
impl ops::Div<f32> for Vec3 {
    type Output = Self;
    fn div(self, t: f32) -> Self{
        Self {
            x: self.x / t,
            y: self.y / t,
            z: self.z / t,
        }
    }
}
impl ops::DivAssign<Vec3> for Vec3 {
    fn div_assign(&mut self,t: Vec3){
        *self = *self / t;
    }
}
impl ops::DivAssign<f32> for Vec3 {
    fn div_assign(&mut self,t: f32){
        *self = *self / t;
    }
}
