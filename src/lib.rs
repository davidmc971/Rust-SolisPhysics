pub mod collision_shapes_2d;
pub mod math;

#[cfg(test)]
mod tests {
    use super::collision_shapes_2d::*;
    use super::math::*;

    #[test]
    fn test_solis_physics() {
        let rect1 = Rectangle2D {
            width: 5.0,
            height: 5.0,
        };
        let rect2 = Rectangle2D {
            width: 2.0,
            height: 4.0,
        };
        let mut isometry = Isometry2D::identity();
        isometry.translation = Vec2 { x: 3.0, y: 2.0 };
        isometry.rotation.x = Real::sqrt(2.0) / 2.0;
        isometry.rotation.y = Real::sqrt(2.0) / 2.0;
        let collision_info = check_rectangle_rectangle(&rect1, &rect2, &isometry);
        assert!(collision_info.is_some());
    }
}
