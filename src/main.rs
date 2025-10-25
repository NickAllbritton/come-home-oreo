use std::time::Duration;

use bevy::prelude::*;
use bevy::input::common_conditions::input_just_pressed;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, setup)
        .add_systems(Update, execute_animations)
        .add_systems(
            Update,
            (
                // Press the right arrow key to animate the right sprite
                trigger_animation::<OreoWagging>.run_if(input_just_pressed(KeyCode::Space)),
            )
        )
        .run();
}

// This system runs when the user clicks the space key
fn trigger_animation<S: Component>(mut animation: Single<&mut AnimationConfig, With<S>>) {
    // We create a new timer when the animation is triggered
    animation.frame_timer = AnimationConfig::timer_from_fps(animation.fps);
}

#[derive(Component)]
struct AnimationConfig {
    first_sprite_index: usize,
    last_sprite_index: usize,
    fps: u8,
    frame_timer: Timer
}

impl AnimationConfig {
    fn new(first: usize, last: usize, fps: u8) -> Self {
        Self {
            first_sprite_index: first,
            last_sprite_index: last,
            fps,
            frame_timer: Self::timer_from_fps(fps)
        }
    }

    fn timer_from_fps(fps: u8) -> Timer {
        Timer::new(Duration::from_secs_f32(1.0 / (fps as f32)), TimerMode::Once)
    }
}

// This system loops through all the sprites in the 'TextureAtlas', from 'first_sprite_index' to 
// 'last_sprite_index' (both defined in 'AnimationConfig').
fn execute_animations(time: Res<Time>, mut query: Query<(&mut AnimationConfig, &mut Sprite)>) {
    for (mut config, mut sprite) in &mut query {
        // We track how long the current sprite has been displayed for
        config.frame_timer.tick(time.delta());

        // If it has been displayed for the user-defined amount of time (fps)...
        if config.frame_timer.just_finished()
            && let Some(atlas) = &mut sprite.texture_atlas
        {
            if atlas.index == config.last_sprite_index {
                // ...and it IS the last frame, then we move back to the first frame and stop
                atlas.index = config.first_sprite_index;
            } else {
                // ..and it is NOT the last frame, then we move to the next frame...
                atlas.index +=1;
                // ...and reset the frame timer to start counting all over again
                config.frame_timer = AnimationConfig::timer_from_fps(config.fps);
            }
        }
    }
}

#[derive(Component)]
struct OreoWagging;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>
) {
    // Spawn the camera
    commands.spawn(Camera2d);

    // Create a minimal UI explaining how to interact with the example
    commands.spawn((
            Text::new("Press space to wag Oreo's tail"),
            Node {
                position_type: PositionType::Absolute,
                top: px(12),
                left: px(12),
                ..default()
            },
    ));

    // Load the sprite sheet using the 'AssetServer'
    let texture = asset_server.load("resources/sprites/oreo_wagging.png");

    // The sprite sheet has 24 sprites arranged in 4 rows, and they are all 56px x 128px
    let layout = TextureAtlasLayout::from_grid(UVec2::new(56, 128), 23, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    let animation_config_wag = AnimationConfig::new(1, 22, 20);

    // Create the sprite
    commands.spawn((
            Sprite {
                image: texture.clone(),
                texture_atlas: Some(TextureAtlas {
                    layout: texture_atlas_layout.clone(),
                    index: animation_config_wag.first_sprite_index
                }),
                ..default()
            },
            Transform::from_scale(Vec3::splat(0.75)).with_translation(Vec3::new(0.0, 0.0, 0.0)),
            OreoWagging,
            animation_config_wag
    ));
}
