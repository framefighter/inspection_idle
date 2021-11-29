use bevy::prelude::Query;
use bevy_inspector_egui::Inspectable;
use bevy_rapier2d::{physics::PhysicsHooksWithQuery, prelude::*};

#[derive(PartialEq, Eq, Clone, Inspectable, Debug, Copy)]
pub enum CollisionFilter {
    WaitForAttach,
    None,
    Robot(u32),
}

impl Default for CollisionFilter {
    fn default() -> Self {
        Self::None
    }
}

pub struct SameUserDataFilter;

impl<'a> PhysicsHooksWithQuery<&'a CollisionFilter> for SameUserDataFilter {
    fn filter_contact_pair(
        &self,
        context: &PairFilterContext<RigidBodyComponentsSet, ColliderComponentsSet>,
        tags: &Query<&'a CollisionFilter>,
    ) -> Option<SolverFlags> {
        match (
            tags.get(context.collider1.entity()),
            tags.get(context.collider2.entity()),
        ) {
            (Ok(CollisionFilter::WaitForAttach), ..) | (.., Ok(CollisionFilter::WaitForAttach)) => {
                None
            }
            (Ok(a), Ok(b)) if a == b => None,
            _ => Some(SolverFlags::default()),
        }
    }
}