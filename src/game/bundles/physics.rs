use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Default, Bundle)]
pub struct PhysicsBundle {
    #[bundle]
    pub rigid_body: RigidBodyBundle,
    pub pos_sync: RigidBodyPositionSync,
}