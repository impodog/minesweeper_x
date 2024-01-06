use crate::prelude::*;

#[derive(Component)]
pub struct GameOverTextMarker;

pub fn system_game_over(
    mut commands: Commands,
    data: Res<Data>,
    mut map: Option<ResMut<Map>>,
    mut event_over: EventReader<GameOverEvent>,
    window: Query<&Window>,
) {
    for e in event_over.read() {
        let map = map.as_mut().unwrap();
        let font_size;
        let text = match map.mode {
            GameMode::Classic | GameMode::Flagger => {
                font_size = 100.0;
                if e.win {
                    "You Win!".to_string()
                } else {
                    "You Lose!".to_string()
                }
            }
            GameMode::Endless => {
                let prev = map.endless_score;
                font_size = 70.0;
                map.calc_score();
                let diff = map.endless_score - prev;
                format!(
                    "Game Over! Score: {:.3}({}{:.3})\nAverage: {:.3}(Played {} Rounds)",
                    map.endless_score,
                    if diff > 0.0 { "+" } else { "" },
                    diff,
                    map.avg_score(),
                    map.rounds
                )
            }
        };
        let window = window.single();
        commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(window.width(), window.height())),
                    color: Color::rgba(0.0, 0.0, 0.0, 0.8),
                    ..Default::default()
                },
                texture: data.img_background.clone(),
                ..Default::default()
            })
            .insert((
                GameOverTextMarker,
                Text2dBundle {
                    text: Text::from_section(
                        text,
                        TextStyle {
                            font: data.font.clone(),
                            font_size,
                            color: if e.win { Color::GREEN } else { Color::RED },
                        },
                    ),
                    transform: Transform::from_translation(Vec3::new(0.0, 0.0, 2.0)),
                    ..Default::default()
                },
            ));
    }
}
