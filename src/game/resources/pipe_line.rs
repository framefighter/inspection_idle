use bevy::{prelude::*, render::pass::TextureAttachment};
use bevy_inspector_egui::Inspectable;

use crate::game::components::robot::AttachmentPointId;

#[derive(Default, Clone)]
pub struct PipeLine {
    pub root: Option<Entity>,
    pub current: Vec<(Entity, AttachmentPointId)>,
    pub timer: Timer,
}

impl PipeLine {
    pub fn new(root: Entity, aid: AttachmentPointId) -> Self {
        Self {
            root: Some(root),
            current: vec![(root, aid)],
            timer: Timer::from_seconds(0.5, true),
        }
    }

    pub fn add_pipe(
        &mut self,
        pipe: Entity,
        aids: Vec<AttachmentPointId>,
        // parent: (Entity, AttachmentPointId),
    ) {
        if self.root.is_none() {
            self.root = Some(pipe);
        }
        for aid in aids {
            self.current.push((pipe, aid));
        }
        // if let Some(index) = self.current.iter().position(|&e_aid| e_aid == parent) {
        //     self.current.swap_remove(index);
        // }
    }
}
