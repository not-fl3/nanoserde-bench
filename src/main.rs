#[cfg(feature = "tiled-nanoserde")]
fn main() {
    use tiled_nanoserde::{DeJson, Map};

    let tiled_map = include_str!("../assets/tiled_map.json");
    let tiled_map: Map = DeJson::deserialize_json(&tiled_map).unwrap();

    assert_eq!(tiled_map.ty, "map");
    assert_eq!(tiled_map.layers.len(), 1);

    #[cfg(feature = "nano-bin")] {
        use nanoserde::{DeBin, SerBin};

        #[derive(SerBin, DeBin)]
        struct Message {
            kind: String,
            data: i32
        }

        let msg = Message {
            kind: "Some message".to_string(),
            data: 23
        };
        let bytes = SerBin::serialize_bin(&msg);
        assert!(bytes.len() != 0);
    }
}

#[cfg(feature = "tiled-serde")]
fn main() {
    use tiled_serde::{from_str, Map};

    let tiled_map = include_str!("../assets/tiled_map.json");
    let tiled_map: Map = from_str(&tiled_map).unwrap();

    assert_eq!(tiled_map.ty, "map");
    assert_eq!(tiled_map.layers.len(), 1);

    #[cfg(feature = "serde-bin")] {
        use serde::{Serialize, Deserialize};

        #[derive(Serialize, Deserialize)]
        struct Message {
            kind: String,
            data: i32
        }

        let msg = Message {
            kind: "Some message".to_string(),
            data: 23
        };
        let bytes = bincode::serialize(&msg).unwrap();
        assert!(bytes.len() != 0);
    }
}
