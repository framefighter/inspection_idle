use bevy::prelude::*;
use bevy_interact_2d::Interactable;

use crate::game::components::robot::*;

#[derive(Default, Bundle)]
pub struct InteractionMarkerBundle {
    #[bundle]
    pub sprite: SpriteSheetBundle,
    pub interactable: Interactable,
    pub apm: AttachmentPointMarker,
    pub aid: AttachmentPointId,
}
