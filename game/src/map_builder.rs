use std::borrow::Borrow;

use crate::map::Map;
use crate::map::TileType;
use rand::Rng;
use sdl2::sys::SDL_YUV_CONVERSION_MODE;

/// Generates a map following this procedure:
///
/// - Create four non-overlapping rooms.
/// -
///
///
/// Drawing a path from rectangle A to B:
///
///
/// If A.left > b.left and a.right < b.right:
///   if a.top < b.bottom:
///     start at top
///   else:
///     start at bottom
///
/// else:
///   if a.right < b.right:
///     start on right
///   else:
///     start on left
///
///
///
///
/// If A.right < b.right:
/// _________     _______
/// |       |     |     |
/// |   A   |     |  B  |
/// |_______|     |_____|
///
///   > start path on left side of B
///
///
///               _______
///               |     |
///               |  B  |
///               |_____|
///               
///                |----|
///                | A  |
///                |----|
///
/// If
///
///
///

#[derive(Debug)]
struct Path {
    start_pos: (i32, i32),
    x_comp: i32,
    y_comp: i32,
}

#[derive(Debug)]
struct Room {
    left: i32,
    right: i32,
    top: i32,
    bottom: i32,
}

impl Room {
    pub fn overlaps(&self, other: &Room) -> bool {
        self.left < other.right
            && self.right > other.left
            && self.top > other.bottom
            && self.bottom < other.top
    }

    pub fn width(&self) -> i32 {
        self.right - self.left
    }

    pub fn height(&self) -> i32 {
        self.top - self.bottom
    }

    pub fn center(&self) -> (i32, i32) {
        (
            self.left + self.width() / 2,
            self.bottom + self.height() / 2,
        )
    }
}

fn gen_path(room_a: &Room, room_b: &Room) -> Path {
    let x_comp = room_b.center().0 - room_a.center().0;
    let y_comp = room_b.center().1 - room_a.center().1;

    Path {
        start_pos: room_a.center(),
        x_comp,
        y_comp,
    }
}

fn gen_paths(rooms: &[Room]) -> Vec<Path> {
    let mut paths = Vec::new();
    for window2 in rooms.windows(2) {
        paths.push(gen_path(window2.get(0).unwrap(), window2.get(1).unwrap()));
    }
    paths
}

fn gen_rooms(num_rooms: usize) -> Vec<Room> {
    let mut rooms = Vec::new();
    while rooms.len() < num_rooms {
        let left: i32 = rand::thread_rng().gen_range(1..40);
        let bottom: i32 = rand::thread_rng().gen_range(1..40);

        let width = rand::thread_rng().gen_range(8..12);
        let height = rand::thread_rng().gen_range(8..12);

        if left + width >= 37 || bottom + height >= 37 {
            continue;
        }

        let new_room = Room {
            left,
            bottom,
            top: bottom + height,
            right: left + width,
        };

        let mut any_overlapping = false;
        for room in &rooms {
            if new_room.overlaps(room) {
                any_overlapping = true;
                break;
            }
        }

        if !any_overlapping {
            rooms.push(new_room)
        }
    }
    rooms
}

pub fn create_map() -> Map {
    const NUM_ROOMS: usize = 6;
    let rooms = gen_rooms(NUM_ROOMS);
    let paths = gen_paths(&rooms);

    let mut tiles = vec![vec![TileType::Wall; 40]; 40];
    for room in rooms {
        for x in (room.left + 1)..(room.right - 1) {
            for y in (room.bottom + 1)..(room.top - 1) {
                if x < 40 && y < 40 {
                    *tiles
                        .get_mut(x as usize)
                        .unwrap()
                        .get_mut(y as usize)
                        .unwrap() = TileType::Floor;
                }
            }
        }
    }

    println!("rooms and paths generated");

    for path in paths {
        for x in (path.start_pos.0)..(path.start_pos.0 + path.x_comp) {
            if x < 40 {
                *tiles
                    .get_mut(x as usize)
                    .unwrap()
                    .get_mut(path.start_pos.1 as usize)
                    .unwrap() = TileType::Floor;

                *tiles
                    .get_mut(x as usize)
                    .unwrap()
                    .get_mut((path.start_pos.1 + 1) as usize)
                    .unwrap() = TileType::Floor;
            }
        }

        for y in (path.start_pos.1)..(path.start_pos.1 + path.y_comp) {
            if y < 40 {
                *tiles
                    .get_mut(path.start_pos.0 as usize)
                    .unwrap()
                    .get_mut(y as usize)
                    .unwrap() = TileType::Floor;

                *tiles
                    .get_mut((path.start_pos.0 + 1) as usize)
                    .unwrap()
                    .get_mut(y as usize)
                    .unwrap() = TileType::Floor;
            }
        }
    }

    Map { tiles }
}
