use crate::prelude::*;

pub fn system_flip(
    mut map: ResMut<Map>,
    mut event_flip: EventReader<FlipEvent>,
    mut event_over: EventWriter<GameOverEvent>,
    mut query: Query<(&TileEntity, &mut Handle<Image>)>,
) {
    for e in event_flip.read() {
        match map.status {
            MapStatus::Win | MapStatus::Lose => {
                continue;
            }
            _ => {
                for (tile, _) in query.iter_mut() {
                    if tile.x == e.x && tile.y == e.y {
                        if map.get_opened() == 0 {
                            map.start(e.x, e.y);
                        }
                        match e.button {
                            FlipType::Open => {
                                map.open(tile.x, tile.y);
                            }
                            FlipType::Mark => {
                                map.mark(tile.x, tile.y);
                            }
                            FlipType::OpenAll => {
                                map.open_all(tile.x, tile.y);
                            }
                        }
                    }
                }
                match map.status {
                    MapStatus::Win => {
                        println!("You Win!");
                        event_over.send(GameOverEvent { win: true });
                    }
                    MapStatus::Lose => {
                        println!("You Lost!");
                        event_over.send(GameOverEvent { win: false });
                    }
                    _ => {}
                }
            }
        }
    }
}

pub fn system_redraw_dirty(
    mut map: ResMut<Map>,
    data: Res<Data>,
    mut query: Query<(&TileEntity, &mut Handle<Image>)>,
) {
    for (tile, mut image) in query.iter_mut() {
        let tile = map.get_tile_mut(tile.x, tile.y).unwrap();
        if tile.get_dirty() {
            *image = data.for_tile(tile);
        }
    }
}
