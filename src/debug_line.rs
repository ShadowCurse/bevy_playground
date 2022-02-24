use bevy::prelude::*;
use bevy::render::{mesh::Mesh, render_resource::PrimitiveTopology};

#[derive(Component)]
pub struct DebugLineMarker;

#[derive(Debug, Default)]
pub struct DebugLine {
    pub start: Vec3,
    pub end: Vec3,
}

impl From<DebugLine> for Mesh {
    fn from(line: DebugLine) -> Self {
        let mut mesh = Mesh::new(PrimitiveTopology::LineStrip);

        let vertices: Vec<[f32; 3]> = vec![line.start.into(), line.end.into()];

        mesh.set_attribute(
            Mesh::ATTRIBUTE_NORMAL,
            vec![[0.0, 0.0, 1.0]; vertices.len()],
        );
        mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, vec![[0.0, 0.0]; vertices.len()]);
        mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
        mesh.set_indices(None);
        mesh
    }
}
