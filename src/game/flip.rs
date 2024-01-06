use crate::prelude::*;

pub fn system_flip(
    mut map: ResMut<Map>,
    mut event_flip: EventReader<FlipEvent>,
    mut event_over: EventWriter<GameOverEvent>,
    mut query: Query<(&TileEntity, &mut Handle<Image>)>,
    time: Res<Time>,
) {
    for e in event_flip.read() {
        match map.status {
            MapStatus::Win | MapStatus::Lose => {
                continue;
            }
            _ => {
                for (tile, _) in query.iter_mut() {
                    if tile.x == e.x && tile.y == e.y {
                        if !map.is_started {
                            map.start(e.x, e.y, time.elapsed_seconds());
                        }
                        match e.button {
                            FlipType::Open => {
                                map.open(tile.x, tile.y);
                            }
                            FlipType::Mark => {
                                map.mark(tile.x, tile.y);
                            }
                            FlipType::OpenAll => map.open_all(tile.x, tile.y),
                        }
                    }
                }
                map.check_win();
                match map.status {
                    MapStatus::Win => {
                        println!("You Win!");
                        event_over.send(GameOverEvent { win: true });
                    }
                    MapStatus::Lose => {
                        println!("You Lose!");
                        event_over.send(GameOverEvent { win: false });
                    }
                    _ => {}
                }
            }
        }
    }

    match map.mode {
        GameMode::Classic | GameMode::Endless => {}
        GameMode::Flagger => {
            map.try_close_far();
        }
    }
}

pub fn system_redraw_dirty(
    mut map: ResMut<Map>,
    data: Res<Data>,
    mut query_tile: Query<(&TileEntity, &mut Handle<Image>)>,
    mut query_cursor: Query<(&CursorMarker, &mut Transform)>,
    mut query_timing: Query<(&TimingMarker, &mut Text)>,
    time: Res<Time>,
) {
    for (tile, mut image) in query_tile.iter_mut() {
        let tile = map.get_tile_mut(tile.x, tile.y).unwrap();
        if tile.get_dirty() {
            *image = data.for_tile(tile);
        }
    }

    if map.cursor_dirty {
        for (_, mut transform) in query_cursor.iter_mut() {
            transform.translation = map.position_of(map.cursor.0, map.cursor.1, 1.0);
        }
        map.cursor_dirty = false;
    }

    map.update_time(time.elapsed_seconds());
    for (_, mut text) in query_timing.iter_mut() {
        text.sections[0].value = format!("{:.3}s", map.get_played_time());
    }
}

pub fn system_restart_game(
    mut commands: Commands,
    mut map: ResMut<Map>,
    input: Res<Input<KeyCode>>,
    query_over: Query<Entity, With<GameOverTextMarker>>,
) {
    if input.just_pressed(KeyCode::R) {
        if input.pressed(KeyCode::ShiftLeft) || input.pressed(KeyCode::ShiftRight) {
            map.restart();
        } else {
            map.replay();
        }
        for entity in query_over.iter() {
            commands.entity(entity).despawn();
        }
    }
}
