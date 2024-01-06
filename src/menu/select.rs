use crate::prelude::*;

#[derive(Resource)]
pub struct Selector {
    pub index: usize,
    size: usize,
    length: f32,
}

#[derive(Component)]
pub struct SelectorMarker;

#[derive(Component)]
pub struct SelectorIndex(pub usize);

pub fn system_selector(
    selector: Option<ResMut<Selector>>,
    window: Query<&Window>,
    input: Res<Input<MouseButton>>,
    state: Res<State<GameState>>,
    mut q_text: Query<(&mut Text, &SelectorIndex)>,
) {
    if *state != GameState::Menu {
        return;
    }

    let mut dirty = false;

    if let Some(mut selector) = selector {
        if input.just_pressed(MouseButton::Left) {
            let pos = window.single().cursor_position().unwrap();
            let index = (pos.x / selector.length) as usize;
            if index < selector.size {
                selector.index = index;
                dirty = true;
                #[cfg(debug_assertions)]
                println!("Selector index: {}", index);
            }
        }
        if dirty {
            for (mut text, index) in q_text.iter_mut() {
                if index.0 == selector.index {
                    text.sections[0].style.color = Color::GREEN;
                } else {
                    text.sections[0].style.color = Color::TEAL;
                }
            }
        }
    }
}

pub fn system_spawn_selector(
    mut commands: Commands,
    data: Res<Data>,
    window: Query<&Window>,
    mut event: EventReader<SpawnMenuEvent>,
) {
    for _ in event.read() {
        let top = window.single().height() / 2.0;
        let length = window.single().width() / MENU_SELECTION.len() as f32;
        commands.insert_resource(Selector {
            index: 0,
            size: MENU_SELECTION.len(),
            length: length,
        });

        for i in 0..MENU_SELECTION.len() {
            commands.spawn((
                Text2dBundle {
                    text: Text::from_section(
                        MENU_SELECTION[i],
                        TextStyle {
                            font: data.font.clone(),
                            font_size: MENU_SIZE / 2.0,
                            color: if i == 0 { Color::GREEN } else { Color::TEAL },
                        },
                    ),
                    transform: Transform::from_translation(Vec3::new(
                        i as f32 * length - window.single().width() / 2.0 + length / 2.0,
                        top - MENU_SIZE / 2.0,
                        0.0,
                    )),
                    ..Default::default()
                },
                SelectorIndex(i),
                SelectorMarker,
            ));
        }
    }
}

pub fn system_despawn_selector(
    mut commands: Commands,
    query: Query<Entity, With<SelectorMarker>>,
    mut event: EventReader<KillMenuEvent>,
) {
    for _ in event.read() {
        for entity in query.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }
}
