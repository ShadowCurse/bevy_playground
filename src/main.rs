use bevy::diagnostic::*;
use bevy::prelude::*;
use bevy::render::primitives::Aabb;
use bevy_editor_pls::prelude::*;
// use bevy_obj::*;
use bevy_rapier3d::prelude::*;
// use heron::prelude::*;

mod debug_line;
mod editor_enhanced;
mod follower;
mod level;
mod player;
mod animated_shader;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticsPlugin)
        .add_plugin(EntityCountDiagnosticsPlugin)
        .add_plugin(EditorPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
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
        .add_system(player::apply_forces)
        .add_system(bevy::input::system::exit_on_esc_system)
        .add_system(display_events)
        .run();
}

fn load_scene(
    asset_server: Res<AssetServer>,
    mut scene_spawner: ResMut<SceneSpawner>,
) {
    let my_gltf = asset_server.load("scene.glb#Scene0");
    scene_spawner.spawn(my_gltf);
    asset_server.watch_for_changes().unwrap();
}

pub fn apply_physics(
    mut commands: Commands,
    objects: Query<
        (Entity, &GlobalTransform, &Transform, &Aabb),
        Without<editor_enhanced::ColliderAdded>,
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
            .insert(editor_enhanced::ColliderAdded);
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
