use crate::components::{Enemy, FromPlayer, Movable, Velocity};
use bevy::{prelude::*, render::texture::ImageSettings, sprite::collide_aabb::collide};

mod animation;
mod components;
mod debug;
mod player;

// Asset Constants - BEGIN
const PLAYER_SPRITE: &str = "character.png";
// Asset Constants - END

// Resources - BEGIN
pub struct WinSize {
    pub w: f32,
    pub h: f32,
}

struct GameTextures {
    player: Handle<Image>,
}
// Resources - END

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(WindowDescriptor {
            title: "MMO Game".to_string(),
            height: 720.0,
            width: 1280.0,
            ..Default::default()
        })
        .insert_resource(ImageSettings::default_nearest())
        .add_plugins(DefaultPlugins)
        .add_plugin(debug::DebugPlugin)
        .add_plugin(player::PlayerPlugin)
        .add_startup_system(setup_system)
        .add_system(bevy::window::close_on_esc) // FOR DEV ONLY
        .add_system(movable_system)
        .add_system(player_attack_enemy_system)
        .run();
}

fn setup_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    // add a camera!
    commands.spawn_bundle(Camera2dBundle::default());

    // add WinSize resource
    let win_size = WinSize { w: 1280., h: 720. };
    commands.insert_resource(win_size);

    // create textures & add GameTextures resource
    let game_textures = GameTextures {
        player: asset_server.load(PLAYER_SPRITE),
    };
    commands.insert_resource(game_textures);
}

fn movable_system(
    mut query: Query<(Entity, &Movable, &Velocity, &mut Transform)>,
    time: Res<Time>,
) {
    // TODO: Do we need a reference to the entity here?
    for (_entity, movable, velocity, mut transform) in query.iter_mut() {
        let translation = &mut transform.translation;
        translation.x += velocity.0.x * time.delta_seconds() * movable.speed;
        translation.y += velocity.0.y * time.delta_seconds() * movable.speed;
    }
}

fn player_attack_enemy_system(
    mut commands: Commands,
    attack_query: Query<(Entity, &Transform), With<FromPlayer>>,
    enemy_query: Query<(Entity, &Transform), With<Enemy>>,
) {
    // TODO: Do we need this "attack_entity" reference?
    for (_attack_entity, attack_tf) in attack_query.iter() {
        for (enemy_entity, enemy_tf) in enemy_query.iter() {
            // determine collision
            let collision = collide(
                attack_tf.translation,
                Vec2::ZERO,
                enemy_tf.translation,
                Vec2::ONE,
            );

            if collision.is_some() {
                // TODO: Do something?
                // TODO: If enemy HP <=0, despawn
                commands.entity(enemy_entity).despawn();
            }
        }
    }
}
