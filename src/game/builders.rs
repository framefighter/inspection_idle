use bevy::{ecs::system::EntityCommands, prelude::*};
use bevy::{
    math::Vec3,
    prelude::{Color, Transform},
};
use bevy_prototype_lyon::{
    entity::ShapeBundle,
    prelude::{DrawMode, FillOptions, GeometryBuilder, ShapeColors, StrokeOptions},
    shapes,
};

use super::types::{
    AirMovement, GroundMovement, MovementAbility, Quality, RobotBundle, Sensor, SpaceMovement,
    WaterMovement, WheelType,
};

pub struct RobotBuilder(RobotBundle);

impl RobotBuilder {
    #[must_use]
    pub fn new() -> Self {
        let shape = shapes::RegularPolygon {
            sides: 5,
            feature: shapes::RegularPolygonFeature::Radius(200.0),
            ..shapes::RegularPolygon::default()
        };

        let geometry = GeometryBuilder::build_as(
            &shape,
            ShapeColors::outlined(Color::TEAL, Color::BLACK),
            DrawMode::Outlined {
                fill_options: FillOptions::default(),
                outline_options: StrokeOptions::default().with_line_width(30.0),
            },
            Transform::from_scale(Vec3::new(0.1, 0.1, 0.1)),
        );
        Self(RobotBundle::default()).geometry(geometry)
    }

    pub fn name(mut self, name: &str) -> Self {
        self.0.info_text.name = name.to_string();
        self
    }

    pub fn description(mut self, description: &str) -> Self {
        self.0.info_text.description = description.to_string();
        self
    }

    pub fn max_speed(mut self, max_speed: f32) -> Self {
        self.0.agility.max_speed = max_speed;
        self
    }

    pub fn max_turn_speed(mut self, max_turn_speed: f32) -> Self {
        self.0.agility.max_turn_speed = max_turn_speed;
        self
    }

    pub fn battery_capacity(mut self, battery_capacity: f32) -> Self {
        self.0.battery.capacity = battery_capacity;
        self
    }

    pub fn battery_recharge_rate(mut self, charge_speed: f32) -> Self {
        self.0.battery.charge_speed = charge_speed;
        self
    }

    pub fn battery_charge(mut self, charge: f32) -> Self {
        self.0.battery.charge = charge;
        self
    }

    pub fn add_sensor(mut self, sensor: Sensor) -> Self {
        self.0.sensors.push(sensor);
        self
    }

    pub fn quality(mut self, quality: Quality) -> Self {
        self.0.quality = quality;
        self
    }

    pub fn cargo_capacity(mut self, cargo_capacity: f32) -> Self {
        self.0.cargo.capacity = cargo_capacity;
        self
    }

    pub fn car(mut self) -> Self {
        self.0.movement_ability = MovementAbilityBuilder::car().build();
        self
    }

    pub fn rover(mut self) -> Self {
        self.0.movement_ability = MovementAbilityBuilder::rover().build();
        self
    }

    pub fn legged(mut self) -> Self {
        self.0.movement_ability = MovementAbilityBuilder::legged().build();
        self
    }

    pub fn drone(mut self) -> Self {
        self.0.movement_ability = MovementAbilityBuilder::drone().build();
        self
    }

    pub fn plane(mut self) -> Self {
        self.0.movement_ability = MovementAbilityBuilder::plane().build();
        self
    }

    pub fn boat(mut self) -> Self {
        self.0.movement_ability = MovementAbilityBuilder::boat().build();
        self
    }

    pub fn submarine(mut self) -> Self {
        self.0.movement_ability = MovementAbilityBuilder::submarine().build();
        self
    }

    pub fn geometry(mut self, geometry: ShapeBundle) -> Self {
        self.0.geometry = geometry;
        self
    }

    #[must_use]
    pub fn build(self) -> RobotBundle {
        self.0
    }

    pub fn spawn<'a, 'b>(self, cmd: &'b mut Commands<'a>) -> EntityCommands<'a, 'b> {
        let mut entity_cmd = cmd.spawn_bundle(self.build());
        entity_cmd.with_children(|parent: &mut ChildBuilder| {
            let shape = shapes::RegularPolygon {
                sides: 3,
                feature: shapes::RegularPolygonFeature::Radius(200.0),
                ..shapes::RegularPolygon::default()
            };
            parent.spawn_bundle(GeometryBuilder::build_as(
                &shape,
                ShapeColors::outlined(Color::TEAL, Color::BLACK),
                DrawMode::Outlined {
                    fill_options: FillOptions::default(),
                    outline_options: StrokeOptions::default().with_line_width(30.0),
                },
                Transform::from_matrix(Mat4::from_scale_rotation_translation(
                    Vec3::splat(0.3),
                    Quat::default(),
                    Vec3::new(0.0, 300.0, 0.0),
                )),
            ));
        });
        entity_cmd
    }
}

pub struct MovementAbilityBuilder(MovementAbility);

impl MovementAbilityBuilder {
    #[must_use]
    pub fn new() -> Self {
        Self(MovementAbility::default())
    }

    pub fn car() -> Self {
        let mut ability = MovementAbility::default();
        ability.ground = GroundMovement::Wheels(WheelType::Street);
        Self(ability)
    }

    pub fn rover() -> Self {
        let mut ability = MovementAbility::default();
        ability.ground = GroundMovement::Tracks(WheelType::Street);
        Self(ability)
    }

    pub fn legged() -> Self {
        let mut ability = MovementAbility::default();
        ability.ground = GroundMovement::Legs;
        Self(ability)
    }

    pub fn drone() -> Self {
        let mut ability = MovementAbility::default();
        ability.air = AirMovement::Propellers;
        Self(ability)
    }

    pub fn plane() -> Self {
        let mut ability = MovementAbility::default();
        ability.air = AirMovement::Wings;
        Self(ability)
    }

    pub fn boat() -> Self {
        let mut ability = MovementAbility::default();
        ability.water = WaterMovement::Propellers;
        Self(ability)
    }

    pub fn submarine() -> Self {
        let mut ability = MovementAbility::default();
        ability.water = WaterMovement::Sub;
        Self(ability)
    }

    pub fn ground_movement(mut self, ground_movement: GroundMovement) -> Self {
        self.0.ground = ground_movement;
        self
    }

    pub fn air_movement(mut self, air_movement: AirMovement) -> Self {
        self.0.air = air_movement;
        self
    }

    pub fn water_movement(mut self, water_movement: WaterMovement) -> Self {
        self.0.water = water_movement;
        self
    }

    pub fn space_movement(mut self, space_movement: SpaceMovement) -> Self {
        self.0.space = space_movement;
        self
    }

    pub fn wheel_type(mut self, wheel_type: WheelType) -> Self {
        match self.0.ground {
            GroundMovement::Wheels(ref mut _wheel_type)
            | GroundMovement::Tracks(ref mut _wheel_type) => *_wheel_type = wheel_type,
            _ => {}
        }
        self
    }

    #[must_use]
    pub fn build(self) -> MovementAbility {
        self.0
    }
}
