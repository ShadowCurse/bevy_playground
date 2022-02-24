use bevy::prelude::*;
use bevy_mod_picking::{PickableBundle, PickingCameraBundle};
use bevy_mod_raycast::{DefaultPluginState, DefaultRaycastingPlugin, RayCastSource};
use bevy_rapier3d::prelude::*;

use crate::follower;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(DefaultRaycastingPlugin::<Player>::default())
            .insert_resource(DefaultPluginState::<Player>::default())
            .add_startup_system(setup_camera)
            .add_startup_system(setup_player)
            .add_system(apply_forces);
    }
}

#[derive(Component)]
pub struct Player;

pub fn setup_camera(mut commands: Commands) {
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
}

pub fn setup_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // player
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(bevy::prelude::shape::Box::new(1.0, 2.0, 1.0))),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(0.8, 0.7, 0.6),
                perceptual_roughness: 1.0,
                ..Default::default()
            }),
            ..Default::default()
        })
        .insert_bundle(RigidBodyBundle {
            position: Vec3::new(0.0, 10.0, 0.0).into(),
            mass_properties: RigidBodyMassPropsComponent(RigidBodyMassProps {
                flags: RigidBodyMassPropsFlags::ROTATION_LOCKED,
                ..Default::default()
            }),
            damping: RigidBodyDampingComponent(RigidBodyDamping {
                linear_damping: 30.0,
                ..Default::default()
            }),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::capsule(
                Vec3::new(0.0, 1.0, 0.0).into(),
                Vec3::new(0.0, -1.0, 0.0).into(),
                0.5,
            )
            .into(),
            // material: ColliderMaterial {
            //     friction: 20.0,
            //     restitution: 0.7,
            //     ..Default::default()
            // }
            // .into(),
            // flags: ColliderFlagsComponent(ColliderFlags {
            //     active_events: ActiveEvents::INTERSECTION_EVENTS,
            //     ..Default::default()
            // }),
            mass_properties: ColliderMassPropsComponent(ColliderMassProps::Density(0.5)),
            ..Default::default()
        })
        .insert(RigidBodyPositionSync::Discrete)
        .insert(follower::FollowerTarget)
        .insert(Player)
        .insert_bundle(PickableBundle::default())
        // ray caster
        .with_children(|command| {
            command
                .spawn()
                .insert(GlobalTransform::default())
                // originally ray points to -z direction
                // so we rotate it 90 degrees
                .insert(Transform::from_rotation(Quat::from_rotation_x(
                    -90.0_f32.to_radians(),
                )))
                .insert(RayCastSource::<Player>::new_transform_empty());
        });
}

pub fn apply_forces(
    keys: Res<Input<KeyCode>>,
    mut rigid_bodies: Query<
        (
            &mut RigidBodyForcesComponent,
            &mut RigidBodyVelocityComponent,
            &RigidBodyMassPropsComponent,
        ),
        With<Player>,
    >,
    camera: Query<&follower::FollowerPosition, With<Camera>>,
    ray: Query<&RayCastSource<Player>>,
) {
    let position = camera.single();
    let mut forward = -position.current_position.to_camera;
    forward.y = 0.0;
    forward = forward.normalize();
    let right = forward.cross(Vec3::Y);

    let force_str = 500.0;
    let mut force = Vec3::default();
    if keys.pressed(KeyCode::W) {
        force = forward * force_str;
    }
    if keys.pressed(KeyCode::S) {
        force = -forward * force_str;
    }
    if keys.pressed(KeyCode::A) {
        force = -right * force_str;
    }
    if keys.pressed(KeyCode::D) {
        force = right * force_str;
    }
    let mut jump = false;
    if keys.just_pressed(KeyCode::Space) {
        jump = true
    }

    let height = 2.0;
    let spring_str = 500.0;

    for (mut rb_forces, mut rb_vel, rb_mprops) in rigid_bodies.iter_mut() {
        let ray = ray.single();
        if let Some(top_intersection) = ray.intersect_top() {
            let diff = height - top_intersection.1.distance();
            let spring_force = diff * spring_str * Vec3::Y;
            rb_forces.force = spring_force.into();
        }

        // Apply forces.
        rb_forces.force += Vector::from(force);
        // rb_forces.torque = torque.into();

        // Apply impulses.
        // rb_vel.apply_impulse(rb_mprops, Vec3::new(100.0, 200.0, 300.0).into());
        // rb_vel.apply_impulse(rb_mprops, torque.into());

        // torque is applyed around the 'torque' axis
        // rb_vel.apply_torque_impulse(rb_mprops, torque.into()); //Vec3::new(140.0, 80.0, 20.0).into());

        if jump {
            rb_vel.apply_impulse(rb_mprops, Vec3::new(0.0, 200.0, 0.0).into());
        }
    }
}
