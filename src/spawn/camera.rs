use crate::prelude::*;

#[derive(Component)]
struct CameraMarker;

pub fn system_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), CameraMarker));
}
