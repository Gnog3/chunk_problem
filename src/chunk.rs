use bevy::{
    prelude::*,
    render::{
        mesh::{Indices, MeshVertexAttribute, PrimitiveTopology},
        render_resource::{Extent3d, TextureDimension, TextureFormat, VertexFormat},
    },
};

use crate::{circuit_states::CircuitStates, material::ChunkMaterial};
#[derive(Component)]
pub struct ComponentChunk {
    states: CircuitStates,
}

#[derive(Bundle)]
pub struct ComponentChunkBundle {
    #[bundle]
    bundle: MaterialMeshBundle<ChunkMaterial>,
    states: ComponentChunk,
}

pub struct ComponentChunkBuilder {
    data1: Vec<[u16; 4]>,
    data2: Vec<[u8; 4]>,
    indices: Vec<u32>,
    states: CircuitStates,
}

enum Vertex {
    Solid {
        pos: Vec3,
        normal: Vec3,
        color: Color,
    },
}

const ATTRIBUTE_0: MeshVertexAttribute =
    MeshVertexAttribute::new("data1_attribute", 5236487123894456, VertexFormat::Uint16x4);

const ATTRIBUTE_1: MeshVertexAttribute =
    MeshVertexAttribute::new("data2_attribute", 6734890293847545, VertexFormat::Uint8x4);

impl ComponentChunkBuilder {
    pub fn new(states: CircuitStates) -> Self {
        Self {
            data1: Vec::new(),
            data2: Vec::new(),
            indices: Vec::new(),
            states,
        }
    }

    fn vertex(&mut self, vertex: Vertex) {
        match vertex {
            Vertex::Solid { pos, normal, color } => {
                let pos = handle_pos(pos);
                let normal = handle_normal(normal);
                let color = handle_color_solid(color);
                let (data1, data2) = construct_vertex_data(pos, normal, color << 2);
                self.data1.push(data1);
                self.data2.push(data2);
            }
        }
    }

    pub fn rectangle(
        &mut self,
        pos: [Vec3; 4],
        color: Color,
        uv: [Vec2; 2],
        normal: Vec3,
    ) -> &mut Self {
        self.vertex(Vertex::Solid {
            pos: pos[0],
            normal,
            color,
        });
        self.vertex(Vertex::Solid {
            pos: pos[1],
            normal,
            color,
        });
        self.vertex(Vertex::Solid {
            pos: pos[2],
            normal,
            color,
        });
        self.vertex(Vertex::Solid {
            pos: pos[3],
            normal,
            color,
        });

        self.indices.extend_from_slice(&[0, 1, 2, 2, 3, 0]);

        self
    }

    pub fn build(
        self,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<ChunkMaterial>,
        images: &mut Assets<Image>,
        position: Vec3,
    ) -> ComponentChunkBundle {
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.insert_attribute(ATTRIBUTE_0, self.data1);
        mesh.insert_attribute(ATTRIBUTE_1, self.data2);
        mesh.set_indices(Some(Indices::U32(self.indices)));
        let (texture_size, texture_side): (usize, u32) = match self.states.vec.len() * 8 {
            0..=256 => (256, 16),
            257..=1024 => (1024, 32),
            1025..=4096 => (4096, 64),
            4097..=16384 => (16384, 128),
            16385..=65536 => (65536, 256),
            len => panic!("Circuit states size exceeded (len = {len})"),
        };

        let mut texture: Vec<u8> = Vec::with_capacity(texture_size * 3);

        for i in 0..texture_size {
            let state = self.states.get(i).unwrap_or_default();

            if state {
                texture.extend_from_slice(&[200, 0, 0, 255]);
            } else {
                texture.extend_from_slice(&[0, 0, 0, 255]);
            }
        }

        let image = Image::new(
            Extent3d {
                width: texture_side,
                height: texture_side,
                depth_or_array_layers: 1,
            },
            TextureDimension::D2,
            texture,
            TextureFormat::Rgba8Unorm,
        );

        let material = ChunkMaterial::default();

        ComponentChunkBundle {
            bundle: MaterialMeshBundle::<ChunkMaterial> {
                mesh: meshes.add(mesh),
                material: materials.add(material),
                transform: Transform::from_translation(position),
                ..default()
            },
            states: ComponentChunk {
                states: self.states,
            },
        }
    }
}

fn handle_pos(pos: Vec3) -> [u16; 3] {
    pos.to_array().map(|x| {
        let a = x * 128.0 + 32768.0;
        if a < 0.0 || a > 65535.0 {
            error!("Vertex position is too small or too big");
        }
        a as u16
    })
}

fn handle_normal(normal: Vec3) -> [u8; 3] {
    normal.to_array().map(|x| {
        let a = x * 127.0 + 128.0;
        if a < 0.0 || a > 255.0 {
            error!("Vertex normal is too small or too big");
        }
        a as u8
    })
}

fn handle_color_solid(color: Color) -> u32 {
    match color.as_rgba() {
        Color::Rgba {
            red,
            green,
            blue,
            alpha,
        } => {
            let red = (red * 127.0).clamp(0.0, 127.0) as u32;
            let green = (green * 255.0).clamp(0.0, 255.0) as u32;
            let blue = (blue * 127.0).clamp(0.0, 127.0) as u32;
            (red << 15) | (green << 7) | blue
        }
        _ => unreachable!(),
    }
}

fn construct_vertex_data(pos: [u16; 3], normal: [u8; 3], data: u32) -> ([u16; 4], [u8; 4]) {
    if data >= (1 << 24) {
        error!("Vertex data is too big!");
    }
    let data1 = (data >> 8) as u16;
    let data2 = (data & 0xFF) as u8;
    let data1 = [pos[0], pos[1], pos[2], data1];
    let data2 = [normal[0], normal[1], normal[2], data2];
    (data1, data2)
}
