use bevy::prelude::*;
use bevy_mod_picking::{PickableBundle, PickingCameraBundle};
use bevy_rapier3d::prelude::*;

use crate::debug_line;
use crate::editor_enhanced;
use crate::follower;

#[derive(Component)]
pub struct Player;

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.insert_resource(follower::FollowerController {
        follower_id: 0,
        rotation_speed: 3.0,
        ..Default::default()
    });

    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        })
        .insert(follower::Follower {
            id: 0,
            f_type: follower::FollowerType::LookAt,
        })
        .insert(follower::FollowerConfig {
            transition_time: 1.5,
            up_direction: Vec3::Y,
        })
        .insert(follower::FollowerPosition {
            position_state: follower::PositionState::Normal,
            current_position: follower::Position {
                distance: 20.0,
                to_camera: Vec3::new(1.0, 1.0, 0.0).normalize(),
            },
        })
        .insert_bundle(PickingCameraBundle::default());

    // player
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(bevy::prelude::shape::Icosphere {
                radius: 0.5,
                subdivisions: 1,
            })),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(0.8, 0.7, 0.6),
                perceptual_roughness: 1.0,
                ..Default::default()
            }),
            ..Default::default()
        })
        .insert_bundle(RigidBodyBundle {
            position: Vec3::new(0.0, 10.0, 0.0).into(),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::ball(0.5).into(),
            material: ColliderMaterial {
                friction: 2.0,
                restitution: 0.7,
                ..Default::default()
            }
            .into(),
            flags: ColliderFlagsComponent(ColliderFlags {
                active_events: ActiveEvents::INTERSECTION_EVENTS,
                ..Default::default()
            }),
            ..Default::default()
        })
        .insert(RigidBodyPositionSync::Discrete)
        .insert(follower::FollowerTarget)
        .insert(Player)
        .insert(editor_enhanced::ColliderAdded)
        .insert_bundle(PickableBundle::default());

    // debug line
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(debug_line::DebugLine::default())),
            material: materials.add(StandardMaterial {
                emissive: Color::rgb(1.0, 0.0, 0.0),
                ..Default::default()
            }),
            ..Default::default()
        })
        .insert(debug_line::DebugLineMarker);

    // light
    commands
        .spawn_bundle(PointLightBundle {
            point_light: PointLight {
                intensity: 1500.0,
                shadows_enabled: true,
                ..Default::default()
            },
            transform: Transform::from_xyz(4.0, 8.0, 4.0),
            ..Default::default()
        })
        .insert(follower::Follower {
            id: 1,
            f_type: follower::FollowerType::Follow,
        })
        .insert(follower::FollowerConfig {
            transition_time: 1.5,
            up_direction: Vec3::Y,
        })
        .insert(follower::FollowerPosition {
            position_state: follower::PositionState::Normal,
            current_position: follower::Position {
                distance: 5.0,
                to_camera: Vec3::new(1.0, 1.0, 0.0).normalize(),
            },
        });
}

pub fn apply_forces(
    keys: Res<Input<KeyCode>>,
    mut rigid_bodies: Query<
        (
            // &mut RigidBodyForcesComponent,
            &mut RigidBodyVelocityComponent,
            &RigidBodyMassPropsComponent,
        ),
        With<Player>,
    >,
    camera: Query<&follower::FollowerPosition, With<Camera>>,

    mut meshes: ResMut<Assets<Mesh>>,
    debug_line: Query<&Handle<Mesh>, With<debug_line::DebugLineMarker>>,
) {
    let line_handle = debug_line.single();

    let position = camera.single();
    let mut forward = -position.current_position.to_camera;
    forward.y = 0.0;
    forward = forward.normalize();
    let right = forward.cross(Vec3::Y);

    if let Some(m) = meshes.get_mut(line_handle) {
        *m = debug_line::DebugLine {
            start: Vec3::new(0.0, forward.y, 0.0),
            end: forward * 10.0,
        }
        .into()
    }

    let torque_mul = 0.01;
    let mut torque = Vec3::default();
    if keys.pressed(KeyCode::W) {
        torque = -right * torque_mul;
    }
    if keys.pressed(KeyCode::S) {
        torque = right * torque_mul;
    }
    if keys.pressed(KeyCode::A) {
        torque = -forward * torque_mul;
    }
    if keys.pressed(KeyCode::D) {
        torque = forward * torque_mul;
    }
    let mut jump = false;
    if keys.just_pressed(KeyCode::Space) {
        jump = true
    }

    for (mut rb_vel, rb_mprops) in rigid_bodies.iter_mut() {
        // Apply forces.
        // rb_forces.force = Vec3::new(1.0, 2.0, 3.0).into();
        // rb_forces.torque = torque.into();

        // Apply impulses.
        // rb_vel.apply_impulse(rb_mprops, Vec3::new(100.0, 200.0, 300.0).into());
        // rb_vel.apply_impulse(rb_mprops, torque.into());

        // torque is applyed around the 'torque' axis
        rb_vel.apply_torque_impulse(rb_mprops, torque.into()); //Vec3::new(140.0, 80.0, 20.0).into());

        if jump {
            rb_vel.apply_impulse(rb_mprops, Vec3::new(0.0, 2.0, 0.0).into());
        }
    }
}
