use bevy::prelude::*;

#[derive(Debug, Component)]
pub struct FollowerTarget;

#[derive(Debug, Component)]
pub enum FollowerType {
    Follow,
    LookAt,
}

#[derive(Debug, Component)]
pub struct Follower {
    pub id: u32,
    pub f_type: FollowerType,
}

#[derive(Debug, Default, Component)]
pub struct Position {
    pub distance: f32,
    pub to_camera: Vec3,
}

impl Position {
    pub fn update(&mut self, h_angle: f32, up: Vec3, v_angle: f32, right: Vec3) -> &Self {
        self.to_camera = Quat::from_axis_angle(right, v_angle)
            * Quat::from_axis_angle(up, -h_angle)
            * self.to_camera;
        self
    }

    pub fn to_transform(&self, player_position: Vec3) -> Transform {
        Transform::from_translation(player_position + self.to_camera * self.distance)
    }

    pub fn transition_to(&self, delta: f32, new_pos: &Position) -> Position {
        Position {
            distance: self.distance + (new_pos.distance - self.distance) * delta,
            to_camera: self.to_camera.lerp(new_pos.to_camera, delta),
        }
    }
}

#[derive(Debug, Default, Component)]
pub struct NewPosition {
    pub transiton_start: f64,
    pub position: Position,
}

#[derive(Debug)]
pub enum PositionState {
    Normal,
    Transition(NewPosition),
}

impl Default for PositionState {
    fn default() -> Self {
        Self::Normal
    }
}

#[derive(Debug, Default, Component)]
pub struct FollowerPosition {
    pub position_state: PositionState,
    pub current_position: Position,
}

#[derive(Debug, Default, Component)]
pub struct FollowerConfig {
    pub transition_time: f64,
    pub up_direction: Vec3,
}

#[derive(Debug, Default, Component)]
pub struct FollowerController {
    pub follower_id: u32,
    pub rotation_speed: f32,
    pub rotate_right: bool,
    pub rotate_left: bool,
    pub rotate_up: bool,
    pub rotate_down: bool,
}

impl FollowerController {
    pub fn rotation_horizontal(&self) -> f32 {
        if self.rotate_right {
            self.rotation_speed
        } else if self.rotate_left {
            -self.rotation_speed
        } else {
            0.0
        }
    }
    pub fn rotation_vertical(&self) -> f32 {
        if self.rotate_up {
            -self.rotation_speed
        } else if self.rotate_down {
            self.rotation_speed
        } else {
            0.0
        }
    }
}

pub fn update_controller(keys: Res<Input<KeyCode>>, mut controller: ResMut<FollowerController>) {
    controller.rotate_left = keys.pressed(KeyCode::Left);
    controller.rotate_right = keys.pressed(KeyCode::Right);
    controller.rotate_up = keys.pressed(KeyCode::Up);
    controller.rotate_down = keys.pressed(KeyCode::Down);
}

pub fn update_followers(
    time: Res<Time>,
    controller: Res<FollowerController>,
    mut query: QuerySet<(
        QueryState<&Transform, With<FollowerTarget>>,
        QueryState<(
            &Follower,
            &FollowerConfig,
            &mut FollowerPosition,
            &mut Transform,
        )>,
    )>,
) {
    let player_position = query.q0().single().translation;

    for (follower, config, mut position, mut transform) in query.q1().iter_mut() {
        let mut final_pos = None;
        match &position.position_state {
            PositionState::Normal => {
                if follower.id == controller.follower_id {
                    let delta = time.delta().as_secs_f32();
                    let h_angle = controller.rotation_horizontal() * delta;
                    let v_angle = controller.rotation_vertical() * delta;

                    let up = config.up_direction.clone();
                    let right = (player_position - transform.translation)
                        .normalize()
                        .cross(up);

                    position
                        .current_position
                        .update(h_angle, up, v_angle, right);
                }

                let new_transform = position
                    .current_position
                    .to_transform(player_position.clone());

                *transform = match follower.f_type {
                    FollowerType::Follow => new_transform,
                    FollowerType::LookAt => {
                        new_transform.looking_at(player_position, config.up_direction)
                    }
                };
            }
            PositionState::Transition(new_pos) => {
                let now = time.seconds_since_startup();
                let mut delta = (now - new_pos.transiton_start) / config.transition_time;

                if delta >= 1.0 {
                    delta = 1.0;
                }

                let intermediate_pos = position
                    .current_position
                    .transition_to(delta as f32, &new_pos.position);

                let new_transform = intermediate_pos.to_transform(player_position.clone());

                *transform = match follower.f_type {
                    FollowerType::Follow => new_transform,
                    FollowerType::LookAt => {
                        new_transform.looking_at(player_position, config.up_direction)
                    }
                };

                if delta == 1.0 {
                    final_pos = Some(intermediate_pos);
                }
            }
        }
        if let Some(pos) = final_pos {
            position.position_state = PositionState::Normal;
            position.current_position = pos;
        }
    }
}

pub struct FollowCameraPlugin;

impl Plugin for FollowCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_controller)
            .add_system(update_followers);
    }
}
