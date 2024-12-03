use bevy::prelude::*;

use crate::{
    demo::letters::letter_loader::{load_letters, LetterList},
    screens::Screen,
    theme::palette::{DRAWER_BACKGROUND_COLOR, DRAWER_BORDER_COLOR},
    AppSet,
};

use super::{
    instructions::{spawn_instruction, Instruction},
    interaction::{CloseDrawer, OpenDrawer},
};

const DRAWER_CLOSED: f32 = -100.0;
const DRAWER_OPEN: f32 = 0.0;
const DRAW_SLIDE_SPEED: f32 = 620.0;

const DRAWER_PADDING: f32 = 24.0;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Gameplay), setup_drawer.after(load_letters))
        .add_systems(
            Update,
            ((show_drawer, hide_drawer), slide_drawer)
                .chain()
                .in_set(AppSet::Update),
        );
}

#[derive(Component)]
pub struct LetterDrawer {
    pub current_position: f32,
    pub opening: bool,
    pub moving: bool,
}

fn setup_drawer(
    mut commands: Commands,
    letter_list: Res<LetterList>,
    asset_server: Res<AssetServer>,
) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    top: Val::Percent(DRAWER_CLOSED),
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                ..default()
            },
            LetterDrawer {
                current_position: DRAWER_CLOSED,
                opening: false,
                moving: false,
            },
            StateScoped(Screen::Gameplay),
        ))
        .with_children(|builder| {
            //  letter drawer
            builder
                .spawn((
                    NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Row,
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            flex_wrap: FlexWrap::Wrap,
                            row_gap: Val::Px(8.),
                            column_gap: Val::Px(8.),
                            height: Val::Auto,
                            width: Val::Percent(60.),
                            padding: UiRect::all(Val::Px(DRAWER_PADDING)),
                            border: UiRect::all(Val::Px(16.0)),
                            ..Default::default()
                        },
                        border_radius: BorderRadius::all(Val::Px(16.0)),
                        background_color: BackgroundColor::from(DRAWER_BACKGROUND_COLOR),
                        border_color: BorderColor::from(DRAWER_BORDER_COLOR),
                        ..default()
                    },
                    StateScoped(Screen::Gameplay),
                ))
                .with_children(|builder| {
                    for letter in letter_list.letters.iter() {
                        let texture: Handle<Image> =
                            asset_server.load(format!("images/letters/{}", letter.asset_path));

                        builder.spawn((
                            letter.clone(),
                            ButtonBundle {
                                style: Style {
                                    width: Val::Px(128.),
                                    height: Val::Px(128.),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                transform: Transform::from_scale(Vec2::splat(1.0).extend(0.0)),
                                image: UiImage {
                                    texture,
                                    ..default()
                                },
                                ..default()
                            },
                            StateScoped(Screen::Gameplay),
                        ));
                    }
                });

            //  spawn instructions
            spawn_instruction(
                builder,
                "Select 'SPACE' to open the letter drawer; 'add' letters with the right mouse button.".into(),
                Val::Percent(-10.0),
                Instruction { index: 0 },
            );
            
            spawn_instruction(
                builder,
                "Move letters around by selecting them with the right mouse button and dragging them around.".into(),
                Val::Percent(-30.0),
                Instruction { index: 1 },
            );
            spawn_instruction(
                builder,
                "'Combine' letters/words by moving them together.".into(),
                Val::Percent(-50.0),
                Instruction { index: 2 },
            );
            spawn_instruction(
                builder,
                "'Disconnect' words by right clicking the red colored links between the letters.".into(),
                Val::Percent(-70.0),
                Instruction { index: 3 },
            );
            spawn_instruction(
                builder,
                "'Remove' words by dragging them over the trash can icon.".into(),
                Val::Percent(-90.0),
                Instruction { index: 4 },
            );

        });
}

fn show_drawer(
    mut open_drawer: EventReader<OpenDrawer>,
    mut drawer_query: Query<&mut LetterDrawer>,
) {
    for _ in open_drawer.read() {
        if let Ok(mut drawer) = drawer_query.get_single_mut() {
            drawer.opening = true;
            drawer.moving = true;
        }
    }
}

fn hide_drawer(
    mut close_drawer: EventReader<CloseDrawer>,
    mut drawer_query: Query<&mut LetterDrawer>,
) {
    for _ in close_drawer.read() {
        if let Ok(mut drawer) = drawer_query.get_single_mut() {
            drawer.opening = false;
            drawer.moving = true;
        }
    }
}

fn slide_drawer(mut drawer_query: Query<(&mut Style, &mut LetterDrawer)>, time: Res<Time>) {
    if let Ok((mut style, mut drawer)) = drawer_query.get_single_mut() {
        //  don't process if nothing is happening
        if !drawer.moving {
            return;
        }

        let adjustment = DRAW_SLIDE_SPEED * time.delta_seconds();

        if drawer.opening {
            let new_position = drawer.current_position + adjustment;
            if new_position < DRAWER_OPEN {
                style.top = Val::Percent(new_position);
                drawer.current_position = new_position;
            } else {
                style.top = Val::Percent(DRAWER_OPEN);
                drawer.current_position = DRAWER_OPEN;
                drawer.moving = false;
            }
        } else {
            let new_position = drawer.current_position - adjustment;
            if new_position > DRAWER_CLOSED {
                style.top = Val::Percent(new_position);
                drawer.current_position = new_position;
            } else {
                style.top = Val::Percent(DRAWER_CLOSED);
                drawer.current_position = DRAWER_CLOSED;
                drawer.moving = false;
            }
        }
    }
}
