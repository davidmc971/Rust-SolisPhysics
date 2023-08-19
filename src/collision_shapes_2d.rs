use super::math::*;
pub use solis_physics_sys::Sol_CollisionContactInfo2D as ContactInfo2D;
pub use solis_physics_sys::Sol_ShapeCapsule2D as Capsule2D;
pub use solis_physics_sys::Sol_ShapeRectangle2D as Rectangle2D;
pub use solis_physics_sys::Sol_ShapeSegment2D as Segment2D;
pub use solis_physics_sys::Sol_ShapeSphere2D as Sphere2D;
use solis_physics_sys::*;

pub trait ContactInfo2DFunctions {
    fn create_zeroed() -> Self;
}

impl ContactInfo2DFunctions for ContactInfo2D {
    fn create_zeroed() -> Self {
        Self {
            distance: 0.0,
            point1: Vec2::create_zeroed(),
            point2: Vec2::create_zeroed(),
            normal1: Vec2::create_zeroed(),
            normal2: Vec2::create_zeroed(),
        }
    }
}

// int Sol_CollisionCheckSphereSphere(Sol_ShapeSphere2D const* s1, Sol_ShapeSphere2D const* s2, Sol_Isometry2D const* difference, Sol_CollisionContactInfo2D* contactInfo);
pub fn check_sphere_sphere(
    s1: &Sphere2D,
    s2: &Sphere2D,
    difference: &Isometry2D,
) -> Option<ContactInfo2D> {
    let mut contact_info = ContactInfo2D::create_zeroed();
    unsafe {
        if Sol_CollisionCheckSphereSphere(s1, s2, difference, &mut contact_info) == 1 {
            return Some(contact_info);
        }
    };
    None
}
// int Sol_CollisionCheckSegmentSegment(Sol_ShapeSegment2D const* s1, Sol_ShapeSegment2D const* s2, Sol_Isometry2D const* difference, Sol_CollisionContactInfo2D* contactInfo);
pub fn check_segment_segment(
    s1: &Segment2D,
    s2: &Segment2D,
    difference: &Isometry2D,
) -> Option<ContactInfo2D> {
    let mut contact_info = ContactInfo2D::create_zeroed();
    unsafe {
        if Sol_CollisionCheckSegmentSegment(s1, s2, difference, &mut contact_info) == 1 {
            return Some(contact_info);
        }
    };
    None
}
// int Sol_CollisionCheckSegmentSphere(Sol_ShapeSegment2D const* segment, Sol_ShapeSphere2D const* sphere, Sol_Isometry2D const* difference, Sol_CollisionContactInfo2D* contactInfo);
pub fn check_segment_sphere(
    segment: &Segment2D,
    sphere: &Sphere2D,
    difference: &Isometry2D,
) -> Option<ContactInfo2D> {
    let mut contact_info = ContactInfo2D::create_zeroed();
    unsafe {
        if Sol_CollisionCheckSegmentSphere(segment, sphere, difference, &mut contact_info) == 1 {
            return Some(contact_info);
        }
    };
    None
}
// int Sol_CollisionCheckRectangleRectangle(Sol_ShapeRectangle2D const* r1, Sol_ShapeRectangle2D const* r2, Sol_Isometry2D const* difference, Sol_CollisionContactInfo2D* contactInfo);
pub fn check_rectangle_rectangle(
    r1: &Rectangle2D,
    r2: &Rectangle2D,
    difference: &Isometry2D,
) -> Option<ContactInfo2D> {
    let mut contact_info = ContactInfo2D::create_zeroed();
    unsafe {
        if Sol_CollisionCheckRectangleRectangle(r1, r2, difference, &mut contact_info) == 1 {
            return Some(contact_info);
        }
    };
    None
}
// int Sol_CollisionCheckRectangleSegment(Sol_ShapeRectangle2D const* r, Sol_ShapeSegment2D const* s, Sol_Isometry2D const* difference, Sol_CollisionContactInfo2D* contactInfo);
pub fn check_rectangle_segment(
    r: &Rectangle2D,
    s: &Segment2D,
    difference: &Isometry2D,
) -> Option<ContactInfo2D> {
    let mut contact_info = ContactInfo2D::create_zeroed();
    unsafe {
        if Sol_CollisionCheckRectangleSegment(r, s, difference, &mut contact_info) == 1 {
            return Some(contact_info);
        }
    };
    None
}
// int Sol_CollisionCheckRectangleSphere(Sol_ShapeRectangle2D const* r, Sol_ShapeSphere2D const* s, Sol_Isometry2D const* difference, Sol_CollisionContactInfo2D* contactInfo);
pub fn check_rectangle_sphere(
    r: &Rectangle2D,
    s: &Sphere2D,
    difference: &Isometry2D,
) -> Option<ContactInfo2D> {
    let mut contact_info = ContactInfo2D::create_zeroed();
    unsafe {
        if Sol_CollisionCheckRectangleSphere(r, s, difference, &mut contact_info) == 1 {
            return Some(contact_info);
        }
    };
    None
}
// int Sol_CollisionCheckCapsuleCapsule(Sol_ShapeCapsule2D const* c1, Sol_ShapeCapsule2D const* c2, Sol_Isometry2D const* difference, Sol_CollisionContactInfo2D* contactInfo);
pub fn check_capsule_capsule(
    c1: &Capsule2D,
    c2: &Capsule2D,
    difference: &Isometry2D,
) -> Option<ContactInfo2D> {
    let mut contact_info = ContactInfo2D::create_zeroed();
    unsafe {
        if Sol_CollisionCheckCapsuleCapsule(c1, c2, difference, &mut contact_info) == 1 {
            return Some(contact_info);
        }
    };
    None
}
// int Sol_CollisionCheckCapsuleRectangle(Sol_ShapeCapsule2D const* c, Sol_ShapeRectangle2D const* r, Sol_Isometry2D const* difference, Sol_CollisionContactInfo2D* contactInfo);
pub fn check_capsule_rectangle(
    c: &Capsule2D,
    r: &Rectangle2D,
    difference: &Isometry2D,
) -> Option<ContactInfo2D> {
    let mut contact_info = ContactInfo2D::create_zeroed();
    unsafe {
        if Sol_CollisionCheckCapsuleRectangle(c, r, difference, &mut contact_info) == 1 {
            return Some(contact_info);
        }
    };
    None
}
// int Sol_CollisionCheckCapsuleSegment(Sol_ShapeCapsule2D const* c, Sol_ShapeSegment2D const* s, Sol_Isometry2D const* difference, Sol_CollisionContactInfo2D* contactInfo);
pub fn check_capsule_segment(
    c: &Capsule2D,
    s: &Segment2D,
    difference: &Isometry2D,
) -> Option<ContactInfo2D> {
    let mut contact_info = ContactInfo2D::create_zeroed();
    unsafe {
        if Sol_CollisionCheckCapsuleSegment(c, s, difference, &mut contact_info) == 1 {
            return Some(contact_info);
        }
    };
    None
}
// int Sol_CollisionCheckCapsuleSphere(Sol_ShapeCapsule2D const* c, Sol_ShapeSphere2D const* s, Sol_Isometry2D const* difference, Sol_CollisionContactInfo2D* contactInfo);
pub fn check_capsule_sphere(
    c: &Capsule2D,
    s: &Sphere2D,
    difference: &Isometry2D,
) -> Option<ContactInfo2D> {
    let mut contact_info = ContactInfo2D::create_zeroed();
    unsafe {
        if Sol_CollisionCheckCapsuleSphere(c, s, difference, &mut contact_info) == 1 {
            return Some(contact_info);
        }
    };
    None
}
