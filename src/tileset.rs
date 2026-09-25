use crate::TILESET;
use macroquad::file::load_file;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Tileset {
    pub tiles: Vec<Tile>,
}

#[derive(Deserialize, Debug)]
pub struct Tile {
    pub id: i32,
    pub image: String,
}

pub async fn load_tileset() {
    let tileset_bytes = load_file("assets/tileset.json").await.unwrap();

    let tileset: Tileset = serde_json::from_slice(&tileset_bytes).unwrap();

    TILESET.set(tileset.tiles).unwrap();
}
