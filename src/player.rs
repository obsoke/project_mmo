use crate::animation::AnimationTimer;
use crate::components::{Direction, Hitbox, Hurtbox, Movable, Player, Velocity, ObjectDirection};
use crate::GameTextures;
use bevy::prelude::*;

const PLAYER_WALK_DOWN_IDX: [usize; 4] = [0, 1, 2, 3];
const PLAYER_WALK_RIGHT_IDX: [usize; 4] = [4, 5, 6, 7];
const PLAYER_WALK_UP_IDX: [usize; 4] = [8, 9, 10, 11];
const PLAYER_WALK_LEFT_IDX: [usize; 4] = [12, 13, 14, 15];

const PLAYER_ATTACK_DOWN_IDX: [usize; 4] = [0, 1, 2, 3];
const PLAYER_ATTACK_UP_IDX: [usize; 4] = [4, 5, 6, 7];
const PLAYER_ATTACK_RIGHT_IDX: [usize; 4] = [8, 9, 10, 11];
const PLAYER_ATTACK_LEFT_IDX: [usize; 4] = [12, 13, 14, 15];

#[derive(Component)]
pub struct StateTimer(Timer);

#[derive(Component, Eq, PartialEq, Debug)]
pub enum PlayerState {
    Idle,
    Walking,
    Attacking,
}

#[derive(Default)]
pub struct PlayerTextureAtlas {
    walk: Handle<TextureAtlas>,
    attack: Handle<TextureAtlas>,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, player_spawn_system)
            .add_system(player_keyboard_event_system)
            .add_system(animate_player_sprite_system);
    }
}

fn player_spawn_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // create texture atlas for player movement & attacks
    let texture_atlas_walk =
        TextureAtlas::from_grid(game_textures.player.clone(), Vec2::new(16., 32.), 4, 4);

    let mut texture_atlas_attack =
        TextureAtlas::new_empty(game_textures.player.clone(), Vec2::new(272., 256.));

    // "Hacky" way of navigating a non-uniform sprite sheet to add items to a TextureAtlas
    for y in 0..4 {
        for x in 0..4 {
            let min_x = x as f32 * 32.;
            let min_y = 128. + y as f32 * 32.;
            texture_atlas_attack.add_texture(bevy::sprite::Rect {
                min: Vec2::new(min_x, min_y),
                max: Vec2::new(min_x + 32., min_y + 32.),
            });
        }
    }

    let atlas_resource = PlayerTextureAtlas {
        walk: texture_atlases.add(texture_atlas_walk),
        attack: texture_atlases.add(texture_atlas_attack),
    };

    // add player
    commands
        .spawn_bundle(SpriteSheetBundle {
            // texture: game_textures.player.clone(),
            texture_atlas: atlas_resource.walk.clone(),
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
        .insert(PlayerState::Idle)
        .insert(AnimationTimer(Timer::from_seconds(0.1, true)))
        .insert(ObjectDirection::new(Direction::Down))
        .insert(Velocity(Vec2::ZERO));

    // add texture atlas resource for player
    commands.insert_resource(atlas_resource);
}

fn player_keyboard_event_system(
    mut commands: Commands,
    time: Res<Time>,
    kb: Res<Input<KeyCode>>,
    mut query: Query<(
        Entity,
        &mut Velocity,
        &mut ObjectDirection,
        &mut PlayerState,
        Option<&mut StateTimer>,
    )>,
) {
    if let Ok((
        entity,
        mut velocity,
        mut object_direction,
        mut player_state,
        state_timer,
    )) = query.get_single_mut()
    {
        object_direction.previous_direction = object_direction.current_direction;

        // Movement handling
        velocity.0.x = if kb.pressed(KeyCode::Left) {
            object_direction.current_direction = Direction::Left;
            -1.
        } else if kb.pressed(KeyCode::Right) {
            object_direction.current_direction = Direction::Right;
            1.
        } else {
            0.
        };
        velocity.0.y = if kb.pressed(KeyCode::Up) {
            object_direction.current_direction = Direction::Up;
            1.
        } else if kb.pressed(KeyCode::Down) {
            object_direction.current_direction = Direction::Down;
            -1.
        } else {
            0.
        };

        // Finalize/normalize current velocity
        velocity.0 = velocity.0.normalize_or_zero();

        // Update state - as long as previous state timer is not active
        if let Some(mut timer) = state_timer {
            timer.0.tick(time.delta());
            if timer.0.just_finished() {
                // println!("done");
                commands.entity(entity).remove::<StateTimer>();
            } else {
                // return;
            }
        } else {
            if velocity.0 == Vec2::ZERO {
                *player_state = PlayerState::Idle;
            } else if velocity.0 != Vec2::ZERO {
                *player_state = PlayerState::Walking;
            }
            if kb.just_pressed(KeyCode::Space) {
                // println!("setting state to attacking");
                *player_state = PlayerState::Attacking;
                commands
                    .entity(entity)
                    .insert(StateTimer(Timer::from_seconds(0.25, false)));
            }
        }
    }
}

pub fn animate_player_sprite_system(
    time: Res<Time>,
    player_tex_atlases: Res<PlayerTextureAtlas>,
    mut query: Query<(
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &mut Handle<TextureAtlas>,
        &ObjectDirection,
        &PlayerState,
    )>,
) {
    for (mut timer, mut sprite, mut atlas, object_direction, player_state) in
        query.iter_mut()
    {
        // println!("state: {:?}", player_state);
        let direction = object_direction.current_direction;
        let current_direction_array = match player_state {
            PlayerState::Idle | PlayerState::Walking => {
                *atlas = player_tex_atlases.walk.clone();
                let direction = object_direction.current_direction;
                if direction == Direction::Up {
                    PLAYER_WALK_UP_IDX
                } else if direction == Direction::Down {
                    PLAYER_WALK_DOWN_IDX
                } else if direction == Direction::Left {
                    PLAYER_WALK_LEFT_IDX
                } else if direction == Direction::Right {
                    PLAYER_WALK_RIGHT_IDX
                } else {
                    [0, 0, 0, 0]
                }
            }
            PlayerState::Attacking => {
                *atlas = player_tex_atlases.attack.clone();
                if direction == Direction::Up {
                    PLAYER_ATTACK_UP_IDX
                } else if direction == Direction::Down {
                    PLAYER_ATTACK_DOWN_IDX
                } else if direction == Direction::Left {
                    PLAYER_ATTACK_LEFT_IDX
                } else if direction == Direction::Right {
                    PLAYER_ATTACK_RIGHT_IDX
                } else {
                    [0, 0, 0, 0]
                }
            }
        };

        // println!("dir {:?}, prev_dir: {:?}", direction, prev_direction.0);

        if *player_state == PlayerState::Idle {
            sprite.index = current_direction_array[0];
            break;
        }

        timer.tick(time.delta());
        if timer.just_finished() {
            // We have recently changed direction
            if !current_direction_array.contains(&sprite.index) {
                sprite.index = current_direction_array[0];
            } else {
                sprite.index += 1;
                if sprite.index >= *current_direction_array.last().unwrap() {
                    sprite.index = current_direction_array[0];
                }
            }
        }
    }
}
