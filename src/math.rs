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
    fn rotate(&mut self, rotation: &Vec2);
    fn rotate_rad(&mut self, radians: Real);
    fn scale(&mut self, scalar: Real);
    fn dot(a: &Vec2, b: &Vec2) -> Real;
    fn add(&mut self, other: &Vec2);
    fn sub(&mut self, other: &Vec2);
    fn mul_add(&mut self, other: &Vec2, scalar: Real);
    fn length(vec: &Vec2) -> Real;
    fn length_sq(vec: &Vec2) -> Real;
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

impl Vec2Functions for Vec2 {
    fn normalize(&mut self) {
        unsafe { Sol_Vec2Normalize(self) }
    }

    fn rotate(&mut self, rotation: &Vec2) {
        unsafe { Sol_Vec2Rotate(self, rotation) }
    }

    fn rotate_rad(&mut self, radians: Real) {
        unsafe { Sol_Vec2RotateRad(self, radians) }
    }

    fn scale(&mut self, scalar: Real) {
        unsafe { Sol_Vec2Scale(self, scalar) }
    }

    fn dot(a: &Vec2, b: &Vec2) -> Real {
        unsafe { Sol_Vec2Dot(a, b) }
    }

    fn add(&mut self, other: &Vec2) {
        unsafe { Sol_Vec2Add(self, other) }
    }

    fn sub(&mut self, other: &Vec2) {
        unsafe { Sol_Vec2Sub(self, other) }
    }

    fn mul_add(&mut self, other: &Vec2, scalar: Real) {
        unsafe { Sol_Vec2MulAdd(self, other, scalar) }
    }

    fn length(vec: &Vec2) -> Real {
        unsafe { Sol_Vec2Length(vec) }
    }

    fn length_sq(vec: &Vec2) -> Real {
        unsafe { Sol_Vec2Length2(vec) }
    }
}

pub fn min(a: Real, b: Real) -> Real {
    unsafe { Sol_Min(a, b) }
}
pub fn max(a: Real, b: Real) -> Real {
    unsafe { Sol_Max(a, b) }
}
