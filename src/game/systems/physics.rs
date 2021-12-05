use crate::consts::PHYSICS_SCALE;
use crate::dev::debug;
use crate::game::components::robot::*;
use crate::game::types::*;
use bevy::{log, prelude::*};
use bevy_prototype_debug_lines::*;
use bevy_rapier2d::na::Isometry2;
use bevy_rapier2d::physics::JointHandleComponent;
use bevy_rapier2d::prelude::*;
use bevy_rapier2d::rapier::na::Vector2;

pub fn reduce_sideways_vel(
    mut lines: ResMut<DebugLines>,
    _rapier_parameters: Res<RapierConfiguration>,
    mut player_info: Query<
        (
            &Motors,
            &mut RigidBodyForces,
            &mut RigidBodyVelocity,
            &RigidBodyPosition,
            &Transform,
            &AttachmentMap<Attachment>,
        ),
        Changed<RigidBodyVelocity>,
    >,
) {
    for (_drive, _forces, mut rb_vel, pos, trans, _attach) in player_info.iter_mut() {
        let dir = pos.position.transform_vector(&Vector2::y());
        let angle = dir.angle(&rb_vel.linvel);
        let projected = rb_vel.linvel.magnitude() * angle.cos();

        let res = dir * projected;

        let t = 0.5;
        let damped = res * t + rb_vel.linvel * (1. - t);

        debug::relative_line(&mut lines, trans, Vec2::new(damped.x, damped.y));
        debug::relative_line(&mut lines, trans, Vec2::new(dir.x, dir.y));

        rb_vel.linvel = damped;
    }
}

pub fn adjust_damping(damping: Query<(&Motors, &mut RigidBodyDamping), Changed<Motors>>) {
    damping.for_each_mut(|(driver, mut rb_damping)| {
        rb_damping.linear_damping = driver.linear_damping;
        rb_damping.angular_damping = driver.angular_damping;
    });
}

pub fn spawn_joints(
    mut commands: Commands,
    query: Query<(
        Entity,
        &WantToAttach,
        &ItemSize,
        &ItemType,
        &ItemOrigin,
        &JointType,
    )>,
    mut tag_queries: QuerySet<(
        Query<(&ParentEntity, &Transform, &RigidBodyPosition)>,
        Query<(
            &mut ParentEntity,
            &mut Transform,
            &mut RigidBodyPosition,
            &mut Visible,
        )>,
    )>,
    mut query_p: Query<&mut AttachmentMap<Attachment>>,
) {
    let mut parent_tags = vec![];
    query.for_each_mut(
        |(entity, want_attach, item_size, item_type, item_origin, joint_type)| match want_attach {
            WantToAttach::To {
                parent: Some(parent_entity),
                aid,
            } => {
                if let Ok(mut attachments) = query_p.get_mut(*parent_entity) {
                    if let Ok((parent_tag, parent_transform, rb_pos)) =
                        tag_queries.q0().get(*parent_entity)
                    {
                        if let Some(at) = attachments.0.get_mut(&aid) {
                            if at.is_compatible(item_size, item_type)
                                && parent_tag != &ParentEntity::WaitForAttach
                            {
                                parent_tags.push((
                                    entity,
                                    (
                                        Some(*parent_tag),
                                        parent_transform.translation.z + at.transform.translation.z,
                                        rb_pos.position
                                            * Isometry2::new(
                                                (at.transform.translation.truncate()
                                                    / PHYSICS_SCALE)
                                                    .into(),
                                                at.transform.rotation.to_axis_angle().1,
                                            )
                                            * Isometry2::translation(
                                                -item_origin.0 / PHYSICS_SCALE,
                                                -item_origin.1 / PHYSICS_SCALE,
                                            ),
                                    ),
                                ));
                                let joint = match joint_type {
                                    JointType::Ball => {
                                        let mut ball = BallJoint::new(
                                            (at.transform.translation.truncate() / PHYSICS_SCALE)
                                                .into(),
                                            (item_origin.to_vec2() / PHYSICS_SCALE).into(),
                                        );
                                        ball.configure_motor_velocity(0.0, 0.2);
                                        JointParams::BallJoint(ball)
                                    }
                                    JointType::Fixed => JointParams::FixedJoint(FixedJoint::new(
                                        (
                                            (at.transform.translation.truncate() / PHYSICS_SCALE),
                                            at.transform.rotation.to_axis_angle().1,
                                        )
                                            .into(),
                                        (item_origin.to_vec2() / PHYSICS_SCALE).into(),
                                    )),
                                    JointType::Prismatic => {
                                        let prismatic = PrismaticJoint::new(
                                            (at.transform.translation.truncate() / PHYSICS_SCALE)
                                                .into(),
                                            Vector::y_axis(),
                                            (item_origin.to_vec2() / PHYSICS_SCALE).into(),
                                            Vector::y_axis(),
                                        );
                                        JointParams::PrismaticJoint(prismatic)
                                    }
                                    _ => {
                                        unimplemented!()
                                    }
                                };

                                let joint_entity = commands
                                    .entity(entity)
                                    .insert(JointBuilderComponent::new(
                                        joint,
                                        *parent_entity,
                                        entity,
                                    ))
                                    .id();
                                at.attach(entity, joint_entity);
                            } else {
                                // log::info!("FAILED TO ATTACH: {}", aid);
                            }
                        } else {
                            // log::info!("FAILED TO ATTACH: {}", aid);
                        }
                    }
                }
            }
            WantToAttach::Me => {
                parent_tags.push((
                    entity,
                    (
                        None,
                        90.0,
                        Isometry2::translation(item_origin.0, item_origin.1),
                    ),
                ));
            }
            _ => {
                // log::info!("FAILED TO ATTACH: {:?}", entity);
            }
        },
    );

    parent_tags
        .iter()
        .for_each(|(child_entity, (parent_tag, z, child_transform))| {
            if let Ok((mut child_tag, mut transform, mut rb_pos, mut visible)) =
                tag_queries.q1_mut().get_mut(*child_entity)
            {
                visible.is_visible = true;
                *child_tag = parent_tag.unwrap_or(ParentEntity::Robot(Some(*child_entity)));
                transform.translation.z = *z;
                log::info!("ATTACH AT ROTATION: {:?}", child_transform.rotation.angle());
                *rb_pos = (*child_transform).into();
                commands.entity(*child_entity).remove::<WantToAttach>();
            }
        });
}

pub fn set_collision_for_item_types(
    query: Query<
        (
            &ItemType,
            &mut ColliderType,
            &mut ColliderShape,
            &mut Transform,
            &mut ColliderMassProps,
            &mut TextureAtlasSprite,
        ),
        Changed<ParentEntity>,
    >,
) {
    query.for_each_mut(
        |(
            item_type,
            mut collider_type,
            mut collider_shape,
            mut transform,
            mut mass,
            mut sprite,
        )| {
            match item_type {
                ItemType::Robot(RobotItemType::CameraLens(CameraLensType::Telephoto {
                    focal_lengths,
                    ..
                }))
                 => {
                    let fl = focal_lengths.1 - focal_lengths.0;
                    *collider_type = ColliderType::Sensor;
                    *collider_shape =
                        ColliderShape::cuboid(fl / (2. * PHYSICS_SCALE), fl / (2. * PHYSICS_SCALE));
                    transform.scale = Vec3::new(fl, fl, 1.);
                    *mass = ColliderMassProps::Density(0.0001);
                    sprite.color = Color::rgba(0.0, 0.2, 1.0, 0.1);
                }
                ItemType::Robot(RobotItemType::CameraLens(CameraLensType::Wide {
                    focal_length,
                    ..
                })) => {
                    let fl = *focal_length;
                    *collider_type = ColliderType::Sensor;
                    *collider_shape =
                        ColliderShape::cuboid(fl / (2. * PHYSICS_SCALE), fl / (2. * PHYSICS_SCALE));
                    transform.scale = Vec3::new(fl, fl, 1.);
                    *mass = ColliderMassProps::Density(0.0001);
                    sprite.color = Color::rgba(0.0, 0.2, 1.0, 0.1);
                }
                _ => {
                    *collider_type = ColliderType::Solid;
                }
            }
        },
    );
}

pub fn despawn_detached_items(
    mut commands: Commands,
    query: Query<(Entity, &JointHandleComponent)>,
    query_all: Query<Entity>,
) {
    query.for_each(|(entity, joints)| {
        match (
            query_all.get(joints.entity1()),
            query_all.get(joints.entity2()),
        ) {
            (Err(..), ..) => commands.entity(entity).despawn_recursive(),
            _ => {}
        }
    });
}
