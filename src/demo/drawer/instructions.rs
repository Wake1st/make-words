use bevy::{color::palettes::css::BLACK, prelude::*};

use crate::{screens::Screen, AppSet};

pub(super) fn plugin(app: &mut App) {
    app.add_event::<IterateInstruction>();

    app.insert_resource(CurrentIndex(0));

    app.add_systems(Update, interate_instruction.in_set(AppSet::Update));
}

#[derive(Component)]
pub struct Instruction {
    pub index: usize,
}

pub fn spawn_instruction(
    builder: &mut ChildBuilder,
    text: String,
    position: Val,
    instruction: Instruction,
) {
    builder.spawn((
        TextBundle {
            text: Text {
                sections: vec![TextSection {
                    value: text,
                    style: TextStyle {
                        color: BLACK.into(),
                        font_size: 32.0,
                        ..default()
                    },
                }],
                justify: JustifyText::Center,
                ..default()
            },
            style: Style {
                display: if instruction.index != 0 {
                    Display::None
                } else {
                    Display::default()
                },
                position_type: PositionType::Absolute,
                bottom: position,
                ..Default::default()
            },
            ..default()
        },
        instruction,
        StateScoped(Screen::Gameplay),
    ));
}

#[derive(Event)]
pub struct IterateInstruction {
    pub index: usize,
}

#[derive(Resource)]
struct CurrentIndex(pub usize);

fn interate_instruction(
    mut iteration: EventReader<IterateInstruction>,
    mut current_index: ResMut<CurrentIndex>,
    mut instructions: Query<(&mut Style, &Instruction)>,
) {
    for event in iteration.read() {
        if current_index.0 == event.index {
            for (mut style, instruction) in instructions.iter_mut() {
                //  hide old instruction
                if instruction.index == event.index {
                    style.display = Display::None;
                }

                //  show new instruction
                if instruction.index == event.index + 1 {
                    style.display = Display::Flex;
                }
            }

            //  increment
            current_index.0 += 1;
        }
    }
}
