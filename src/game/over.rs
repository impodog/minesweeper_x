use crate::prelude::*;

#[derive(Component)]
pub struct GameOverTextMarker;

pub fn system_game_over(
    mut commands: Commands,
    data: Res<Data>,
    mut event_over: EventReader<GameOverEvent>,
) {
    for e in event_over.read() {
        commands.spawn((
            GameOverTextMarker,
            Text2dBundle {
                text: Text::from_section(
                    if e.win { "You Win!" } else { "You Lost!" },
                    TextStyle {
                        font: data.font.clone(),
                        font_size: 200.0,
                        color: if e.win { Color::GREEN } else { Color::RED },
                    },
                ),
                ..Default::default()
            },
        ));
    }
}
