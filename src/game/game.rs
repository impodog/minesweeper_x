use crate::prelude::*;

pub fn system_mouse_listener(
    input: Res<Input<MouseButton>>,
    map: Res<Map>,
    time: Res<Time>,
    window: Query<&Window>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    mut event: EventWriter<FlipEvent>,
    mut click_time: Local<f32>,
) {
    let (camera, camera_transform) = q_camera.single();

    if let Some(pos) = window
        .single()
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        let scale = map.get_scale();
        let (width, height) = map.get_size();
        let x = (pos.x / scale + width as f32 / 2.0).floor() as usize;
        let y = (pos.y / scale + height as f32 / 2.0).floor() as usize;

        if input.just_pressed(MouseButton::Left) {
            let now = time.elapsed_seconds();

            if now - *click_time < 0.3 {
                event.send(FlipEvent {
                    button: FlipType::OpenAll,
                    x,
                    y,
                });
                return;
            } else {
                event.send(FlipEvent {
                    button: FlipType::Open,
                    x,
                    y,
                });
            }
            *click_time = now;
        } else if input.just_pressed(MouseButton::Right) {
            event.send(FlipEvent {
                button: FlipType::Mark,
                x,
                y,
            });
        }
    }
}

pub fn system_keyboard_listener(
    input: Res<Input<KeyCode>>,
    mut map: ResMut<Map>,
    time: Res<Time>,
    mut event: EventWriter<FlipEvent>,
    mut click_time: Local<f32>,
) {
    let (x, y) = map.cursor;
    let mut moved = false;

    if input.just_pressed(KeyCode::J) {
        let now = time.elapsed_seconds();

        if now - *click_time < 0.3 {
            event.send(FlipEvent {
                button: FlipType::OpenAll,
                x,
                y,
            });
            return;
        } else {
            event.send(FlipEvent {
                button: FlipType::Open,
                x,
                y,
            });
        }
        *click_time = now;
    } else if input.just_pressed(KeyCode::K) {
        event.send(FlipEvent {
            button: FlipType::Mark,
            x,
            y,
        });
    } else if input.just_pressed(KeyCode::Space) {
        event.send(FlipEvent {
            button: FlipType::OpenAll,
            x,
            y,
        });
    } else if input.just_pressed(KeyCode::W) {
        map.move_cursor(x, y + 1);
        moved = true;
    } else if input.just_pressed(KeyCode::S) {
        map.move_cursor(x, y.saturating_sub(1));
        moved = true;
    } else if input.just_pressed(KeyCode::A) {
        map.move_cursor(x.saturating_sub(1), y);
        moved = true;
    } else if input.just_pressed(KeyCode::D) {
        map.move_cursor(x + 1, y);
        moved = true;
    } else if input.just_pressed(KeyCode::Left) {
        event.send(FlipEvent {
            button: FlipType::Mark,
            x: x.saturating_sub(1),
            y,
        });
    } else if input.just_pressed(KeyCode::Right) {
        event.send(FlipEvent {
            button: FlipType::Mark,
            x: x + 1,
            y,
        });
    } else if input.just_pressed(KeyCode::Up) {
        event.send(FlipEvent {
            button: FlipType::Mark,
            x,
            y: y + 1,
        });
    } else if input.just_pressed(KeyCode::Down) {
        event.send(FlipEvent {
            button: FlipType::Mark,
            x,
            y: y.saturating_sub(1),
        });
    }

    if moved {
        match map.mode {
            GameMode::Classic => {}
            GameMode::Flagger => {
                event.send(FlipEvent {
                    button: FlipType::Open,
                    x: map.cursor.0,
                    y: map.cursor.1,
                });
                event.send(FlipEvent {
                    button: FlipType::OpenAll,
                    x: map.cursor.0,
                    y: map.cursor.1,
                });
            }
        }
    }
}
