use crate::map::Map;
use crate::map::TileType;
use rand::Rng;

pub fn create_map() -> Map {
    const NUM_ROOMS: usize = 4;

    let mut tiles = vec![vec![TileType::Wall; 30]; 30];
    /*
    for i in 0..30 {
        *tiles.get_mut(0).unwrap().get_mut(i).unwrap() = TileType::Wall;
        *tiles.get_mut(29).unwrap().get_mut(i).unwrap() = TileType::Wall;
        *tiles.get_mut(i).unwrap().get_mut(0).unwrap() = TileType::Wall;
        *tiles.get_mut(i).unwrap().get_mut(29).unwrap() = TileType::Wall;
    }
        */

    let mut rooms: Vec<(u32, u32, u32, u32)> = Vec::new();

    while rooms.len() < NUM_ROOMS {
        let x: u32 = rand::thread_rng().gen_range(1..30);
        let y: u32 = rand::thread_rng().gen_range(1..30);

        let width = rand::thread_rng().gen_range(8..12);
        let height = rand::thread_rng().gen_range(8..12);

        //   _____
        //   |   |
        //   |   |
        //   |___|
        if x == 0 || y == 0 || x + width >= 27 || y + height >= 27 {
            continue;
        }

        println!("trying {:?}", (x, y, width, height));
        let mut any_overlapping = false;
        for room in &rooms {
            let overlap = x < (room.0) + (room.2)
                && x + width > room.0
                && y + height > room.1
                && y < room.1 + room.3;
            if overlap {
                any_overlapping = true;
                println!("found overlap with {:?}", (x, y, width, height));
                break;
            }
        }

        if !any_overlapping {
            rooms.push((x, y, width, height));
            println!("added room: {:?}", (x, y, width, height));
            println!("computed {} rooms", rooms.len())
        } else {
        }
    }

    for room in rooms {
        for x in (room.0 + 1)..(room.0 + room.2 - 1) {
            for y in (room.1 + 1)..(room.1 + room.3 - 1) {
                if x < 30 && y < 30 {
                    *tiles
                        .get_mut(x as usize)
                        .unwrap()
                        .get_mut(y as usize)
                        .unwrap() = TileType::Floor;
                }
            }
        }
    }

    for row in &tiles {
        println!("row: {:?}", row);
    }
    Map { tiles }
}
