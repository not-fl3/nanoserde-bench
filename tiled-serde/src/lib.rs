use serde::{Deserialize, Serialize};

pub use serde_json::from_str;

mod layer;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Orientation {
    #[serde(rename = "orthogonal")]
    Orthogonal,
    #[serde(rename = "isometric")]
    Isometric,
}
impl Default for Orientation {
    fn default() -> Orientation {
        Orientation::Orthogonal
    }
}

/// https://doc.mapeditor.org/en/stable/reference/tmx-map-format/#tmx-grid
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Grid {
    pub orientation: Orientation,
    pub width: i32,
    pub height: i32,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Property {
    pub name: String,
    pub value: String,
    pub ty: String,
}

/// https://doc.mapeditor.org/en/stable/reference/json-map-format/#json-layer
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Layer {}

/// https://doc.mapeditor.org/en/stable/reference/json-map-format/#json-frame
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Frame {
    duration: i32,
    tileid: i32,
}

/// https://doc.mapeditor.org/en/stable/reference/json-map-format/#json-tile
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Tile {
    /// Array of Frames
    animation: Vec<Frame>,
    /// Local ID of the tile
    id: usize,
    /// Image representing this tile (optional)
    image: Option<String>,
    /// Height of the tile image in pixels
    imagewidth: i32,
    /// Width of the tile image in pixels
    imageheight: i32,
    /// Layer with type objectgroup (optional)
    objectgroup: Option<Layer>,
    /// A list of properties (name, value, type)
    properties: Vec<Property>,
    /// Index of terrain for each corner of tile
    terrain: Vec<i32>,
    /// The type of the tile (optional)
    #[serde(rename = "type")]
    ty: Option<String>,
}

/// https://doc.mapeditor.org/en/stable/reference/tmx-map-format/#tmx-tileoffset
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Tileoffset {
    pub x: i32,
    pub y: i32,
}

/// https://doc.mapeditor.org/en/stable/reference/json-map-format/#json-terrain
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Terrain {
    pub name: String,
    pub tile: i32,
}

/// https://doc.mapeditor.org/en/stable/reference/json-map-format/#tileset
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Tileset {
    /// The number of tile columns in the tileset
    pub columns: i32,
    /// GID corresponding to the first tile in the set
    pub firstgid: Option<i32>,
    /// See <grid> (optional)
    pub grid: Option<Grid>,
    /// Image used for tiles in this set
    pub image: Option<String>,
    /// Width of source image in pixels
    pub imagewidth: i32,
    /// Height of source image in pixels
    pub imageheight: i32,
    /// Buffer between image edge and first tile (pixels)
    pub margin: i32,
    /// Name given to this tileset
    pub name: String,
    /// A list of properties (name, value, type).
    pub properties: Vec<Property>,
    /// Spacing between adjacent tiles in image (pixels)
    pub spacing: i32,
    /// Array of Terrains (optional)
    pub terrains: Option<Vec<Terrain>>,
    /// The number of tiles in this tileset
    pub tilecount: i32,
    /// Maximum height of tiles in this set
    pub tileheight: i32,
    /// See <tileoffset> (optional)
    pub tileoffset: Option<Tileoffset>,
    /// Array of Tiles (optional)
    pub tiles: Option<Vec<Tile>>,
    /// Maximum width of tiles in this set
    pub tilewidth: i32,
    /// Hex-formatted color (#RRGGBB) (optional)
    pub transparentcolor: Option<String>,
}

/// https://doc.mapeditor.org/en/stable/reference/json-map-format/#map
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Map {
    /// Hex-formatted color (#RRGGBB or #AARRGGBB) (optional)
    pub backgroundcolor: String,
    /// Number of tile rows
    pub height: u32,

    pub properties: Vec<Property>,

    pub orientation: String,
    pub renderorder: String,

    pub tileheight: u32,
    pub tilewidth: u32,

    pub layers: Vec<layer::Layer>,
    pub tilesets: Vec<Tileset>,

    /// "map" (since 1.0)
    #[serde(rename = "type")]
    pub ty: String,

    /// The JSON format version
    pub version: f32,
    /// Number of tile columns
    pub width: u32,
}
