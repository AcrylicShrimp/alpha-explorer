use crate::asset::{AssetLoadError, AssetLoader};
use crate::render::*;
use serde::{Deserialize, Serialize};
use serde_json::Error as JSONError;
use std::fs::read_to_string;

#[derive(Serialize, Deserialize)]
struct TilemapJSON {
    width: u32,
    height: u32,
    tilewidth: u32,
    tileheight: u32,
    layers: Vec<TilemapLayerJSON>,
    tilesets: Vec<TilemapTilesetJSON>,
}

#[derive(Serialize, Deserialize)]
struct TilemapLayerJSON {
    data: Vec<usize>,
}

#[derive(Serialize, Deserialize)]
struct TilemapTilesetJSON {
    firstgid: usize,
    source: String,
}

impl From<JSONError> for AssetLoadError {
    fn from(err: JSONError) -> Self {
        Self::other(err)
    }
}

pub fn tilemap_loader() -> AssetLoader<Tilemap> {
    AssetLoader::new(|asset_mgr, base, path| {
        let tilemap_json: TilemapJSON = serde_json::from_str(&read_to_string(
            &base.join("maps").join(path).with_extension("json"),
        )?)?;

        if tilemap_json.tilesets.len() != 1 {
            return Err(AssetLoadError::from(
                "tilemap must use a single tileset only",
            ));
        }

        if tilemap_json.tilesets[0].firstgid != 1 {
            return Err(AssetLoadError::from("tileset's firstgid must be 1"));
        }

        let palette = asset_mgr.load(&tilemap_json.tilesets[0].source)?;

        Ok(Tilemap {
            tile_width: tilemap_json.tilewidth as f32,
            tile_height: tilemap_json.tileheight as f32,
            tile_count_x: tilemap_json.width as usize,
            tile_count_y: tilemap_json.height as usize,
            layers: tilemap_json
                .layers
                .into_iter()
                .map(|layer| layer.data)
                .collect(),
            palette,
        }
        .into())
    })
}
