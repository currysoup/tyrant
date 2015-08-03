use tile::Tile;

mod layer;
mod tile;

pub struct World {
    tiles: Vec<Layer>,
    width: u32,
    height: u32,
}
