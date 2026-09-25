use crate::{TILESET, objects::*, player::Player};
use macroquad::file::load_file;
use macroquad::math::{Rect, Vec2};
use serde::Deserialize;

// TODO: scale map to fit screen size
pub struct Map {
    // multiply by tile_width
    pub width: u32,
    // multiply by tile_height
    pub height: u32,
    pub objects: Vec<MapObject>,
}

impl Map {
    pub async fn place_objects(&self, objects: &mut Vec<Box<dyn Object>>) -> Player {
        let mut player = None;

        for object in &self.objects {
            let x = object.x - object.width / 2.0;
            let y = object.y - object.height / 2.0;
            let rect = Rect::new(x, y, object.width, object.height);

            match object._type {
                Type::Player => player = Some(Player::new(x, y).await),
                Type::Platform => objects.push(Box::new(Platform::new(rect).await)),
                Type::Trampoline => objects.push(Box::new(Trampoline::new(rect).await)),
                Type::DestroyingPlatform => {
                    objects.push(Box::new(DestroyingPlatform::new(rect).await))
                }
                Type::MovingPlatform => {
                    let start = if let Some(min_x) = object.get_property("min_x") {
                        Vec2::new(min_x.parse().unwrap(), object.y)
                    } else if let Some(min_y) = object.get_property("min_y") {
                        Vec2::new(object.x, min_y.parse().unwrap())
                    } else {
                        panic!("Either min_x or min_y should be set for a moving platform")
                    };

                    let end = if let Some(max_x) = object.get_property("max_x") {
                        Vec2::new(max_x.parse().unwrap(), object.x)
                    } else if let Some(max_y) = object.get_property("max_y") {
                        Vec2::new(object.x, max_y.parse().unwrap())
                    } else {
                        panic!("Either max_x or max_y should be set for a moving platform")
                    };

                    objects.push(Box::new(
                        MovingPlatform::new(
                            rect,
                            start,
                            end,
                            object
                                .get_property("speed")
                                .expect("Speed should be set for a moving platform")
                                .parse()
                                .unwrap(),
                        )
                        .await,
                    ))
                }
                Type::ExtraDash => objects.push(Box::new(ExtraDash::new(rect).await)),
                Type::Spikes => objects.push(Box::new(Spikes::new(rect).await)),
                Type::LevelTrigger => objects.push(Box::new(LevelTrigger::new(rect).await)),
                Type::Text => objects.push(Box::new(
                    Text::new(
                        Rect::new(object.x, object.y, 0., 0.),
                        &object.text.as_ref().unwrap().text,
                        object.text.as_ref().unwrap().pixelsize,
                    )
                    .await,
                )),
            }
        }

        player.expect("Player should exist in map")
    }
}

#[derive(Deserialize, Clone)]
struct Tileset {
    pub firstgid: i32,
}

pub struct MapObject {
    pub _type: Type,
    pub width: f32,
    pub height: f32,
    pub x: f32,
    pub y: f32,
    pub properties: Option<Vec<Property>>,
    pub text: Option<MapText>,
}

impl MapObject {
    fn get_property(&self, name: &str) -> Option<String> {
        if let Some(ref properties) = self.properties {
            Some(properties.iter().find(|p| p.name == name)?.value.clone())
        } else {
            None
        }
    }
}

pub enum Type {
    Player,
    Platform,
    Trampoline,
    DestroyingPlatform,
    MovingPlatform,
    ExtraDash,
    Spikes,
    LevelTrigger,
    Text,
}

#[derive(Deserialize, Clone)]
pub struct MapText {
    pub text: String,
    pub pixelsize: u16,
}

#[derive(Deserialize, Clone)]
pub struct Property {
    pub name: String,
    pub value: String,
}

#[derive(Deserialize)]
struct RawMap {
    width: u32,
    height: u32,
    tilewidth: u32,
    tileheight: u32,
    tilesets: Vec<Tileset>,
    layers: Vec<Layer>,
}

impl From<RawMap> for Map {
    fn from(raw_map: RawMap) -> Self {
        let first_gid = raw_map.tilesets[0].firstgid;
        Map {
            width: raw_map.width * raw_map.tileheight,
            height: raw_map.height * raw_map.tilewidth,
            objects: raw_map.layers[0]
                .objects
                .iter()
                .map(|r| r.clone().into_map_object(first_gid))
                .collect(),
        }
    }
}

#[derive(Deserialize)]
struct Layer {
    objects: Vec<RawObject>,
}

#[derive(Deserialize, Clone)]
struct RawObject {
    gid: Option<i32>,
    width: f32,
    height: f32,
    x: f32,
    y: f32,
    properties: Option<Vec<Property>>,
    text: Option<MapText>,
}

impl RawObject {
    fn into_map_object(self, first_gid: i32) -> MapObject {
        let tileset = TILESET.get().unwrap();

        let _type = if let Some(gid) = self.gid {
            let gid = gid - first_gid;

            let tile = tileset.iter().find(|t| t.id == gid).unwrap();

            match tile.image.as_str() {
                "sprites/player/player.png" => Type::Player,
                "sprites/platform/platform.png" => Type::Platform,
                "sprites/trampoline/trampoline.png" => Type::Trampoline,
                "sprites/destroyingplatform/destroying_platform_0.png" => Type::DestroyingPlatform,
                "sprites/moving_platform/moving_platform.png" => Type::MovingPlatform,
                "sprites/extradash/extradash.png" => Type::ExtraDash,
                "sprites/spikes/spikes.png" => Type::Spikes,
                "sprites/leveltrigger/leveltrigger.png" => Type::LevelTrigger,
                _ => panic!("Unknown type"),
            }
        } else {
            Type::Text
        };

        MapObject {
            _type,
            width: self.width,
            height: self.height,
            x: self.x,
            y: self.y,
            properties: self.properties,
            text: self.text,
        }
    }
}

pub async fn load_map(name: &str) -> Map {
    let path = format!("assets/maps/{}.json", name);
    let map_bytes = load_file(&path).await.unwrap();

    let raw_map: RawMap = serde_json::from_slice(&map_bytes).unwrap();

    raw_map.into()
}
