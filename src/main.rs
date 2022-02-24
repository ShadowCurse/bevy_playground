use bevy::diagnostic::*;
use bevy::prelude::*;
use bevy_editor_pls::prelude::*;
use bevy_mod_picking::DefaultPickingPlugins;
use bevy_rapier3d::prelude::*;

mod animated_shader;
mod debug_line;
mod editor_enhanced;
mod follower;
mod scene;
mod player;

fn main() {
    App::new()
        // default
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticsPlugin)
        .add_plugin(EntityCountDiagnosticsPlugin)
        .add_plugin(EditorPlugin)
        // mod picking and raycasting
        .add_plugins(DefaultPickingPlugins)
        // .add_system_to_stage(CoreStage::PostUpdate, print_events)
        // physics
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        // custom
        .add_plugin(animated_shader::CustomMaterialPlugin)
        .add_plugin(follower::FollowCameraPlugin)
        .add_plugin(editor_enhanced::EditorAdditionsPlugin)
        .add_plugin(player::PlayerPlugin)
        .add_plugin(scene::ScenePlugin)
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
