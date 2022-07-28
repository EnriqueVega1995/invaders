use bevy::prelude::*;
use crate::{GameTextures, WinSize, SPRITE_SCALE, PLAYER_SIZE};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin{
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, player_spawn_system);
    }
}

fn player_spawn_system(
    mut commands: Commands,
    game_texture: Res<GameTextures>,
    win_size: Res<WinSize>,
){
    //Add Player
    let bottom = -win_size.h / 2.0;
    commands.spawn_bundle(SpriteBundle{
        texture: game_texture.player.clone(),
        transform: Transform{
            translation: Vec3::new(0.0, bottom + PLAYER_SIZE.1 / 2.0 * SPRITE_SCALE + 5.0, 10.0),
            scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.0),
            ..Default::default()
        },
        ..Default::default()
    });
}