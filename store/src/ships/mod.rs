pub mod components;
pub mod systems;

use bevy::prelude::*;
// use serde::{Deserialize, Serialize};

use crate::{
    map::components::{CubeCoords, Hexagon},
    PlayerId,
};

pub use components::*;

pub const SHIPS: [ShipType; 2] = [ShipType::Light, ShipType::Light];

fn calc_ship_coords(ship_type: ShipType, pos: Vec2, orientation: u8) -> Vec<CubeCoords> {
    let mut vec = Vec::new();
    match ship_type {
        ShipType::Light => {
            vec.push(CubeCoords {
                q: pos.x as i32,
                r: pos.y as i32,
                s: (pos.x - pos.y) as i32,
            });
        }
        // ShipType::Medium => todo!(),
        // ShipType::Large => todo!(),
    }
    vec
}
