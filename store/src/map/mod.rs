use bevy::prelude::*;
use bevy::render::{mesh::Indices, render_resource::PrimitiveTopology};

#[derive(Clone, Debug)]
pub struct Hexagon {
    pub size: f32,
    pub height: f32,
    pub width: f32,
    pub coordinates: Option<[u32; 3]>,
    // pub neighbors: Option<[&Hexagon; 6]>,
}

impl Hexagon {
    /// Create a new Hexagon struct
    pub fn new(size: f32) -> Self {
        Hexagon {
            size,
            height: 3.0_f32.sqrt() * size,
            width: 2.0 * size,
            coordinates: None,
            // neighbors: None,
        }
    }

    /// Return the Vec2 coordinate of point i in a Hexagon
    fn hex_corner_pos(&self, i: usize) -> Vec2 {
        let angle = 60.0_f32.to_radians() * i as f32;
        return Vec2 {
            x: self.size * angle.cos(),
            y: self.size * angle.sin(),
        };
    }

    /// Generate a ['MaterialMeshBundle'] based on Hexagon coordinates and size.
    pub fn to_mesh(&self) -> Mesh {
        let mut vectors = Vec::with_capacity(8);
        vectors.push([0.0, 0.0, 0.0]);
        let mut indices = Vec::new();
        for i in 0..6 {
            let vec2d_pos = self.hex_corner_pos(i);
            trace!("{:?}", vec2d_pos);
            vectors.push([vec2d_pos.x, vec2d_pos.y, 0.0]);
            indices.push(0);
            indices.push(i as u32 + 1);
            if i < 5 {
                indices.push(i as u32 + 2);
            } else {
                indices.push(1);
            }
        }
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vectors);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0.0, 0.0, 1.0]; 7]);
        mesh.set_indices(Some(Indices::U32(indices)));
        mesh
    }

    // TODO: change to Axial Coordinate System
    pub fn world_pos(&self) -> Vec3 {
        let pos = Vec3::new(0.0, 0.0, 0.0);
        let coordinates = self
            .coordinates
            .expect("Cannot return Vec3 for a hex without a coordinate");
        let y_offset = (coordinates[0] % 2) as f32 * self.height * 0.5;
        let x_offset = 0.75 * self.width;
        Vec3::new(
            pos.x + coordinates[0] as f32 * x_offset,
            pos.y + self.height * coordinates[1] as f32 + y_offset,
            0.0,
        )
    }
}

#[derive(Component)]
pub struct HexMap {
    pub hex_size: f32,
    pub hexes: Vec<Hexagon>,
}

impl HexMap {
    pub fn new(self, size: f32, coordinate_sys: CoordinateSystem) -> Self {
        // TODO: change to Axial Coordinate System
        let hexes = match coordinate_sys {
            CoordinateSystem::Offset(offset_type) => hexes_from_offset(offset_type, size),
            CoordinateSystem::Axial(radius) => hexes_from_axial(radius, size),
        };
        HexMap {
            hex_size: size,
            hexes,
        }
    }

    // pub fn get_hex_from_pos(pos: Vec3) -> &Hexagon {}
    pub fn coordinate_from_pos(pos: Vec2) -> [u32; 3] {
        [0, 0, 0]
    }
}

fn hexes_from_offset(offset_type: OffsetType, size: f32) -> Vec<Hexagon> {
    let mut hex = Hexagon::new(size);
    let mut hexes = Vec::new();
    match offset_type {
        OffsetType::EvenQ(width, height) => {
            for i in 0..height {
                for j in 0..width {
                    hex.coordinates = Some([j, i, 0]);
                    hex.coordinates = Some([j, i, 0]);
                    hexes.push(hex.clone());
                }
            }
        }
    };
    hexes
}

fn hexes_from_axial(radius: u32, size: f32) -> Vec<Hexagon> {
    let mut hex = Hexagon::new(size);
    let mut hexes = Vec::new();
}

fn offset_to_axial_coords(x: u32, y: u32) -> [u32; 2] {}
fn axial_to_offset_coords(x: u32, y: u32) -> [u32; 2] {}

pub enum CoordinateSystem {
    // Square maps. u32, u32 sets width and heigh respectively
    Offset(OffsetType),
    // Round maps. u32 the radius (number of 'rings' around center)
    Axial(u32),
}

pub enum OffsetType {
    // vertical layout. shoves odd columns up
    EvenQ(u32, u32),
}
