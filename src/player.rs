use crate::animation::AnimationTimer;
use crate::components::{Movable, Player, Velocity};
use crate::{GameTextures, BASE_SPEED, TIME_STEP};
use bevy::{prelude::*, transform};

#[derive(Component, PartialEq, Debug)]
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

const PLAYER_WALK_DOWN_IDX: [usize; 4] = [0, 1, 2, 3];
const PLAYER_WALK_RIGHT_IDX: [usize; 4] = [4, 5, 6, 7];
const PLAYER_WALK_UP_IDX: [usize; 4] = [8, 9, 10, 11];
const PLAYER_WALK_LEFT_IDX: [usize; 4] = [12, 13, 14, 15];

#[derive(PartialEq)]
enum PlayerStates {
    Idle,
    Walking,
}

#[derive(Component)]
pub struct PlayerState(PlayerStates);

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, player_spawn_system)
            .add_system(player_keyboard_event_system)
            .add_system(animate_player_sprite_system)
            .add_system(player_attack_system);
    }
}

fn player_spawn_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // create texture atlas
    let texture_atlas =
        TextureAtlas::from_grid(game_textures.player.clone(), Vec2::new(16., 32.), 4, 4);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    // add player
    commands
        .spawn_bundle(SpriteSheetBundle {
            // texture: game_textures.player.clone(),
            texture_atlas: texture_atlas_handle,
            transform: Transform {
                scale: Vec3::new(4., 4., 1.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Player)
        .insert(Movable {
            auto_despawn: false,
        })
        .insert(PlayerState(PlayerStates::Idle))
        .insert(AnimationTimer(Timer::from_seconds(0.1, true)))
        .insert(Direction::Down)
        .insert(Velocity(Vec2::new(0., 0.)));
}

fn player_attack_system(kb: Res<Input<KeyCode>>, query: Query<(&Transform, &mut PlayerState)>) {
    if let Ok((player_transform, mut player_state)) = query.get_single() {
        if kb.just_pressed(KeyCode::Space) {
            // TODO: Attack in direction we are facing
            println!("ATTACK!");
        }
    }
}

fn player_keyboard_event_system(
    kb: Res<Input<KeyCode>>,
    mut query: Query<(&mut Velocity, &mut Direction, &mut PlayerState), With<Player>>,
) {
    if let Ok((mut velocity, mut direction, mut player_state)) = query.get_single_mut() {
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

        // Change direction based on velocity
        if velocity.0 == Direction::Down.into() {
            *direction = Direction::Down
        } else if velocity.0 == Direction::Left.into() {
            *direction = Direction::Left
        } else if velocity.0 == Direction::Right.into() {
            *direction = Direction::Right
        } else if velocity.0 == Direction::Up.into() {
            *direction = Direction::Up
        };

        // Finalize/normalize current velocity
        velocity.0 = velocity.0.normalize_or_zero();

        if velocity.0 == Vec2::ZERO {
            player_state.0 = PlayerStates::Idle;
        } else {
            player_state.0 = PlayerStates::Walking;
        }
    }
}

pub fn animate_player_sprite_system(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &Direction,
        &PlayerState,
        &Handle<TextureAtlas>,
    )>,
) {
    for (mut timer, mut sprite, direction, player_state, texture_atlas_handle) in query.iter_mut() {
        let current_direction_array = if *direction == Direction::Up {
            PLAYER_WALK_UP_IDX
        } else if *direction == Direction::Down {
            PLAYER_WALK_DOWN_IDX
        } else if *direction == Direction::Left {
            PLAYER_WALK_LEFT_IDX
        } else if *direction == Direction::Right {
            PLAYER_WALK_RIGHT_IDX
        } else {
            [0, 0, 0, 0]
        };

        if player_state.0 == PlayerStates::Idle {
            sprite.index = *current_direction_array.first().unwrap();
            break;
        }

        timer.tick(time.delta());
        if timer.just_finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();

            // We have recently changed direction
            if !current_direction_array.contains(&sprite.index) {
                println!("new");
                sprite.index = current_direction_array[0];
            } else {
                println!("current");
                sprite.index = (sprite.index + 1); // % current_direction_array.len();
                if sprite.index >= *current_direction_array.last().unwrap() {
                    sprite.index = current_direction_array[0];
                }
            }
        }
    }
}
