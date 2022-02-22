use bevy::prelude::*;
// use bevy_rapier3d::prelude::*;

use crate::animated_shader;

pub fn setup_level(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    commands.spawn().insert_bundle((
        meshes.add(Mesh::from(bevy::prelude::shape::Icosphere {
            radius: 0.5,
            subdivisions: 5,
        })),
        Transform::from_xyz(1.0, 2.0, 1.0),
        GlobalTransform::default(),
        animated_shader::CustomMaterial,
        Visibility::default(),
        ComputedVisibility::default(),
    ));
}
