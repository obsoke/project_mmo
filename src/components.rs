use bevy::prelude::*;

// COMMON COMPONENTS - START
#[derive(Component)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
pub struct Movable {
    pub auto_despawn: bool, // TODO: Might not be necessary for my game...
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
// COMMON COMPONENTS - END

// PLAYER COMPONENTS - START
#[derive(Component)]
pub struct Player;
// PLAYER COMPONENTS - END

#[derive(Component)]
pub struct Enemy;
