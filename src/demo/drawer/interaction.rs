use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use crate::{
    demo::letters::letter::{Letter, SpawnLetter},
    AppSet,
};

pub(super) fn plugin(app: &mut App) {
    app.add_event::<OpenDrawer>();
    app.add_event::<CloseDrawer>();

    app.insert_resource(DrawerState { opened: false });
    app.add_systems(
        Update,
        (
            open_drawer
                .in_set(AppSet::RecordInput)
                .run_if(input_just_pressed(KeyCode::Space)),
            letter_selected.in_set(AppSet::RecordInput),
        ),
    );
}

#[derive(Event)]
pub struct OpenDrawer {}

#[derive(Event)]
pub struct CloseDrawer {}

#[derive(Resource)]
pub struct DrawerState {
    pub opened: bool,
}

fn open_drawer(
    mut drawer_state: ResMut<DrawerState>,
    mut open_drawer: EventWriter<OpenDrawer>,
    mut close_drawer: EventWriter<CloseDrawer>,
) {
    //  send the display event
    if drawer_state.opened {
        close_drawer.send(CloseDrawer {});
    } else {
        open_drawer.send(OpenDrawer {});
    }

    //  set new statef
    drawer_state.opened = !drawer_state.opened;
}

fn letter_selected(
    mut interaction_query: Query<
        (&Interaction, &Transform, &Letter),
        (Changed<Interaction>, With<Letter>),
    >,
    mut spawn_letter: EventWriter<SpawnLetter>,
) {
    for (interaction, transform, letter) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                spawn_letter.send(SpawnLetter {
                    letter: letter.clone(),
                    position: transform.translation.xy(),
                });
            }
            _ => (),
        }
    }
}
