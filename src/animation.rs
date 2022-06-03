use bevy::prelude::*;

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer); // TODO: Where should this live?
