use bevy::prelude::*;

use crate::{demo::movement::ScreenWrap, screens::Screen, AppSet};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, spawn_letter.in_set(AppSet::Update));
}

#[derive(Component)]
pub struct Letter {
    pub value: String,
}

fn spawn_letter(
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(64), 1, 1, Some(UVec2::splat(1)), None);
    let texture_atlas_layout: Handle<TextureAtlasLayout> = texture_atlas_layouts.add(layout);
    let texture = asset_server.load("images/b.png");

    commands.spawn((
        Name::new("Letter"),
        Letter { value: "B".into() },
        SpriteBundle {
            texture,
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
