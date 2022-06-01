use bevy::prelude::*;

fn player_spawn_system(mut commands: Commands, game_textures: Res<GameTextures>) {
    // add player
    commands.spawn_bundle(SpriteBundle {
        texture: game_textures.player.clone(),
        transform: Transform {
            ..Default::default()
        },
        ..Default::default()
    });
}
