use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

#[derive(Default, Clone)]
pub struct PipeLine {
    pub root: Option<Entity>,
    pub current: Option<Entity>,
    pub timer: Timer,
}

impl PipeLine {
    pub fn new(root: Entity) -> Self {
        Self {
            root: Some(root),
            current: Some(root),
            timer: Timer::from_seconds(0.5, true),
        }
    }

    pub fn add_pipe(&mut self, pipe: Entity) {
        if self.root.is_none() {
            self.root = Some(pipe);
            self.current = Some(pipe);
        } else {
            self.current = Some(pipe);
        }
    }
}
