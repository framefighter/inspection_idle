use bevy::{prelude::*, utils::HashMap};

use crate::{game::components::robot::*, utils::get_robot_body};

pub fn update_consumption(mut query: Query<(&mut EnergyConsumption, &ParentEntity)>) {
    let mut total_consumptions: HashMap<Entity, f32> = HashMap::default();
    query.for_each_mut(|(mut energy_consumption, parent_entity)| {
        if let Some(parent) = get_robot_body(parent_entity) {
            total_consumptions
                .entry(parent)
                .and_modify(|total_consumption| {
                    *total_consumption += energy_consumption.consumption;
                })
                .or_insert(energy_consumption.consumption);
            energy_consumption.consumption = 0.0;
        }
    });
    total_consumptions
        .iter()
        .for_each(|(parent, total_consumption)| {
            query
                .get_mut(*parent)
                .map(|(mut energy_consumption, _)| {
                    energy_consumption.consumption += *total_consumption;
                })
                .ok();
        });
}

pub fn update_battery(
    query: Query<(&ParentEntity, &mut Battery)>,
    mut parent_query: Query<&mut EnergyConsumption>,
) {
    query.for_each_mut(
        |(parent_entity, mut battery)| {
            if let Some(parent) = get_robot_body(parent_entity) {
                parent_query.get_mut(parent).map(|mut energy_consumption| {
                    battery.charge -= energy_consumption.consumption;
                    energy_consumption.consumption = 0.0;
                }).ok();
            }
        },
    );
}
