use avian3d::prelude::*;
use bevy::prelude::*;
use bevy::reflect::Reflect;
use leafwing_input_manager::{prelude::InputMap, Actionlike, InputManagerBundle};

// Define movement constants
const PLAYER_WALK_SPEED: f32 = 4.0; // Horizontal movement speed

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum Action {
    Open,
    MoveForward,
    MoveBackward,
    MoveLeft,
    MoveRight,
    Jump,
}

#[derive(Resource, Default)]
pub struct PlayerStartPosition {
    pub position: Option<Vec3>,
}

#[derive(Component)]
pub struct Player {
    pub walk_speed: f32,
    pub state: PlayerState,
    pub direction: PlayerDirection,
}

impl Player {
    pub const fn default() -> Self {
        Self {
            walk_speed: PLAYER_WALK_SPEED,
            state: PlayerState::Stand,
            direction: PlayerDirection::Up,
        }
    }
}

#[derive(Bundle)]
pub struct PlayerBundle {
    pub transform: Transform,
    pub rigid_body: RigidBody,
    pub collider: Collider,
    pub external_force: ExternalForce,
    pub player: Player,
    pub input_manager: InputManagerBundle<Action>,
    pub gravity: GravityScale,
    pub mass: Mass,
    pub friction: Friction,
    pub movable: Movable,
    pub grounded: Grounded,
}

impl PlayerBundle {
    pub fn new(config: &PlayerConfig, position: Vec3) -> Self {
        let input_map = InputMap::new([
            (Action::MoveForward, KeyCode::ArrowUp),
            (Action::MoveBackward, KeyCode::ArrowDown),
            (Action::MoveLeft, KeyCode::ArrowLeft),
            (Action::MoveRight, KeyCode::ArrowRight),
            (Action::Jump, KeyCode::Space),
            (Action::Open, KeyCode::ShiftLeft),
        ]);

        Self {
            transform: Transform::from_translation(position), // Start above the floor
            rigid_body: RigidBody::Dynamic,
            collider: Collider::cuboid(config.x, config.y, config.z), // Collider matching the player size
            external_force: ExternalForce::default(),
            player: Player::default(),
            input_manager: InputManagerBundle::with_map(input_map),
            gravity: GravityScale(config.gravity_scale),
            mass: Mass(config.mass),
            friction: Friction {
                dynamic_coefficient: config.dynamic_coefficient,
                static_coefficient: config.static_coefficient,
                combine_rule: CoefficientCombine::Average,
            },
            movable: Movable,
            grounded: Grounded(false),
        }
    }
}

#[derive(Resource)]
pub struct PlayerConfig {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub jump_force: f32,
    pub gravity_scale: f32,
    pub mass: f32,
    pub dynamic_coefficient: f32,
    pub static_coefficient: f32,
}

impl Default for PlayerConfig {
    fn default() -> Self {
        Self {
            x: 1.2,
            y: 1.2,
            z: 1.2,
            jump_force: 10.0,
            gravity_scale: 2.5,
            mass: 4.0,
            dynamic_coefficient: 0.3,
            static_coefficient: 0.5,
        }
    }
}

#[derive(Component)]
pub struct Grounded(pub bool);

#[derive(Default, Resource)]
pub struct GroundedState {
    pub grounded: bool,
    pub timer: f32,
}

#[derive(Component)]
pub struct Movable;

#[derive(Debug, PartialEq, Eq)]
pub enum PlayerState {
    Walk,
    Stand,
    Jump,
    //Fall,
}

#[derive(Debug, PartialEq, Eq)]
pub enum PlayerDirection {
    Up,
    Down,
    Left,
    Right,
}
