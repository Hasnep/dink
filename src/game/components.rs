#[derive(Clone, Copy)]
pub struct Position {
    pub x: u32,
    pub y: u32,
}

pub struct Drawable {
    pub texture_index: u16,
}

#[derive(Clone, Copy)]
pub enum Action {
    StayStill,
    North,
    South,
    West,
    East,
}

pub struct GoingToTakeAction {
    pub action: Option<Action>,
}

pub struct HaveUpdatedTilemap(pub bool);

pub struct PlayerTag;

pub struct EnemyTag;

pub struct MoveRandomlyTag;
