use bevy::prelude::*;

#[derive(Component)]
pub struct Position {
    pub x: u32,
    pub y: u32,
}
#[derive(Component)]
pub struct Drawable {
    pub texture_index: u16,
}
#[derive(Component)]
pub struct CanPlayerMove(pub bool);
