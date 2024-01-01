use crate::prelude::*;

#[derive(Component)]
pub struct TileEntity {
    pub x: usize,
    pub y: usize,
}

fn calc_scale(window: &Window, width: usize, height: usize) -> f32 {
    let window_width = window.width() as usize;
    let window_height = window.height() as usize;
    let scale_x = window_width / width;
    let scale_y = window_height / height;
    scale_x.min(scale_y) as f32
}

fn calc_tile_pos(width: usize, height: usize, scale: f32, x: usize, y: usize) -> Vec3 {
    let pos = Vec3::new(
        (x as f32 - width as f32 / 2.0 + 0.5) * scale,
        (y as f32 - height as f32 / 2.0 + 0.5) * scale,
        0.0,
    );
    pos
}

pub fn system_spawn_map(
    mut commands: Commands,
    window: Query<&Window>,
    res: Res<Data>,
    mut events: EventReader<GameStartEvent>,
) {
    for e in events.read() {
        let scale = calc_scale(&window.single(), e.width, e.height);
        let mut map = Map::new(e.width, e.height, e.mines, scale);

        map.randomize();

        for x in 0..e.width {
            for y in 0..e.height {
                commands.spawn((
                    TileEntity { x, y },
                    SpriteBundle {
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(scale, scale)),
                            ..Default::default()
                        },
                        texture: res.img_unknown.clone(),
                        transform: Transform::from_translation(calc_tile_pos(
                            e.width, e.height, scale, x, y,
                        )),
                        ..Default::default()
                    },
                ));
            }
        }

        commands.insert_resource(map);
    }
}

pub fn system_despawn_map(
    mut commands: Commands,
    mut event: EventReader<GameEndEvent>,
    query_tile: Query<Entity, With<TileEntity>>,
    query_over: Query<Entity, With<GameOverTextMarker>>,
) {
    for _ in event.read() {
        for entity in query_tile.iter() {
            commands.entity(entity).despawn_recursive();
        }
        for entity in query_over.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }
}
