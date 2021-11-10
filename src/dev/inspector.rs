use bevy::prelude::{AppBuilder, Plugin};
use bevy_inspector_egui::{InspectableRegistry, WorldInspectorPlugin};

use crate::game::types::*;

pub struct InspectAllPlugin;

impl Plugin for InspectAllPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(WorldInspectorPlugin::new());
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
