use bevy::{
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
};
use serde::{Deserialize, Serialize};

use crate::{
    map::{
        components::{CubeCoords, Hexagon},
        HEX_CONFIG_PADDING, HEX_CONFIG_SIZE,
    },
    PlayerId,
};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum ShipType {
    Light,
    // Medium,
    // Large,
}

#[derive(Clone, Debug, Component)]
pub struct Ship {
    pub hexes: Vec<Hexagon>,
    pub hex_end: Hexagon,
    pub ship_type: ShipType,
    pub player_id: PlayerId,
}

impl Ship {
    /// Create a new Hexagon struct
    pub fn new(
        ship_type: ShipType,
        player_id: PlayerId,
        coords: CubeCoords,
        coords_end: CubeCoords,
    ) -> Self {
        let mut hexes = Vec::new();
        let hex_layer = 2.0;
        let hex = Hexagon::new(HEX_CONFIG_SIZE, HEX_CONFIG_PADDING, Some(coords), hex_layer);
        let hex_end = Hexagon::new(
            HEX_CONFIG_SIZE,
            HEX_CONFIG_PADDING,
            Some(coords_end),
            hex_layer,
        );
        hexes.push(hex);
        match ship_type {
            ShipType::Light => hexes.push(Hexagon::new(
                HEX_CONFIG_SIZE,
                HEX_CONFIG_PADDING,
                Some(coords - CubeCoords::Q),
                hex_layer,
            )),
        }
        Ship {
            hexes,
            hex_end,
            ship_type,
            player_id,
        }
    }

    /// Generate a ['MaterialMeshBundle'] based on Hexagon coordinates and ship type.
    pub fn to_mesh(&self) -> Mesh {
        let hex_top = &self.hexes[0];
        let triangle_top = [0.0, hex_top.height * 0.25, 0.0];
        let upper_left = [-hex_top.width / 4.0, 0.0, 0.0];
        let upper_right = [hex_top.width / 4.0, 0.0, 0.0];
        let mut vectors = Vec::new();
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        match &self.ship_type {
            ShipType::Light => {
                let bottom_left = [-hex_top.width / 4.0, -hex_top.height * 1.2, 0.0];
                let bottom_right = [hex_top.width / 4.0, -hex_top.height * 1.2, 0.0];
                vectors.push(upper_left);
                vectors.push(bottom_left);
                vectors.push(bottom_right);
                vectors.push(upper_right);
                vectors.push(triangle_top);
                mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vectors);
                mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0.0, 0.0, 1.0]; 5]);
                mesh.set_indices(Some(Indices::U32(vec![0, 1, 3, 1, 2, 3, 0, 3, 4])));
            } // ShipType::Medium => {}
              // ShipType::Large => {}
        }
        mesh
    }

    // pub fn hover_mesh(&self) -> Mesh {
    //     match self.ship_type {
    //         ShipType::Light => todo!(),
    //     }
    // }

    pub fn world_pos(&self) -> Vec3 {
        let hex_top = &self.hexes[0];
        let coords = hex_top
            .coords
            .as_ref()
            .expect("Cannot return Vec3 for a hex without a coordinate");

        // this is for axial coordinates
        let y_offset = hex_top.height * (coords.s as f32 + 0.5 * coords.q as f32);
        let x_offset = 0.75 * hex_top.width * coords.q as f32;
        Vec3::new(x_offset, y_offset, hex_top.layer)
    }

    pub fn rotate_right(&mut self, center: &CubeCoords) {
        for hex in self.hexes.iter_mut() {
            let mut vec = hex.coords.unwrap() - *center;
            let len = vec.magnitude();
            vec.rotate_right();
            hex.coords = Some(vec + *center);
        }
    }
    pub fn rotate_left(&mut self, center: &CubeCoords) {
        for hex in self.hexes.iter_mut() {
            let mut vec = hex.coords.unwrap() - *center;
            vec.rotate_left();
            hex.coords = Some(vec + *center);
        }
    }
}
