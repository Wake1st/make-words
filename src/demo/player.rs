//! Plugin handling the player character in particular.
//! Note that this is separate from the `movement` module as that could be used
//! for other characters as well.

use bevy::{
    ecs::{system::RunSystemOnce as _, world::Command},
    prelude::*,
    render::texture::{ImageLoaderSettings, ImageSampler},
    window::PrimaryWindow,
};

use crate::{asset_tracking::LoadResource, demo::movement::ScreenWrap, screens::Screen, AppSet};

const CURSOR_OFFSET: Vec2 = Vec2::new(318.0, 62.0);

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Player>();
    app.load_resource::<PlayerAssets>();

    // Record directional input as movement controls.
    app.add_systems(Update, animate_cursor.in_set(AppSet::RecordInput));
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Player;

/// A command to spawn the player character.
#[derive(Debug)]
pub struct SpawnPlayer {}

impl Command for SpawnPlayer {
    fn apply(self, world: &mut World) {
        world.run_system_once_with(self, spawn_player);
    }
}

fn spawn_player(
    In(_config): In<SpawnPlayer>,
    mut commands: Commands,
    player_assets: Res<PlayerAssets>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // A texture atlas is a way to split one image with a grid into multiple
    // sprites. By attaching it to a [`SpriteBundle`] and providing an index, we
    // can specify which section of the image we want to see. We will use this
    // to animate our player character. You can learn more about texture atlases in
    // this example: https://github.com/bevyengine/bevy/blob/latest/examples/2d/texture_atlas.rs
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(64), 1, 1, Some(UVec2::splat(1)), None);
    let texture_atlas_layout: Handle<TextureAtlasLayout> = texture_atlas_layouts.add(layout);

    commands.spawn((
        Name::new("Player"),
        Player,
        SpriteBundle {
            texture: player_assets.ducky.clone(),
            transform: Transform::from_scale(Vec2::splat(2.0).extend(1.0)),
            ..Default::default()
        },
        TextureAtlas {
            layout: texture_atlas_layout.clone(),
            index: 0,
        },
        ScreenWrap,
        StateScoped(Screen::Gameplay),
    ));
}

fn animate_cursor(
    q_windows: Query<&Window, With<PrimaryWindow>>,
    mut transform_query: Query<&mut Transform, With<Player>>,
) {
    let window = q_windows.single();
    let window_half_size = Vec2::new(window.width(), window.height()) / 2.0;

    // Games typically only have one window (the primary window)
    if let Some(position) = window.cursor_position() {
        for mut transform in &mut transform_query {
            transform.translation = (window_half_size
                + Vec2::new(position.x + CURSOR_OFFSET.x, -position.y - CURSOR_OFFSET.y))
            .extend(10.0);
        }
    }
}

#[derive(Resource, Asset, Reflect, Clone)]
pub struct PlayerAssets {
    // This #[dependency] attribute marks the field as a dependency of the Asset.
    // This means that it will not finish loading until the labeled asset is also loaded.
    #[dependency]
    pub ducky: Handle<Image>,
}

impl PlayerAssets {
    pub const PATH_DUCKY: &'static str = "images/blue_arrow_cursor.png";
}

impl FromWorld for PlayerAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            ducky: assets.load_with_settings(
                PlayerAssets::PATH_DUCKY,
                |settings: &mut ImageLoaderSettings| {
                    // Use `nearest` image sampling to preserve the pixel art style.
                    settings.sampler = ImageSampler::nearest();
                },
            ),
        }
    }
}
