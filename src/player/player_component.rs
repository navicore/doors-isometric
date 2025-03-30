use avian3d::prelude::*;
use bevy::prelude::*;
use bevy::reflect::Reflect;
use leafwing_input_manager::{prelude::InputMap, Actionlike, InputManagerBundle};

// Define movement constants
const PLAYER_WALK_SPEED: f32 = 4.0; // Horizontal movement speed

pub const PLAYER_SHAPE_X: f32 = 1.2;
pub const PLAYER_SHAPE_Y: f32 = 1.8;
pub const PLAYER_SHAPE_Z: f32 = 1.2;

const PLAYER_MASS: f32 = 4.0;
pub const PLAYER_JUMP_FORCE: f32 = 250.0; // Jump force applied when pressing space
pub const PLAYER_GRAVITY_SCALE: f32 = 2.5; // Gravity multiplier for falling speed

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum Action {
    MoveForward,
    MoveBackward,
    MoveLeft,
    MoveRight,
    Jump,
    Enter,
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
    pub fn new(// texture: Handle<Image>,
    ) -> Self {
        let input_map = InputMap::new([
            (Action::MoveForward, KeyCode::ArrowUp),
            (Action::MoveBackward, KeyCode::ArrowDown),
            (Action::MoveLeft, KeyCode::ArrowLeft),
            (Action::MoveRight, KeyCode::ArrowRight),
            (Action::Jump, KeyCode::Space),
            (Action::Enter, KeyCode::Enter),
        ]);

        Self {
            transform: Transform::from_translation(Vec3::new(0.0, 8.0, 0.0)), // Start above the floor
            rigid_body: RigidBody::Dynamic,
            collider: Collider::cuboid(PLAYER_SHAPE_X, PLAYER_SHAPE_Y, PLAYER_SHAPE_Z), // Collider matching the player size
            external_force: ExternalForce::default(),
            player: Player::default(),
            input_manager: InputManagerBundle::with_map(input_map),
            gravity: GravityScale(PLAYER_GRAVITY_SCALE),
            mass: Mass(PLAYER_MASS),
            friction: Friction {
                // TODO: tune
                dynamic_coefficient: 0.3,
                static_coefficient: 0.5,
                combine_rule: CoefficientCombine::Average,
            },
            movable: Movable,
            grounded: Grounded(false),
        }
    }
}

#[derive(Component)]
pub struct Grounded(pub bool);

#[derive(Default, Resource)]
pub struct GroundedState(pub bool);

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
