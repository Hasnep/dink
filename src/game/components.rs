#[derive(Clone, Debug, Copy)]
pub struct Position {
    pub x: u32,
    pub y: u32,
}

#[derive(Clone, Debug)]
pub struct Drawable {
    pub texture_index: u16,
}

#[derive(Default)]
pub struct PlayerJustMoved(pub bool);
