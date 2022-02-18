use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub fn setup_level(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let scale = 10.0;

    // plane
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(bevy::prelude::shape::Plane { size: scale })),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(scale / 2.0, 0.1, scale / 2.0).into(),
            ..Default::default()
        });

    // walls
    let wall = meshes.add(Mesh::from(bevy::prelude::shape::Box::new(scale, 4.0, 1.0)));
    let material = materials.add(Color::rgb(0.3, 0.5, 0.3).into());
    // top_wall
    let position = Vec3::new(0.0, 0.0, -scale / 2.0);
    commands
        .spawn_bundle(PbrBundle {
            mesh: wall.clone(),
            material: material.clone(),
            transform: Transform::from_translation(position),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(scale / 2.0, 2.0, 0.5).into(),
            position: position.into(),
            ..Default::default()
        });

    // bottom wall
    let position = Vec3::new(0.0, 0.0, scale / 2.0);
    commands
        .spawn_bundle(PbrBundle {
            mesh: wall.clone(),
            material: material.clone(),
            transform: Transform::from_translation(position),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(scale / 2.0, 2.0, 0.5).into(),
            position: position.into(),
            ..Default::default()
        });

    // right wall
    let position = Vec3::new(scale / 2.0, 0.0, 0.0);
    commands
        .spawn_bundle(PbrBundle {
            mesh: wall.clone(),
            material: material.clone(),
            transform: Transform {
                translation: position,
                rotation: Quat::from_rotation_y(90.0_f32.to_radians()),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(0.5, 2.0, scale / 2.0).into(),
            position: position.into(),
            ..Default::default()
        });

    // left wall
    let position = Vec3::new(-scale / 2.0, 0.0, 0.0);
    commands
        .spawn_bundle(PbrBundle {
            mesh: wall.clone(),
            material: material.clone(),
            transform: Transform {
                translation: position,
                rotation: Quat::from_rotation_y(90.0_f32.to_radians()),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(0.5, 2.0, scale / 2.0).into(),
            position: position.into(),
            ..Default::default()
        });

    // sensor
    // let sensor = meshes.add(Mesh::from(bevy::prelude::shape::Box::new(
    //     scale / 8.0,
    //     1.0,
    //     scale / 8.0,
    // )));
    // let sensor_material = materials.add(StandardMaterial {
    //     base_color: Color::rgba(0.4, 0.0, 0.7, 0.2),
    //     alpha_mode: AlphaMode::Blend,
    //     ..Default::default()
    // });
    // let position = Vec3::new(-scale / 4.0, 0.3, -scale / 4.0);
    // commands
    //     .spawn_bundle(PbrBundle {
    //         mesh: sensor,
    //         material: sensor_material,
    //         transform: Transform {
    //             translation: position,
    //             ..Default::default()
    //         },
    //         ..Default::default()
    //     })
    //     .insert_bundle(ColliderBundle {
    //         collider_type: ColliderTypeComponent(ColliderType::Sensor),
    //         shape: ColliderShape::cuboid(scale / 16.0, 0.5, scale / 16.0).into(),
    //         position: position.into(),
    //         ..Default::default()
    //     })
    //     .insert(RigidBodyPositionSync::Discrete);

    // // platform
    // let platform = meshes.add(Mesh::from(bevy::prelude::shape::Box::new(
    //     scale / 4.0,
    //     0.2,
    //     scale / 4.0,
    // )));
    // let platform_material = materials.add(StandardMaterial {
    //     base_color: Color::rgb(0.4, 0.0, 0.9),
    //     ..Default::default()
    // });
    // let position = Vec3::new(-scale / 4.0, 0.2, -scale / 4.0);
    // commands
    //     .spawn_bundle(PbrBundle {
    //         mesh: platform,
    //         material: platform_material,
    //         transform: Transform {
    //             translation: position,
    //             ..Default::default()
    //         },
    //         ..Default::default()
    //     })
    //     .insert_bundle(RigidBodyBundle {
    //         body_type: RigidBodyTypeComponent(RigidBodyType::KinematicPositionBased),
    //         position: position.into(),
    //         ..Default::default()
    //     })
    //     .insert_bundle(ColliderBundle {
    //         shape: ColliderShape::cuboid(scale / 8.0, 0.1, scale / 8.0).into(),
    //         ..Default::default()
    //     })
    //     .insert(RigidBodyPositionSync::Discrete);
}
