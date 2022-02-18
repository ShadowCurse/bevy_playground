use bevy::prelude::*;
use bevy::render::primitives::Aabb;
use bevy_editor_pls::default_windows::add::*;
use bevy_rapier3d::prelude::*;

#[derive(Default, Component, Reflect)]
#[reflect(Component)]
pub struct TmpColliderType {
    pub solid: bool,
    pub sensor: bool,
}

impl Into<ColliderType> for &TmpColliderType {
    fn into(self) -> ColliderType {
        if self.solid {
            ColliderType::Solid
        } else {
            ColliderType::Sensor
        }
    }
}

#[derive(Default, Component, Reflect)]
#[reflect(Component)]
pub struct TmpRigidBodyType {
    pub dynamic: bool,
    pub static_: bool,
    pub kinematic_position_based: bool,
    pub kinematic_velocity_based: bool,
}

impl Into<RigidBodyType> for &TmpRigidBodyType {
    fn into(self) -> RigidBodyType {
        if self.dynamic {
            RigidBodyType::Dynamic
        } else if self.static_ {
            RigidBodyType::Static
        } else if self.kinematic_position_based {
            RigidBodyType::KinematicPositionBased
        } else {
            RigidBodyType::KinematicVelocityBased
        }
    }
}

#[derive(Default, Component, Reflect)]
#[reflect(Component)]
pub struct TmpRigidBodyPosition(Vec3);

impl Into<RigidBodyPosition> for &TmpRigidBodyPosition {
    fn into(self) -> RigidBodyPosition {
        self.0.into()
    }
}

// #[derive(Default, Component, Reflect)]
// #[reflect(Component)]
// pub struct TmpRigidBodyVelocity {
//     pub linvel: Vec3,
//     pub angvel: Vec3,
// }

// impl Into<RigidBodyVelocity> for &TmpRigidBodyVelocity {
//     fn into(self) -> RigidBodyVelocity {
//         RigidBodyVelocity {
//             linvel: self.linvel.into(),
//             angvel: self.angvel.into(),
//         }
//     }
// }

pub struct EditorAdditionsPlugin;

impl Plugin for EditorAdditionsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<TmpColliderType>();
        app.register_type::<TmpRigidBodyType>();
        app.register_type::<TmpRigidBodyPosition>();
        app.add_startup_system(editor_setup);
    }
}

pub fn editor_setup(mut editor: ResMut<bevy_editor_pls::Editor>) {
    let mut state = editor.window_state_mut::<AddWindow>().unwrap();

    add_collider(&mut state);
    add_rigit_body(&mut state);
}

pub fn collider_components(
    global_transform: &GlobalTransform,
    transform: &Transform,
    aabb: &Aabb,
    collider_type: &TmpColliderType,
) -> (
    ColliderTypeComponent,
    ColliderShapeComponent,
    ColliderPositionComponent,
) {
    (
        ColliderTypeComponent(collider_type.into()),
        ColliderShape::cuboid(
            global_transform.scale.x * transform.scale.x * aabb.half_extents.x,
            global_transform.scale.y * transform.scale.y * aabb.half_extents.y,
            global_transform.scale.z * transform.scale.z * aabb.half_extents.z,
        )
        .into(),
        (
            global_transform.translation + transform.translation,
            global_transform.rotation * transform.rotation,
        )
            .into(),
    )
}

pub fn add_collider(state: &mut AddWindowState) {
    state.add("PhysicsTmp", AddItem::component::<TmpColliderType>());

    state.add(
        "Physics",
        AddItem::new("ColliderBundle".into(), |world, entity| {
            let mut e = world.entity_mut(entity);
            if let Some(global_transform) = e.get::<GlobalTransform>() {
                if let Some(transform) = e.get::<Transform>() {
                    if let Some(aabb) = e.get::<Aabb>() {
                        if let Some(collider_type) = e.get::<TmpColliderType>() {
                            let (collider_type, shape, position) = collider_components(
                                global_transform,
                                transform,
                                aabb,
                                collider_type,
                            );
                            e.insert_bundle(ColliderBundle {
                                collider_type,
                                shape,
                                position,
                                ..Default::default()
                            });
                        }
                    }
                }
            }
        }),
    );
    state.add(
        "Physics",
        AddItem::new("UpdateCollider".into(), |world, entity| {
            let mut e = world.entity_mut(entity);
            if let Some(global_transform) = e.get::<GlobalTransform>() {
                if let Some(transform) = e.get::<Transform>() {
                    if let Some(aabb) = e.get::<Aabb>() {
                        if let Some(collider_type) = e.get::<TmpColliderType>() {
                            if let Some(mut type_component) = e.get_mut::<ColliderTypeComponent>() {
                                if let Some(mut shape_component) =
                                    e.get_mut::<ColliderShapeComponent>()
                                {
                                    if let Some(mut position_component) =
                                        e.get_mut::<ColliderPositionComponent>()
                                    {
                                        let (collider_type, shape, position) = collider_components(
                                            global_transform,
                                            transform,
                                            aabb,
                                            collider_type,
                                        );
                                        *type_component = collider_type;
                                        *shape_component = shape;
                                        *position_component = position;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }),
    );
}

pub fn add_rigit_body(state: &mut AddWindowState) {
    state.add("PhysicsTmp", AddItem::component::<TmpRigidBodyType>());
    state.add("PhysicsTmp", AddItem::component::<TmpRigidBodyPosition>());

    state.add("Physics", AddItem::component::<RigidBodyPositionSync>());
    state.add(
        "Physics",
        AddItem::new("RigidBodyBundle".into(), |world, entity| {
            let mut e = world.entity_mut(entity);
            if let Some(body_type) = e.get::<TmpRigidBodyType>() {
                if let Some(pos) = e.get::<TmpRigidBodyPosition>() {
                    e.insert_bundle(RigidBodyBundle {
                        body_type: RigidBodyTypeComponent(body_type.into()),
                        position: RigidBodyPositionComponent(pos.into()),
                        ..Default::default()
                    });
                }
            }
        }),
    );
    state.add(
        "Physics",
        AddItem::new("UpdateRigidBody".into(), |world, entity| {
            let mut e = world.entity_mut(entity);

            if let Some(body_type) = e.get::<TmpRigidBodyType>() {
                if let Some(pos) = e.get::<TmpRigidBodyPosition>() {
                    if let Some(mut body_type_c) = e.get_mut::<RigidBodyTypeComponent>() {
                        if let Some(mut position_c) = e.get_mut::<RigidBodyPositionComponent>() {
                            *body_type_c = RigidBodyTypeComponent(body_type.into());
                            *position_c = RigidBodyPositionComponent(pos.into());
                        }
                    }
                }
            }
        }),
    );
}
