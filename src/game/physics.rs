use bevy::{prelude::*};
use bevy_rapier2d::{
    na::{Isometry2, Vector2},
    prelude::*,
};
use nalgebra::Point2;

#[derive(Default)]
pub struct FirstBlock;

pub fn enable_physics_profiling(mut pipeline: ResMut<PhysicsPipeline>) {
    pipeline.counters.enable()
}

pub fn setup_graphics(mut commands: Commands, mut configuration: ResMut<RapierConfiguration>) {
    configuration.scale = 12.0;

    let mut camera = OrthographicCameraBundle::new_2d();
    camera.transform = Transform::from_translation(Vec3::new(200.0, -200.0, 0.0));
    commands.spawn_bundle(LightBundle {
        transform: Transform::from_translation(Vec3::new(1000.0, 10.0, 2000.0)),
        light: Light {
            intensity: 100_000_000_.0,
            range: 6000.0,
            ..Default::default()
        },
        ..Default::default()
    });
    commands.spawn_bundle(camera);
}

pub fn setup_physics(mut commands: Commands) {
    let rad = 0.4;
    let numi = 30; // Num vertical nodes.
    let shift = 5.0;

    let mut body_entities = Vec::new();
    let mut color = 0;

    for i in 0..numi {
        let fi = i as f32;
        color += 1;

        let body_type = if i == 0 {
            RigidBodyType::KinematicPositionBased
        } else {
            RigidBodyType::Dynamic
        };

        let rigid_body = RigidBodyBundle {
            body_type,
            position: [-10.0, -fi * shift].into(),
            ..RigidBodyBundle::default()
        };

        let collider = ColliderBundle {
            shape: ColliderShape::cuboid(rad, rad * 6.0),
            ..ColliderBundle::default()
        };
        let child_entity = commands
            .spawn_bundle(rigid_body)
            .insert_bundle(collider)
            .insert(ColliderPositionSync::Discrete)
            .insert(ColliderDebugRender::with_id(color))
            .id();

        if i == 0 {
            commands.entity(child_entity).insert(FirstBlock);
        }

        // Vertical joint.
        if i > 0 {
            let parent_entity = *body_entities.last().unwrap();
            let joint = BallJoint::new(Point2::origin(), Point2::from([0.0, -shift]));
            commands.spawn_bundle((JointBuilderComponent::new(
                joint,
                parent_entity,
                child_entity,
            ),));
        }

        body_entities.push(child_entity);
    }
}

pub fn move_block(
    query: Query<&mut RigidBodyPosition, With<FirstBlock>>,
    mut mouse_moved_events: EventReader<CursorMoved>,
) {
    for event in mouse_moved_events.iter() {
        query.for_each_mut(|mut body| {
            body.next_position = Isometry2::new(
                Vector2::from([event.position.x / 30.0, event.position.y / 30.0]),
                body.next_position.rotation.angle(),
            );
        });
    }
}
