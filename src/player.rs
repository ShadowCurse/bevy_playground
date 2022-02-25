use std::ops::Deref;
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
            .register_type::<PlayerControllerSettings>()
            .add_startup_system(setup_camera)
            .add_startup_system(setup_player)
            .add_system(apply_forces);
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Debug, Default, Component, Reflect)]
#[reflect(Component)]
pub struct PlayerControllerSettings {
    pub max_speed: f32,
    pub acceleration: f32,
    pub max_accel_force: f32,
    pub ride_height: f32,
    pub force_str: f32,
    pub spring_str: f32,
    pub spring_damper: f32,
    pub upright_spring_str: f32,
    pub upright_spring_damper: f32,
    pub rotate_str: f32,
    pub jump_str: f32,
}

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
            // mass_properties: RigidBodyMassPropsComponent(RigidBodyMassProps {
            //     flags: RigidBodyMassPropsFlags::ROTATION_LOCKED,
            //     ..Default::default()
            // }),
            forces: RigidBodyForces {
                gravity_scale: 0.0,
                ..Default::default()
            }
            .into(),
            // damping: RigidBodyDampingComponent(RigidBodyDamping {
            //     linear_damping: 1.0,
            //     ..Default::default()
            // }),
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
            // mass_properties: ColliderMassPropsComponent(ColliderMassProps::Density(0.5)),
            ..Default::default()
        })
        .insert(RigidBodyPositionSync::Discrete)
        .insert(follower::FollowerTarget)
        .insert(Player)
        .insert(PlayerControllerSettings {
            max_speed: 10.0,
            acceleration: 10.0,
            max_accel_force: 10.0,
            ride_height: 2.0,
            force_str: 10.0,
            spring_str: 10.0,
            spring_damper: 1.0,
            upright_spring_str: 20.0,
            upright_spring_damper: 5.0,
            rotate_str: 10.0,
            jump_str: 10.0,
        })
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
            &PlayerControllerSettings,
            &Transform,
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

    let mut force = Vec3::default();
    if keys.pressed(KeyCode::W) {
        force = forward;
    }
    if keys.pressed(KeyCode::S) {
        force = -forward;
    }
    if keys.pressed(KeyCode::A) {
        force = -right;
    }
    if keys.pressed(KeyCode::D) {
        force = right;
    }
    let mut jump = false;
    if keys.just_pressed(KeyCode::Space) {
        jump = true
    }

    for (mut rb_forces, mut rb_vel, rb_mprops, settings, t) in rigid_bodies.iter_mut() {
        let ray = ray.single();
        if let Some(top_intersection) = ray.intersect_top() {
            let vel = rb_vel.deref().linvel;
            let ray_dir = ray.ray().unwrap().direction();

            let relative_vel = ray_dir.dot(vel.into());

            let diff = settings.ride_height - top_intersection.1.distance();
            let spring_force =
                (diff * settings.spring_str + relative_vel * settings.spring_damper) * Vec3::Y;
            // Floating
            rb_forces.force = spring_force.into();
        }

        // Movement
        rb_forces.force += Vector::from(force * settings.force_str);

        let player_up = -ray.ray().unwrap().direction();
        let rotation = Quat::from_rotation_arc_colinear(player_up, Vec3::Y);
        let (axis, angle) = rotation.to_axis_angle();
        let ang_vel: Vec3 = rb_vel.deref().angvel.into();
        // Staing upright rotation
        let upright_torque =
            axis * angle * settings.upright_spring_str - ang_vel * settings.upright_spring_damper;

        let player_forward = t.rotation.mul_vec3(Vec3::new(1.0, 0.0, 0.0));
        let rotation = Quat::from_rotation_arc(player_forward, forward);
        let (axis, angle) = rotation.to_axis_angle();
        // Forward rotation
        let rotate_torque = axis * angle * settings.rotate_str;

        // Torque
        rb_forces.torque = (upright_torque + rotate_torque).into();

        if jump {
            rb_vel.apply_impulse(rb_mprops, Vec3::new(0.0, settings.jump_str, 0.0).into());
        }
    }
}
