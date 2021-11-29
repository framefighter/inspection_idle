use crate::consts::PHYSICS_SCALE;
use crate::dev::debug;
use crate::game::components::collision_filter::CollisionFilter;
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
            &Attachments,
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
    query: Query<(Entity, &WantToAttach, &ItemSize, &ItemType)>,
    mut tag_queries: QuerySet<(Query<&CollisionFilter>, Query<&mut CollisionFilter>)>,
    mut query_p: Query<&mut Attachments>,
) {
    let mut parent_tags = vec![];
    query.for_each_mut(
        |(entity, want_attach, item_size, item_type)| match want_attach {
            WantToAttach::To {
                parent: Some(parent_entity),
                aid,
            } => {
                if let Ok(mut attachments) = query_p.get_mut(*parent_entity) {
                    if let Ok(parent_tag) = tag_queries.q0().get(*parent_entity) {
                        if let Some(at) = attachments.0.get_mut(&aid) {
                            if at.is_compatible(item_size, item_type)
                                && parent_tag != &CollisionFilter::WaitForAttach
                            {
                                log::info!("ATTACHED: {}", aid);
                                parent_tags.push((entity, Some(*parent_tag)));

                                let joint = FixedJoint::new(
                                    (at.transform.translation / PHYSICS_SCALE).into(),
                                    Isometry::identity(),
                                );

                                let joint_entity = commands
                                    .spawn()
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
                parent_tags.push((entity, None));
            }
            _ => {
                log::info!("FAILED TO ATTACH: {:?}", entity);
            }
        },
    );

    parent_tags.iter().for_each(|(child_entity, parent_tag)| {
        if let Ok(mut child_tag) = tag_queries.q1_mut().get_mut(*child_entity) {
            *child_tag = parent_tag.unwrap_or(CollisionFilter::Robot(child_entity.id()));
            commands.entity(*child_entity).remove::<WantToAttach>();
        }
    });
}