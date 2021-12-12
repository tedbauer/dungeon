#[derive(Clone, PartialEq, Debug)]
pub enum TileType {
    Wall,
    Floor,
}

#[derive(Debug)]
pub struct Map {
    pub tiles: Vec<Vec<TileType>>,
}
