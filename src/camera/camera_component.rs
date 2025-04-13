use bevy::prelude::Component;
use bevy::prelude::Reflect;
use leafwing_input_manager::Actionlike;

pub struct IsometricCameraPlugin;

#[derive(Component)]
pub struct MainCamera;

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum CameraAction {
    TranslationXInc,
    TranslationXDec,
    TranslationYInc,
    TranslationYDec,
    TranslationZInc,
    TranslationZDec,
    RotateXInc,
    RotateXDec,
    RotateYInc,
    RotateYDec,
    RotateZInc,
    RotateZDec,
    ResetXYZ,
}
