use crate::consts::PHYSICS_SCALE;
use crate::dev::debug;
use crate::game::components::robot::*;
use bevy::{log, prelude::*};
use bevy_prototype_debug_lines::*;
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
    query: Query<(Entity, &WantToAttach, &ItemSize, &ItemType, &JointType)>,
    mut tag_queries: QuerySet<(
        Query<(&ParentEntity, &Transform)>,
        Query<(&mut ParentEntity, &mut Transform)>,
    )>,
    mut query_p: Query<&mut AttachmentMap<Attachment>>,
) {
    let mut parent_tags = vec![];
    query.for_each_mut(
        |(entity, want_attach, item_size, item_type, joint_type)| match want_attach {
            WantToAttach::To {
                parent: Some(parent_entity),
                aid,
            } => {
                if let Ok(mut attachments) = query_p.get_mut(*parent_entity) {
                    if let Ok((parent_tag, parent_transform)) = tag_queries.q0().get(*parent_entity)
                    {
                        if let Some(at) = attachments.0.get_mut(&aid) {
                            if at.is_compatible(item_size, item_type)
                                && parent_tag != &ParentEntity::WaitForAttach
                            {
                                log::info!("Spawned Joint: {}", aid);
                                parent_tags.push((
                                    entity,
                                    (
                                        Some(*parent_tag),
                                        parent_transform.translation.z + at.transform.translation.z,
                                    ),
                                ));
                                let joint = match joint_type {
                                    JointType::Ball => {
                                        let mut ball = BallJoint::new(
                                            (at.transform.translation.truncate() / PHYSICS_SCALE)
                                                .into(),
                                            Vec2::default().into(),
                                        );
                                        ball.configure_motor_velocity(0.0, 0.2);
                                        JointParams::BallJoint(ball)
                                    }
                                    JointType::Fixed => JointParams::FixedJoint(FixedJoint::new(
                                        (at.transform.translation / PHYSICS_SCALE).into(),
                                        Isometry::identity(),
                                    )),
                                    JointType::Prismatic => {
                                        let prismatic = PrismaticJoint::new(
                                            (at.transform.translation.truncate() / PHYSICS_SCALE)
                                                .into(),
                                            Vector::y_axis(),
                                            Vec2::default().into(),
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
                                log::info!("FAILED TO ATTACH: {}", aid);
                            }
                        } else {
                            log::info!("FAILED TO ATTACH: {}", aid);
                        }
                    }
                }
            }
            WantToAttach::Me => {
                parent_tags.push((entity, (None, 90.)));
            }
            _ => {
                log::info!("FAILED TO ATTACH: {:?}", entity);
            }
        },
    );

    parent_tags
        .iter()
        .for_each(|(child_entity, (parent_tag, parent_height))| {
            if let Ok((mut child_tag, mut transform)) = tag_queries.q1_mut().get_mut(*child_entity)
            {
                *child_tag = parent_tag.unwrap_or(ParentEntity::Robot(Some(*child_entity)));
                transform.translation.z = *parent_height;
                commands.entity(*child_entity).remove::<WantToAttach>();
            }
        });
}

pub fn set_collision_for_item_types(
    query: Query<(&ItemType, &mut ColliderType), Changed<ParentEntity>>,
) {
    query.for_each_mut(|(item_type, mut collider_type)| {
        match item_type {
            ItemType::Robot(RobotItemType::CameraLens { .. }) => {
                *collider_type = ColliderType::Sensor;
            }
            _ => {
                *collider_type = ColliderType::Solid;
            }
        }
        log::info!(
            "Set Collider Type of {:?} to {:?}",
            item_type,
            collider_type
        );
    });
}
