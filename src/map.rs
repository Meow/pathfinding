use crate::room::Room;
use crate::tile::{Tile, TileType};
use bevy::prelude::*;
use rand::{seq::SliceRandom, Rng};

#[derive(Clone, Debug, Component, Default)]
pub struct Map {
    pub rooms: Vec<Room>,
    pub objects: Vec<Tile>,
    has_exit: bool,
    has_blue: bool,
    has_orange: bool,
}

impl Map {
    pub fn random() -> Self {
        let mut map = Self::default();
        map.generate();
        map
    }

    pub fn generate(&mut self) {
        let room = Room::prefab("spawn");
        self.rooms.push(room);
        self.generate_adjacent();

        if !self.has_exit || !self.has_blue || !self.has_orange {
            self.has_exit = false;
            self.has_blue = false;
            self.has_orange = false;

            self.rooms.clear();
            self.generate();

            return;
        }

        self.generate_items(15);
    }

    fn generate_adjacent(&mut self) {
        let last_room = self.rooms.last().unwrap();
        let id = last_room.id.clone();
        let pos = last_room.pos;

        if !self.room_exists(pos + Vec2 { x: -5.0, y: 0.0 }) && self.gen_left(&id, &pos) {
            self.generate_adjacent();
        }

        if !self.room_exists(pos + Vec2 { x: 5.0, y: 0.0 }) && self.gen_right(&id, &pos) {
            self.generate_adjacent();
        }

        if !self.room_exists(pos + Vec2 { x: 0.0, y: 5.0 }) && self.gen_top(&id, &pos) {
            self.generate_adjacent();
        }

        if !self.room_exists(pos + Vec2 { x: 0.0, y: -5.0 }) && self.gen_bottom(&id, &pos) {
            self.generate_adjacent();
        }
    }

    fn generate_items(&mut self, item_count: u32) {
        let mut rng = rand::thread_rng();
        let mut items = 0;

        while items < item_count {
            for room in self.rooms.iter() {
                for tile in room.tiles.iter() {
                    if tile.tile_type == TileType::Brush && rng.gen::<f32>() > 0.99 {
                        self.objects.push(Tile {
                            pos: room.pos + tile.pos,
                            tile_type: TileType::Item,
                            texture_path: "cube_32x32.png".to_string(),
                        });

                        items += 1;

                        if items >= item_count {
                            break;
                        }
                    }
                }
            }
        }
    }

    fn room_exists(&self, pos: Vec2) -> bool {
        for room in &self.rooms {
            if room.pos.x == pos.x && room.pos.y == pos.y {
                return true;
            }
        }

        false
    }

    fn gen_left(&mut self, id: &str, pos: &Vec2) -> bool {
        let can_generate = matches!(
            id,
            "spawn"
                | "c_room_1"
                | "c_room_2"
                | "m_room_0"
                | "m_room_1"
                | "m_room_2"
                | "m_room_3"
                | "exit"
        );

        if can_generate {
            let variants = vec![
                "m_room_0",
                "c_room_3",
                "c_room_0",
                "d_room_2",
                "d_room_2",
                "d_room_2",
                "d_room_2",
                "d_room_2",
                "exit",
                "m_room_1",
                "m_room_2",
                "m_room_3",
                "p_room_blue",
            ];
            let mut pick = variants.choose(&mut rand::thread_rng()).unwrap_or(&"");

            if pick == &"" {
                return false;
            }

            while (self.has_exit && pick == &"exit") || (self.has_blue && pick == &"p_room_blue") {
                pick = variants.choose(&mut rand::thread_rng()).unwrap_or(&"");

                if pick == &"" {
                    return false;
                }
            }

            if pick == &"exit" {
                self.has_exit = true;
            } else if pick == &"p_room_blue" {
                self.has_blue = true;
            }

            let mut room = Room::prefab(pick);
            room.pos = *pos + Vec2 { x: -5.0, y: 0.0 };
            self.rooms.push(room);
            true
        } else {
            false
        }
    }

    fn gen_top(&mut self, id: &str, pos: &Vec2) -> bool {
        let can_generate = matches!(
            id,
            "spawn" | "c_room_2" | "c_room_3" | "m_room_0" | "m_room_1" | "m_room_3"
        );

        if can_generate {
            let variants = vec![
                "c_room_0", "c_room_1", "d_room_3", "d_room_3", "d_room_3", "d_room_3", "d_room_3",
                "exit", "m_room_0", "m_room_1", "m_room_2",
            ];
            let mut pick = variants.choose(&mut rand::thread_rng()).unwrap_or(&"");

            if pick == &"" {
                return false;
            }

            let room_count = self.rooms.len();

            while (self.has_exit || room_count < 10) && pick == &"exit" {
                pick = variants.choose(&mut rand::thread_rng()).unwrap_or(&"");

                if pick == &"" {
                    return false;
                }
            }

            if pick == &"exit" {
                self.has_exit = true;
            }

            let mut room = Room::prefab(pick);
            room.pos = *pos + Vec2 { x: 0.0, y: 5.0 };
            self.rooms.push(room);
            true
        } else {
            false
        }
    }

    fn gen_right(&mut self, id: &str, pos: &Vec2) -> bool {
        let can_generate = matches!(
            id,
            "spawn"
                | "m_room_0"
                | "c_room_0"
                | "c_room_3"
                | "exit"
                | "m_room_1"
                | "m_room_2"
                | "m_room_3"
        );

        if can_generate {
            let variants = vec![
                "c_room_1",
                "c_room_2",
                "m_room_0",
                "m_room_1",
                "m_room_2",
                "m_room_3",
                "exit",
                "d_room_0",
                "d_room_0",
                "d_room_0",
                "d_room_0",
                "d_room_0",
                "p_room_orange",
            ];
            let mut pick = variants.choose(&mut rand::thread_rng()).unwrap_or(&"");

            if pick == &"" {
                return false;
            }

            while (self.has_exit && pick == &"exit")
                || (self.has_orange && pick == &"p_room_orange")
            {
                pick = variants.choose(&mut rand::thread_rng()).unwrap_or(&"");

                if pick == &"" {
                    return false;
                }
            }

            if pick == &"exit" {
                self.has_exit = true;
            } else if pick == &"p_room_orange" {
                self.has_orange = true;
            }

            let mut room = Room::prefab(pick);
            room.pos = *pos + Vec2 { x: 5.0, y: 0.0 };
            self.rooms.push(room);
            true
        } else {
            false
        }
    }

    fn gen_bottom(&mut self, id: &str, pos: &Vec2) -> bool {
        let can_generate = matches!(
            id,
            "c_room_0" | "c_room_1" | "exit" | "m_room_0" | "m_room_1" | "m_room_2"
        );

        if can_generate {
            let variants = vec![
                "c_room_2", "c_room_3", "d_room_1", "d_room_1", "d_room_1", "d_room_1", "d_room_1",
                "m_room_0", "m_room_1", "m_room_3",
            ];
            let pick = variants.choose(&mut rand::thread_rng()).unwrap_or(&"");

            if pick == &"" {
                return false;
            }

            let mut room = Room::prefab(pick);
            room.pos = *pos + Vec2 { x: 0.0, y: -5.0 };
            self.rooms.push(room);
            true
        } else {
            false
        }
    }
}
