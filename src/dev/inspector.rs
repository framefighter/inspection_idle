use bevy::prelude::{AppBuilder, Plugin};
use bevy_inspector_egui::{
    Inspectable, InspectableRegistry, InspectorPlugin, WorldInspectorPlugin,
};
use std::fmt::{Debug, Display};

use crate::{game::types::*, ui::types::UiState};

pub struct InspectAllPlugin;

impl Plugin for InspectAllPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(WorldInspectorPlugin::new())
            .add_plugin(InspectorPlugin::<UiState>::new());
        let mut registry = app
            .world_mut()
            .get_resource_or_insert_with(InspectableRegistry::default);
        registry.register::<InfoText>();
        registry.register::<Agility>();
        registry.register::<Battery>();
        registry.register::<Sensor>();
        registry.register::<Quality>();
        registry.register::<Cargo>();
        registry.register::<MovementAbility>();
        registry.register::<AutonomyLevel>();
        registry.register::<GroundMovement>();
        registry.register::<WheelType>();
        registry.register::<AirMovement>();
        registry.register::<WaterMovement>();
        registry.register::<SpaceMovement>();
    }
}

pub struct VecAsDropdown<T>
where
    T: Clone + PartialEq,
{
    pub from: Vec<T>,
    pub selected: usize,
}

impl<T> VecAsDropdown<T>
where
    T: Clone + PartialEq,
{
    pub fn new(from_vec: Vec<T>) -> Self {
        Self {
            from: from_vec,
            selected: 0,
        }
    }

    pub fn push(&mut self, item: T) {
        self.from.push(item);
    }

    pub fn selected_value(&self) -> T {
        self.from[self.selected].clone()
    }
}

impl<T> Default for VecAsDropdown<T>
where
    T: Clone + PartialEq,
{
    fn default() -> Self {
        Self {
            from: Vec::new(),
            selected: 0,
        }
    }
}

impl<T> Inspectable for VecAsDropdown<T>
where
    T: Clone + PartialEq + Debug + Default,
{
    type Attributes = Vec<T>;

    fn ui(
        &mut self,
        ui: &mut bevy_inspector_egui::egui::Ui,
        _: Self::Attributes,
        _: &bevy_inspector_egui::Context,
    ) -> bool {
        let mut display = T::default();
        if self.from.len() > 0 {
            display = self.from[self.selected].clone();
        }
        let hash = format!("{:?}", self.from);

        bevy_inspector_egui::egui::ComboBox::from_id_source(hash)
            .selected_text(format!("{:?}", display))
            .show_ui(ui, |ui| {
                for (index, value) in self.from.iter().enumerate() {
                    ui.selectable_value(&mut self.selected, index, format!("{:?}", value));
                }
            });
        true
    }

    fn setup(_: &mut AppBuilder) {
        // eprintln!("Running setup code...");

        // app.init_resource::<WhateverYouNeed>();
    }
}
