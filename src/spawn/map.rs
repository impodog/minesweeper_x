use crate::prelude::*;

#[derive(Component)]
pub struct TileEntity {
    pub x: usize,
    pub y: usize,
}

#[derive(Component)]
pub struct CursorMarker;

#[derive(Component)]
pub struct TimingMarker;

fn calc_scale(window: &Window, width: usize, height: usize) -> f32 {
    let window_width = window.width() as usize;
    let window_height = window.height() as usize;
    let scale_x = window_width / width;
    let scale_y = window_height / height;
    scale_x.min(scale_y) as f32
}

pub fn calc_tile_pos(
    width: usize,
    height: usize,
    scale: f32,
    x: usize,
    y: usize,
    z_pos: f32,
) -> Vec3 {
    let pos = Vec3::new(
        (x as f32 - width as f32 / 2.0 + 0.5) * scale,
        (y as f32 - height as f32 / 2.0 + 0.5) * scale,
        z_pos,
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
        let mut map = Map::new(e.width, e.height, e.mines, scale, e.mode);

        map.generate();

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
                            e.width, e.height, scale, x, y, 0.0,
                        )),
                        ..Default::default()
                    },
                ));
            }
        }

        system_help_spawn_appendix(&mut commands, scale, &res, &map, &e);

        match map.mode {
            GameMode::Classic => {}
            GameMode::Flagger => {}
        }

        commands.insert_resource(map);
    }
}

fn system_help_spawn_appendix(
    commands: &mut Commands,
    scale: f32,
    res: &Res<Data>,
    map: &Map,
    e: &GameStartEvent,
) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(scale, scale)),
                color: Color::rgba(1.0, 1.0, 1.0, 0.3),
                ..Default::default()
            },
            texture: res.img_cursor.clone(),
            transform: Transform::from_translation(calc_tile_pos(
                e.width,
                e.height,
                scale,
                map.cursor.0,
                map.cursor.1,
                1.0,
            )),
            ..Default::default()
        },
        CursorMarker,
    ));
    commands.spawn((
        TextBundle {
            text: Text::from_section(
                "0",
                TextStyle {
                    font: res.font.clone(),
                    font_size: 40.0,
                    color: Color::WHITE,
                },
            ),
            global_transform: GlobalTransform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            ..Default::default()
        },
        TimingMarker,
    ));
}

pub fn system_despawn_map(
    mut commands: Commands,
    mut event: EventReader<GameEndEvent>,
    query_tile: Query<Entity, With<TileEntity>>,
    query_cursor: Query<Entity, With<CursorMarker>>,
    query_over: Query<Entity, With<GameOverTextMarker>>,
    query_timing: Query<Entity, With<TimingMarker>>,
) {
    for _ in event.read() {
        for entity in query_tile.iter() {
            commands.entity(entity).despawn_recursive();
        }
        for entity in query_cursor.iter() {
            commands.entity(entity).despawn_recursive();
        }
        for entity in query_over.iter() {
            commands.entity(entity).despawn_recursive();
        }
        for entity in query_timing.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }
}
