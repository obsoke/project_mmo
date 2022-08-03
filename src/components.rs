use bevy::{prelude::*, sprite::Rect};
use bevy_inspector_egui::Inspectable;

// COMMON COMPONENTS - START
#[derive(Component)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
pub struct Movable {
    pub speed: f32,
    pub auto_despawn: bool, // TODO: Might not be necessary for my game...
}

#[derive(Component, Eq, PartialEq, Copy, Clone, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<Direction> for Vec2 {
    fn from(dir: Direction) -> Self {
        match dir {
            Direction::Up => Vec2::new(0., 1.),
            Direction::Down => Vec2::new(0., -1.),
            Direction::Left => Vec2::new(-1., 0.),
            Direction::Right => Vec2::new(1., 0.),
        }
    }
}

#[derive(Component)]
pub struct ObjectDirection {
    pub current_direction: Direction,
    pub previous_direction: Direction,
}
impl ObjectDirection {
    pub fn new(current: Direction) -> Self {
        Self {
            current_direction: current,
            previous_direction: current,
        }
    }
}

#[derive(Component)]
pub struct SpriteSize(pub Vec2);
impl From<(f32, f32)> for SpriteSize {
    fn from(val: (f32, f32)) -> Self {
        SpriteSize(Vec2::new(val.0, val.1))
    }
}

#[derive(Component)]
pub struct FromPlayer;

#[derive(Component)]
pub struct FromEnemy;

/// The Hurtbox defines an area where an object can take damage.
#[derive(Component, Default)]
pub struct Hurtbox {
    pub size: Vec2,
    pub offset: Vec2,
}
impl Hurtbox {
    pub fn new(size: Vec2, offset: Option<Vec2>) -> Self {
        Self {
            size,
            offset: offset.unwrap_or_default(),
        }
    }
}

/// The Hitbox defines an area where damage can be dealt from.
#[derive(Component, Default)]
pub struct Hitbox {
    pub size: Vec2,
    pub offset: Vec2,
}
impl Hitbox {
    pub fn new(size: Vec2, offset: Option<Vec2>) -> Self {
        Self {
            size,
            offset: offset.unwrap_or_default(),
        }
    }
}
// COMMON COMPONENTS - END

// PLAYER COMPONENTS - START
#[derive(Component, Inspectable)]
pub struct Player;
// PLAYER COMPONENTS - END

#[derive(Component)]
pub struct Enemy;
