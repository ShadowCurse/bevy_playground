use bevy::diagnostic::*;
use bevy::prelude::*;
use bevy::render::primitives::Aabb;
use bevy_editor_pls::prelude::*;
use bevy_mod_picking::PickingCamera;
use bevy_mod_picking::{DefaultPickingPlugins, PickableBundle, PickableMesh, PickingEvent};
use bevy_rapier3d::prelude::*;

mod animated_shader;
mod debug_line;
mod editor_enhanced;
mod follower;
mod level;
mod player;

fn main() {
    App::new()
        // default
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticsPlugin)
        .add_plugin(EntityCountDiagnosticsPlugin)
        .add_plugin(EditorPlugin)
        // mod picking
        .add_plugins(DefaultPickingPlugins)
        // .add_system_to_stage(CoreStage::PostUpdate, print_events)
        // physics
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        // custom
        .add_plugin(animated_shader::CustomMaterialPlugin)
        .add_plugin(follower::FollowCameraPlugin)
        .add_plugin(editor_enhanced::EditorAdditionsPlugin)
        .add_startup_system(level::setup_level)
        .add_startup_system(load_scene)
        .add_startup_system(player::setup)
        .add_stage_after(
            CoreStage::PostUpdate,
            "physics",
            SystemStage::single_threaded(),
        )
        .add_system_to_stage("physics", apply_physics)
        .add_system_to_stage(CoreStage::PostUpdate, spawn_obj)
        .add_system_to_stage(CoreStage::PreUpdate, remove_obj)
        .add_system(player::apply_forces)
        .add_system(bevy::input::system::exit_on_esc_system)
        .add_system(display_events)
        .run();
}

// pub fn print_events(mut events: EventReader<PickingEvent>) {
//     for event in events.iter() {
//         match event {
//             PickingEvent::Selection(e) => info!("A selection event happened: {:?}", e),
//             PickingEvent::Hover(e) => info!("Egads! A hover event!? {:?}", e),
//             PickingEvent::Clicked(e) => info!("Gee Willikers, it's a click! {:?}", e),
//         }
//     }
// }

pub fn spawn_obj(
    mut commands: Commands,
    mouse_button_input: Res<Input<MouseButton>>,
    pc: Query<&PickingCamera>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        for ps in pc.iter() {
            if let Some((_e, i)) = ps.intersect_top() {
                println!("spawning obj with position: {:?}", i.position());
                commands.spawn().insert_bundle((
                    meshes.add(Mesh::from(bevy::prelude::shape::Icosphere {
                        radius: 0.2,
                        subdivisions: 5,
                    })),
                    Transform::from_translation(i.position()),
                    GlobalTransform::default(),
                    animated_shader::CustomMaterial,
                    Visibility::default(),
                    ComputedVisibility::default(),
                ));
            }
        }
    }
}

pub fn remove_obj(
    mut commands: Commands,
    mouse_button_input: Res<Input<MouseButton>>,
    pc: Query<&PickingCamera>,
) {
    if mouse_button_input.just_pressed(MouseButton::Right) {
        for ps in pc.iter() {
            if let Some((e, _i)) = ps.intersect_top() {
                println!("removing obj: {:?}", e);
                commands.entity(e).despawn();
            }
        }
    }
}

pub fn load_scene(asset_server: Res<AssetServer>, mut scene_spawner: ResMut<SceneSpawner>) {
    let my_gltf = asset_server.load("scene.glb#Scene0");
    scene_spawner.spawn(my_gltf);
    asset_server.watch_for_changes().unwrap();
}

pub fn apply_physics(
    mut commands: Commands,
    objects: Query<
        (Entity, &GlobalTransform, &Transform, &Aabb),
        (
            Without<editor_enhanced::ColliderAdded>,
            Without<PickableMesh>,
        ),
    >,
) {
    for (e, gt, t, aabb) in objects.iter() {
        println!("applyind physics: {:?}, {:?}, {:?}, {:?}", e, gt, t, aabb);
        let (collider_type, shape, position) = editor_enhanced::collider_components(
            gt,
            t,
            aabb,
            &editor_enhanced::TmpColliderType {
                solid: true,
                sensor: false,
            },
        );
        commands
            .entity(e)
            .insert_bundle(ColliderBundle {
                collider_type,
                shape,
                position,
                ..Default::default()
            })
            .insert(editor_enhanced::ColliderAdded)
            .insert_bundle(PickableBundle::default());
    }
}

fn display_events(
    mut intersection_events: EventReader<IntersectionEvent>,
    mut contact_events: EventReader<ContactEvent>,
) {
    for intersection_event in intersection_events.iter() {
        println!("Received intersection event: {:?}", intersection_event);
    }

    for contact_event in contact_events.iter() {
        println!("Received contact event: {:?}", contact_event);
    }
}
