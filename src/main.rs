#[cfg(feature = "tiled-nanoserde")]
fn main() {
    use tiled_nanoserde::{DeJson, Map};

    let tiled_map = include_str!("../assets/tiled_map.json");
    let tiled_map: Map = DeJson::deserialize_json(&tiled_map).unwrap();

    assert_eq!(tiled_map.ty, "map");
    assert_eq!(tiled_map.layers.len(), 1);
}

#[cfg(feature = "tiled-serde")]
fn main() {
    use tiled_serde::{from_str, Map};

    let tiled_map = include_str!("../assets/tiled_map.json");
    let tiled_map: Map = from_str(&tiled_map).unwrap();

    assert_eq!(tiled_map.ty, "map");
    assert_eq!(tiled_map.layers.len(), 1);
}
