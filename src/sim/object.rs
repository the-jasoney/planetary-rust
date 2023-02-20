use super::vec2::Vec2;

#[derive(Clone, Copy)]
pub struct Object {
    pub position: Vec2,
    pub velocity: Vec2,
    pub acceleration: Vec2,
    pub constant_pos: bool,
    pub mass: f64
}
