use bevy::prelude::Query;
use bevy_rapier2d::{physics::PhysicsHooksWithQuery, prelude::*};

use super::robot::ParentEntity;

pub struct RapierUserData;

impl<'a> PhysicsHooksWithQuery<&'a ParentEntity> for RapierUserData {
    fn filter_contact_pair(
        &self,
        context: &PairFilterContext<RigidBodyComponentsSet, ColliderComponentsSet>,
        tags: &Query<&'a ParentEntity>,
    ) -> Option<SolverFlags> {
        match (
            tags.get(context.collider1.entity()),
            tags.get(context.collider2.entity()),
        ) {
            (Ok(ParentEntity::WaitForAttach), ..) | (.., Ok(ParentEntity::WaitForAttach)) => {
                None
            }
            (Ok(a), Ok(b)) if a == b => None,
            _ => Some(SolverFlags::default()),
        }
    }
}