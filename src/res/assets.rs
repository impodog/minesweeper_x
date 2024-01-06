use crate::prelude::*;

#[derive(Resource, Debug)]
pub struct Data {
    pub img_flag: Handle<Image>,
    pub img_question: Handle<Image>,
    pub img_mine: Handle<Image>,
    pub img_unknown: Handle<Image>,
    pub img_0: Handle<Image>,
    pub img_1: Handle<Image>,
    pub img_2: Handle<Image>,
    pub img_3: Handle<Image>,
    pub img_4: Handle<Image>,
    pub img_5: Handle<Image>,
    pub img_6: Handle<Image>,
    pub img_7: Handle<Image>,
    pub img_8: Handle<Image>,
    pub img_cursor: Handle<Image>,
    pub img_background: Handle<Image>,
    pub font: Handle<Font>,
}

impl Data {
    fn new(server: Res<AssetServer>) -> Self {
        let result = Self {
            img_flag: server.load("flag.png"),
            img_question: server.load("question.png"),
            img_mine: server.load("mine.png"),
            img_unknown: server.load("unknown.png"),
            img_0: server.load("0.png"),
            img_1: server.load("1.png"),
            img_2: server.load("2.png"),
            img_3: server.load("3.png"),
            img_4: server.load("4.png"),
            img_5: server.load("5.png"),
            img_6: server.load("6.png"),
            img_7: server.load("7.png"),
            img_8: server.load("8.png"),
            img_cursor: server.load("cursor.png"),
            img_background: server.load("background.png"),
            font: server.load("CascadiaCode.ttf"),
        };
        result
    }

    pub fn for_tile(&self, tile: &Tile) -> Handle<Image> {
        match tile.get_type() {
            TileType::Unknown => self.img_unknown.clone(),
            TileType::Open => match tile.get_num() {
                0 => self.img_0.clone(),
                1 => self.img_1.clone(),
                2 => self.img_2.clone(),
                3 => self.img_3.clone(),
                4 => self.img_4.clone(),
                5 => self.img_5.clone(),
                6 => self.img_6.clone(),
                7 => self.img_7.clone(),
                8 => self.img_8.clone(),
                usize::MAX => self.img_mine.clone(),
                _ => unreachable!(),
            },
            TileType::Flag => self.img_flag.clone(),
            TileType::Question => self.img_question.clone(),
        }
    }
}

pub fn system_init_assets(mut commands: Commands, server: Res<AssetServer>) {
    commands.insert_resource(Data::new(server));
}

pub fn system_init_window(mut window: Query<&mut Window>) {
    for mut window in window.iter_mut() {
        window.title = "Minesweeper X".to_string();
    }
}
