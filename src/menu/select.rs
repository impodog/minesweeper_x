use crate::prelude::*;

#[derive(Resource)]
pub struct Selector {
    pub index: usize,
    size: usize,
    length: f32,
}

#[derive(Component)]
pub struct SelectorMarker;

pub fn system_selector(
    selector: Option<ResMut<Selector>>,
    window: Query<&Window>,
    input: Res<Input<MouseButton>>,
    state: Res<State<GameState>>,
) {
    if *state != GameState::Menu {
        return;
    }

    if let Some(mut selector) = selector {
        if input.just_pressed(MouseButton::Left) {
            let pos = window.single().cursor_position().unwrap();
            let index = (pos.x / selector.length) as usize;
            if index < selector.size {
                selector.index = index;
                #[cfg(debug_assertions)]
                println!("Selector index: {}", index);
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
                            color: Color::YELLOW,
                        },
                    ),
                    transform: Transform::from_translation(Vec3::new(
                        i as f32 * length - window.single().width() / 2.0 + length / 2.0,
                        top - MENU_SIZE / 2.0,
                        0.0,
                    )),
                    ..Default::default()
                },
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
