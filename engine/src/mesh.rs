use std::path::Path;
use glium::implement_vertex;
use crate::context::GlBackend;

#[derive(Copy, Clone)]
pub struct Vertex {
    pub id: u32,
    pub color: (f32, f32, f32, f32),
    pub data: (f32, f32, f32, f32),
    pub position: (f32, f32, f32),
    pub tex_coords: (f32, f32),
}

#[derive(Copy, Clone)]
pub struct Normal {
    pub normal: (f32, f32, f32),
}

implement_vertex!(Vertex, id, data, color, position, tex_coords);
implement_vertex!(Normal, normal);

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub normals:  Option<Vec<Normal>>,
    pub indices: Vec<u32>,
    pub primitive: glium::index::PrimitiveType,
}

impl Mesh {
    pub fn new(_backend: &GlBackend, path: &str) -> Mesh {
        let obj = tobj::load_obj(&Path::new(&format!("./res/meshes/{}.obj", path)));

        assert!(obj.is_ok());

        let mut vertices: Vec<Vertex> = Vec::new();
        let mut normals: Vec<Normal> = Vec::new();
        let mut indices: Vec<u32> = Vec::new();

        let (models, materials) = obj.unwrap();

        for (_i, m) in models.iter().enumerate() {
        	let mesh = &m.mesh;
        	for f in 0..mesh.indices.len() / 3 {
        		indices.push(mesh.indices[3 * f]);
        		indices.push(mesh.indices[3 * f + 1]);
        		indices.push(mesh.indices[3 * f + 2]);
        	}

            assert!(!mesh.normals.is_empty());
            for n in 0..mesh.normals.len() / 3 {
                normals.push(Normal {
                    normal: (mesh.normals[3 * n], mesh.normals[3 * n + 1], mesh.normals[3 * n + 2])
                })
            }

        	assert!(mesh.positions.len() % 3 == 0);
        	for v in 0..mesh.positions.len() / 3 {
                vertices.push(Vertex {
                    id: 0,
                    color: (0.0, 0.0, 0.0, 0.0),
                    data: (0.0, 0.0, 0.0, 0.0),
                    position: (mesh.positions[3 * v], mesh.positions[3 * v + 1], mesh.positions[3 * v + 2]),
                    tex_coords: (0.0, 0.0),
                });
        	}
        }
        for (i, m) in materials.iter().enumerate() {
        	println!("material[{}].name = \'{}\'", i, m.name);
        	println!("    material.Ka = ({}, {}, {})", m.ambient[0], m.ambient[1],
        		m.ambient[2]);
        	println!("    material.Kd = ({}, {}, {})", m.diffuse[0], m.diffuse[1],
        		m.diffuse[2]);
        	println!("    material.Ks = ({}, {}, {})", m.specular[0], m.specular[1],
        		m.specular[2]);
        	println!("    material.Ns = {}", m.shininess);
        	println!("    material.d = {}", m.dissolve);
        	println!("    material.map_Ka = {}", m.ambient_texture);
        	println!("    material.map_Kd = {}", m.diffuse_texture);
        	println!("    material.map_Ks = {}", m.specular_texture);
        	println!("    material.map_Ns = {}", m.normal_texture);
        	println!("    material.map_d = {}", m.dissolve_texture);
        	for (k, v) in &m.unknown_param {
        		println!("    material.{} = {}", k, v);
        	}
        }

        Mesh {
            vertices,
            normals: Some(normals),
            indices,
            primitive: glium::index::PrimitiveType::TrianglesList,
        }
    }
}
