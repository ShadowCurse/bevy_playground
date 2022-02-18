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

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticsPlugin)
        .add_plugin(EntityCountDiagnosticsPlugin)
        .add_plugin(EditorPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        // .add_plugin(ObjPlugin)
        // .add_plugin(PhysicsPlugin::default())
        .add_plugin(follower::FollowCameraPlugin)
        .add_plugin(editor_enhanced::EditorAdditionsPlugin)
        // .add_startup_system(level::setup_level)
        .add_startup_system(load_scene.label("load_scene"))
        .add_startup_system(apply_physics.label("apply_physics").after("load_scene"))
        .add_startup_system(player::setup.after("apply_physics"))
        .add_system(player::apply_forces)
        .add_system(bevy::input::system::exit_on_esc_system)
        .add_system(display_events)
        .run();
}

fn load_scene(
    // mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut scene_spawner: ResMut<SceneSpawner>,
) {
    // Scenes are loaded just like any other asset.
    // let scene_handle: Handle<DynamicScene> = asset_server.load("scene.scn.ron");
    // SceneSpawner can "spawn" scenes. "Spawning" a scene creates a new instance of the scene in
    // the World with new entity ids. This guarantees that it will not overwrite existing
    // entities.
    // scene_spawner.spawn_dynamic(scene_handle);

    let my_gltf = asset_server.load("scene.glb#Scene0");
    // commands.spawn_bundle((
    //     Transform::from_xyz(0.0, 0.0, 0.0),
    //     GlobalTransform::identity(),
    // )).with_children(|parent| {
    //     parent.spawn_scene(my_gltf);
    // });
    scene_spawner.spawn(my_gltf);

    // This tells the AssetServer to watch for changes to assets.
    // It enables our scenes to automatically reload in game when we modify their files
    // asset_server.watch_for_changes().unwrap();
}

pub fn apply_physics(
    mut commands: Commands,
    objects: Query<(Entity, &GlobalTransform, &Transform, &Aabb)>,
) {
        println!("apply_physics");
    for (e, gt, t, aabb) in objects.iter() {
        println!("rstrs");
        let (collider_type, shape, position) = editor_enhanced::collider_components(
            gt,
            t,
            aabb,
            &editor_enhanced::TmpColliderType {
                solid: true,
                sensor: false,
            },
        );
        commands.entity(e).insert_bundle(ColliderBundle {
            collider_type,
            shape,
            position,
            ..Default::default()
        });
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
