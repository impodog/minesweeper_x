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

            if now - *click_time < 0.5 {
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
