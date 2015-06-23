use super::Vec2;

/// Circle structure, with center and radius
#[derive(Copy, Clone)]
pub struct Circle {
    pub center: Vec2<f32>,
    pub radius: f32,
}
