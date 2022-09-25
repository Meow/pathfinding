use crate::room::Room;
use crate::vector2::Vector2;
use bevy::prelude::*;
use rand::seq::SliceRandom;

#[derive(Clone, Debug, Component)]
pub struct Map {
    pub rooms: Vec<Room>,
    has_exit: bool,
}

impl Map {
    pub fn random() -> Self {
        let mut map = Map::default();
        map.generate();
        map
    }

    pub fn generate(&mut self) {
        let room = Room::prefab("spawn");
        self.rooms.push(room);
        self.generate_adjacent();

        if !self.has_exit {
            self.rooms.clear();
            self.generate();
        }
    }

    fn generate_adjacent(&mut self) {
        let last_room = self.rooms.last().unwrap();
        let id = last_room.id.clone();
        let pos = last_room.pos.clone();

        if !self.room_exists(&pos + &Vector2 { x: -5.0, y: 0.0 }) && self.gen_left(&id, &pos) {
            self.generate_adjacent();
        }

        if !self.room_exists(&pos + &Vector2 { x: 5.0, y: 0.0 }) && self.gen_right(&id, &pos) {
            self.generate_adjacent();
        }

        if !self.room_exists(&pos + &Vector2 { x: 0.0, y: -5.0 }) && self.gen_top(&id, &pos) {
            self.generate_adjacent();
        }

        if !self.room_exists(&pos + &Vector2 { x: 0.0, y: 5.0 }) && self.gen_bottom(&id, &pos) {
            self.generate_adjacent();
        }
    }

    fn room_exists(&self, pos: Vector2) -> bool {
        for room in &self.rooms {
            if room.pos.x == pos.x && room.pos.y == pos.y {
                return true;
            }
        }

        false
    }

    fn gen_left(&mut self, id: &String, pos: &Vector2) -> bool {
        let can_generate = match id.as_str() {
            "spawn" => true,
            "c_room_1" => true,
            "c_room_2" => true,
            "m_room_0" => true,
            "m_room_1" => true,
            "m_room_2" => true,
            "m_room_3" => true,
            "exit" => true,
            _ => false,
        };

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
            let mut pick = variants
                .choose(&mut rand::thread_rng())
                .unwrap_or_else(|| &"");

            if pick == &"" {
                return false;
            }

            let room_count = self.rooms.len();

            while (self.has_exit || room_count < 10) && pick == &"exit" {
                pick = variants
                    .choose(&mut rand::thread_rng())
                    .unwrap_or_else(|| &"");

                if pick == &"" {
                    return false;
                }
            }

            if pick == &"exit" {
                self.has_exit = true;
            }

            let mut room = Room::prefab(&pick);
            room.pos = pos + &Vector2 { x: -5.0, y: 0.0 };
            self.rooms.push(room);
            true
        } else {
            false
        }
    }

    fn gen_top(&mut self, id: &String, pos: &Vector2) -> bool {
        let can_generate = match id.as_str() {
            "spawn" => true,
            "c_room_2" => true,
            "c_room_3" => true,
            "m_room_0" => true,
            "m_room_1" => true,
            "m_room_3" => true,
            _ => false,
        };

        if can_generate {
            let variants = vec![
                "c_room_0", "c_room_1", "d_room_3", "d_room_3", "d_room_3", "d_room_3", "d_room_3",
                "exit", "m_room_0", "m_room_1", "m_room_2",
            ];
            let mut pick = variants
                .choose(&mut rand::thread_rng())
                .unwrap_or_else(|| &"");

            if pick == &"" {
                return false;
            }

            let room_count = self.rooms.len();

            while (self.has_exit || room_count < 10) && pick == &"exit" {
                pick = variants
                    .choose(&mut rand::thread_rng())
                    .unwrap_or_else(|| &"");

                if pick == &"" {
                    return false;
                }
            }

            if pick == &"exit" {
                self.has_exit = true;
            }

            let mut room = Room::prefab(&pick);
            room.pos = pos + &Vector2 { x: 0.0, y: -5.0 };
            self.rooms.push(room);
            true
        } else {
            false
        }
    }

    fn gen_right(&mut self, id: &String, pos: &Vector2) -> bool {
        let can_generate = match id.as_str() {
            "spawn" => true,
            "m_room_0" => true,
            "c_room_0" => true,
            "c_room_3" => true,
            "exit" => true,
            "m_room_1" => true,
            "m_room_2" => true,
            "m_room_3" => true,
            _ => false,
        };

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
            let mut pick = variants
                .choose(&mut rand::thread_rng())
                .unwrap_or_else(|| &"");

            if pick == &"" {
                return false;
            }

            let room_count = self.rooms.len();

            while (self.has_exit || room_count < 10) && pick == &"exit" {
                pick = variants
                    .choose(&mut rand::thread_rng())
                    .unwrap_or_else(|| &"");

                if pick == &"" {
                    return false;
                }
            }

            if pick == &"exit" {
                self.has_exit = true;
            }

            let mut room = Room::prefab(&pick);
            room.pos = pos + &Vector2 { x: 5.0, y: 0.0 };
            self.rooms.push(room);
            true
        } else {
            false
        }
    }

    fn gen_bottom(&mut self, id: &String, pos: &Vector2) -> bool {
        let can_generate = match id.as_str() {
            "c_room_0" => true,
            "c_room_1" => true,
            "exit" => true,
            "m_room_0" => true,
            "m_room_1" => true,
            "m_room_2" => true,
            _ => false,
        };

        if can_generate {
            let variants = vec![
                "c_room_2", "c_room_3", "d_room_1", "d_room_1", "d_room_1", "d_room_1", "d_room_1",
                "m_room_0", "m_room_1", "m_room_3",
            ];
            let mut pick = variants
                .choose(&mut rand::thread_rng())
                .unwrap_or_else(|| &"");

            if pick == &"" {
                return false;
            }

            let mut room = Room::prefab(&pick);
            room.pos = pos + &Vector2 { x: 0.0, y: 5.0 };
            self.rooms.push(room);
            true
        } else {
            false
        }
    }
}

impl Default for Map {
    fn default() -> Self {
        Self {
            rooms: vec![],
            has_exit: false,
        }
    }
}
