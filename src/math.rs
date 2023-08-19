pub use solis_physics_sys::Real;
use solis_physics_sys::Sol_ISOMETRY2D_IDENTITY;
pub use solis_physics_sys::Sol_Isometry2D as Isometry2D;
pub use solis_physics_sys::Sol_Vec2 as Vec2;
use solis_physics_sys::*;

pub trait Isometry2DFunctions {
    fn identity() -> Self;
    fn add(&mut self, other: &Isometry2D);
    fn sub(&mut self, other: &Isometry2D);
}

pub trait Vec2Functions {
    fn normalize(&mut self);
    fn rotate(&mut self, rotation: Vec2);
    fn scale(&mut self, scalar: Real);
    fn dot(a: Vec2, b: Vec2) -> Real;
    fn add(&mut self);
    fn sub(&mut self);
    fn mul_add(&mut self);
}

impl Isometry2DFunctions for Isometry2D {
    fn identity() -> Self {
        unsafe { Sol_ISOMETRY2D_IDENTITY }
    }

    fn add(&mut self, other: &Isometry2D) {
        unsafe {
            Sol_Isometry2DAdd(self, other);
        }
    }

    fn sub(&mut self, other: &Isometry2D) {
        unsafe {
            Sol_Isometry2DSub(self, other);
        }
    }
}
