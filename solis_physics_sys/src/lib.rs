#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindgen_ffi_bindings.rs"));

#[cfg(feature = "glam")]
impl From<glam::Vec2> for Sol_Vec2 {
    fn from(value: glam::Vec2) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}

#[cfg(feature = "glam")]
impl From<glam::Vec3> for Sol_Vec2 {
    fn from(value: glam::Vec3) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}

#[test]
fn test_solis_physics() {
    unsafe {
        let rect1 = Sol_ShapeRectangle2D {
            width: 5.0,
            height: 5.0,
        };
        let rect2 = Sol_ShapeRectangle2D {
            width: 2.0,
            height: 4.0,
        };
        let mut isometry = Sol_ISOMETRY2D_IDENTITY;
        isometry.translation = Sol_Vec2 { x: 3.0, y: 2.0 };
        isometry.rotation.x = f32::sqrt(2.0) / 2.0;
        isometry.rotation.y = f32::sqrt(2.0) / 2.0;
        let collision_info =
            std::alloc::alloc(std::alloc::Layout::new::<Sol_CollisionContactInfo2D>())
                as *mut Sol_CollisionContactInfo2D;
        let result =
            Sol_CollisionCheckRectangleRectangle(&rect1, &rect2, &isometry, collision_info);
        assert_eq!(result, 1);
    }
}
