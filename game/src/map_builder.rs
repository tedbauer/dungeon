use crate::map::Map;
use crate::map::TileType;
use rand;
use rand::Rng;

const NUM_ROOMS: usize = 10;

struct Room {
    x0: usize,
    x1: usize,
    y0: usize,
    y1: usize,
}

pub struct MapBuilder {
    map: Map,
    rooms: Vec<Room>,
    player_start: (usize, usize),
}

impl MapBuilder {
    pub fn new(map: Map, player_start: (usize, usize)) -> Self {
        Self {
            map,
            rooms: vec![],
            player_start,
        }
    }

    fn fill(&mut self, tile: TileType) {
        self.map.tiles.iter_mut().for_each(|t| *t = tile)
    }

    fn build_random_rooms(&mut self) {
        let mut r = rand::thread_rng();
        for _ in 0..NUM_ROOMS {
            self.rooms.push(Room {
                x0: r.gen_range(1..30),
                x1: r.gen_range(1..30),
                y0: r.gen_range(1..30),
                y1: r.gen_range(1..30),
            })
        }

        for room in &self.rooms {
            for x in room.x0..room.x1 {
                for y in room.y0..room.y1 {
                    self.map.set_tile(
                        x.try_into().unwrap(),
                        y.try_into().unwrap(),
                        TileType::Floor,
                    );
                }
            }
        }
    }

    pub fn build(&mut self) -> Map {
        self.fill(TileType::Wall);
        println!("---");
        println!("{:?}", self.map);
        self.build_random_rooms();
        println!("---");
        println!("{:?}", self.map);
        self.map.clone()
    }
}
