use super::{MapSize, MapTile};
use mk::mlua::prelude::*;

pub struct T(Option<MapTile>);

// #[derive(Debug, Clone, Hash)]
// pub struct Map {
//     size: MapSize,
//     tiles: Vec<Option<MapTile>>,
// }

// impl Map {
//     pub fn new(size: MapSize, tiles: Vec<Option<MapTile>>) -> Self {
//         Self { size, tiles }
//     }

//     pub fn size(&self) -> MapSize {
//         self.size
//     }

//     pub fn tiles(&self) -> &[Option<MapTile>] {
//         &self.tiles
//     }
// }

// impl LuaUserData for Map {
//     fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
//         fields.add_field_method_get("size", |_lua, this| Ok(this.size));
//     }
// }
