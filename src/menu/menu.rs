use crate::prelude::*;

#[derive(Debug, Clone, Component)]
pub struct InputBox;

pub fn system_spawn_input_box(
    mut commands: Commands,
    mut event: EventReader<SpawnMenuEvent>,
    data: Res<Data>,
) {
    for _ in event.read() {
        commands.spawn((
            InputBox,
            Text2dBundle {
                text: Text {
                    sections: vec![
                        TextSection {
                            value: "".to_string(),
                            style: TextStyle {
                                font: data.font.clone(),
                                font_size: 50.0,
                                color: Color::WHITE,
                            },
                        },
                        TextSection {
                            value: "Input Game Size {9<=width<=32};{9<=height<=28};{9<=mines<=200}"
                                .to_string(),
                            style: TextStyle {
                                font: data.font.clone(),
                                font_size: 30.0,
                                color: Color::CYAN,
                            },
                        },
                    ],
                    ..Default::default()
                },
                ..Default::default()
            },
        ));
    }
}

pub fn system_despawn_input_box(
    mut commands: Commands,
    mut event: EventReader<KillMenuEvent>,
    query: Query<Entity, With<InputBox>>,
) {
    for _ in event.read() {
        for entity in query.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub fn system_input_box(
    mut query: Query<(&mut InputBox, &mut Text)>,
    mut evr_char: EventReader<ReceivedCharacter>,
    kbd: Res<Input<KeyCode>>,
    state: Res<State<GameState>>,
) {
    if *state != GameState::Menu {
        return;
    }

    match query.iter_mut().next() {
        Some((mut _input_box, mut text)) => {
            for ev in evr_char.read() {
                if kbd.just_pressed(KeyCode::Back) {
                    text.sections[0].value.pop();
                } else if ev.char.is_numeric() || ev.char == ';' {
                    text.sections[0].value.push(ev.char);
                    if text.sections.len() > 1 {
                        text.sections.remove(1);
                    }
                }
            }
        }
        None => return,
    }
}

pub fn system_start_game(
    mut query: Query<(&mut InputBox, &mut Text)>,
    selector: Option<Res<Selector>>,
    mut event: EventWriter<GameStartEvent>,
    kbd: ResMut<Input<KeyCode>>,
    state: Res<State<GameState>>,
) {
    if *state != GameState::Menu {
        return;
    }
    if selector.is_none() {
        return;
    }
    let selector = selector.unwrap();

    for (_input_box, text) in query.iter_mut() {
        if kbd.just_pressed(KeyCode::Return) {
            let parts: Vec<_> = text.sections[0].value.split(';').collect();
            if parts.len() != 3 {
                return;
            }

            if let (Ok(width), Ok(height), Ok(mines)) = (
                parts[0].parse::<usize>(),
                parts[1].parse::<usize>(),
                parts[2].parse::<usize>(),
            ) {
                if width < 9
                    || width > 32
                    || height < 9
                    || height > 28
                    || mines < 9
                    || mines > 200
                    || mines > width * height / 2
                {
                    return;
                }
                let mode = selector.index.into();

                match mode {
                    GameMode::Classic => {
                        if mines > width * height / 2 {
                            return;
                        }
                    }
                    GameMode::Flagger => {
                        if mines > width * height / 5 {
                            return;
                        }
                    }
                }

                event.send(GameStartEvent {
                    width,
                    height,
                    mines,
                    mode,
                });
            }
        }
    }
}

pub fn system_end_game(
    mut event: EventWriter<GameEndEvent>,
    kbd: ResMut<Input<KeyCode>>,
    state: Res<State<GameState>>,
) {
    if *state != GameState::Game {
        return;
    }

    if kbd.just_pressed(KeyCode::Escape) {
        event.send(GameEndEvent {});
    }
}

pub fn system_start_game_event_listener(
    mut event: EventReader<GameStartEvent>,
    mut kill_menu: EventWriter<KillMenuEvent>,
    mut state: ResMut<NextState<GameState>>,
) {
    for _ in event.read() {
        state.set(GameState::Game);
        kill_menu.send(KillMenuEvent);
    }
}

pub fn system_end_game_event_listener(
    mut event: EventReader<GameEndEvent>,
    mut spawn_menu: EventWriter<SpawnMenuEvent>,
    mut state: ResMut<NextState<GameState>>,
) {
    for _ in event.read() {
        state.set(GameState::Menu);
        spawn_menu.send(SpawnMenuEvent);
    }
}
