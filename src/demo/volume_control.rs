use bevy::{audio::Volume, prelude::*};

use crate::{screens::Screen, theme::palette::VOLUME_BACKGROUND_COLOR, AppSet};

use super::{dnd::cursor::CursorPosition, letters::window_size::WindowSize};

const TOP_LEFT_SLIDER_ADJUSTMENT: Vec2 = Vec2::new(146., 16.);
const SLIDER_HALF_WIDTH: f32 = 82.;

pub(super) fn plugin(app: &mut App) {
    // app.insert_resource(GlobalAudio(0.5));

    app.add_systems(OnEnter(Screen::Gameplay), setup_volume_slider)
        .add_systems(Update, adjust_volume.in_set(AppSet::RecordInput));
}

// #[derive(Resource)]
// pub struct GlobalAudio(pub f32);

#[derive(Component)]
pub struct VolumeSlider {}

fn setup_volume_slider(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_size: Res<WindowSize>,
) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    top: Val::Px(32.),
                    left: Val::Px(32.),
                    height: Val::Px(32.),
                    width: Val::Px(200.),
                    flex_direction: FlexDirection::Row,
                    ..Default::default()
                },
                ..default()
            },
            StateScoped(Screen::Gameplay),
        ))
        .with_children(|builder| {
            //  volume icon
            let texture: Handle<Image> = asset_server.load(format!("images/volume icon.png"));
            builder.spawn((
                ButtonBundle {
                    style: Style {
                        height: Val::Px(32.),
                        width: Val::Px(32.),
                        ..Default::default()
                    },
                    image: UiImage::new(texture),
                    ..default()
                },
                StateScoped(Screen::Gameplay),
            ));

            //  slider
            let texture: Handle<Image> = asset_server.load(format!("images/volume slider.png"));
            let half_window = window_size.size / 2.0;
            let slider_origin = Vec2::new(
                -half_window.x + TOP_LEFT_SLIDER_ADJUSTMENT.x,
                half_window.y - TOP_LEFT_SLIDER_ADJUSTMENT.y,
            );
            info!("set origin: {:?}", slider_origin);
            builder
                .spawn((
                    NodeBundle {
                        style: Style {
                            height: Val::Px(16.),
                            width: Val::Px(SLIDER_HALF_WIDTH * 2.),
                            margin: UiRect::all(Val::Px(8.)),
                            ..Default::default()
                        },
                        border_radius: BorderRadius::all(Val::Percent(50.)),
                        background_color: BackgroundColor::from(VOLUME_BACKGROUND_COLOR),
                        ..default()
                    },
                    StateScoped(Screen::Gameplay),
                ))
                .with_children(|builder| {
                    builder.spawn((
                        SpriteBundle {
                            texture,
                            transform: Transform::from_xyz(-900., 260., 100.),
                            ..default()
                        },
                        VolumeSlider {},
                        StateScoped(Screen::Gameplay),
                    ));
                });
        });
}

fn adjust_volume(
    buttons: Res<ButtonInput<MouseButton>>,
    mut slider: Query<&mut Transform, With<VolumeSlider>>,
    window_size: Res<WindowSize>,
    cursor_position: Res<CursorPosition>,
    mut global_volume: ResMut<GlobalVolume>,
) {
    if buttons.pressed(MouseButton::Left) {
        let slider_origin = -window_size.size.x / 2.0 + TOP_LEFT_SLIDER_ADJUSTMENT.x;
        let clamped_value = cursor_position.0.x.clamp(
            slider_origin - SLIDER_HALF_WIDTH,
            slider_origin + SLIDER_HALF_WIDTH,
        );
        info!("origin: {:?}\tclamp: {:?}", slider_origin, clamped_value);
        for mut transform in &mut slider {
            let slider = Rect::from_center_half_size(transform.translation.xy(), Vec2::splat(32.));
            if slider.contains(cursor_position.0) {}
            //	adjust the slider
            transform.translation.x = clamped_value;
        }

        //	set the global volume
        global_volume.volume = Volume::new((clamped_value + window_size.size.x / 2.0) / 164.);
    }
}
