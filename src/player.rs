use crate::components::{Movable, Player, Velocity};
use crate::{GameTextures, BASE_SPEED, TIME_STEP};
use bevy::{prelude::*, transform};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, player_spawn_system)
            // .add_system(player_movement_system)
            .add_system(player_keyboard_event_system)
            .add_system(player_attack_system);
    }
}

fn player_spawn_system(mut commands: Commands, game_textures: Res<GameTextures>) {
    // add player
    commands
        .spawn_bundle(SpriteBundle {
            texture: game_textures.player.clone(),
            transform: Transform {
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Player)
        .insert(Movable {
            auto_despawn: false,
        })
        .insert(Velocity(Vec2::new(0., 0.)));
}

fn player_attack_system(kb: Res<Input<KeyCode>>, query: Query<&Transform, With<Player>>) {
    if let Ok(player_transform) = query.get_single() {
        if kb.just_pressed(KeyCode::Space) {
            // TODO: Attack in direction we are facing
            println!("ATTACK!");
        }
    }
}

fn player_keyboard_event_system(
    kb: Res<Input<KeyCode>>,
    mut query: Query<&mut Velocity, With<Player>>,
) {
    if let Ok(mut velocity) = query.get_single_mut() {
        velocity.0.x = if kb.pressed(KeyCode::Left) {
            -1.
        } else if kb.pressed(KeyCode::Right) {
            1.
        } else {
            0.
        };

        velocity.0.y = if kb.pressed(KeyCode::Up) {
            1.
        } else if kb.pressed(KeyCode::Down) {
            -1.
        } else {
            0.
        };

        velocity.0 = velocity.0.normalize_or_zero();
    }
}
