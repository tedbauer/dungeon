use crate::map::Map;
use crate::map::TileType;
use rand::Rng;

pub fn create_map() -> Map {
    const NUM_ROOMS: usize = 10;

    let mut tiles = vec![vec![TileType::Floor; 20]; 20];
    for i in 0..20 {
        *tiles.get_mut(0).unwrap().get_mut(i).unwrap() = TileType::Wall;
        *tiles.get_mut(19).unwrap().get_mut(i).unwrap() = TileType::Wall;
        *tiles.get_mut(i).unwrap().get_mut(0).unwrap() = TileType::Wall;
        *tiles.get_mut(i).unwrap().get_mut(19).unwrap() = TileType::Wall;
    }

    /*
    let mut rooms: Vec<(u32, u32, u32, u32)> = Vec::new();

    while rooms.len() < NUM_ROOMS {
        let x: u32 = rand::thread_rng().gen_range(0..20);
        let y: u32 = rand::thread_rng().gen_range(0..20);

        let width = rand::thread_rng().gen_range(0..10);
        let height = rand::thread_rng().gen_range(0..10);

        rooms.push((x, y, width, height));
    }

    for room in rooms {
        for x in room.0..(room.0 + room.2) {
            for y in room.1..(room.1 + room.3) {
                if x < 20 && y < 20 {
                    *tiles
                        .get_mut(x as usize)
                        .unwrap()
                        .get_mut(y as usize)
                        .unwrap() = TileType::Floor;
                }
            }
        }
    }
        */

    println!("{:?}", tiles);
    Map { tiles }
}
