use amethyst::renderer::DebugLines;
use amethyst::{core::transform::ParentHierarchy, core::Transform, ecs::*};
use std::f32;

use crate::components::combat::Health;

pub struct DeathByHealthSystem;

// Entities die if their health reaches zero (or less).
impl<'s> System<'s> for DeathByHealthSystem {
    type SystemData = (ReadStorage<'s, Health>, Entities<'s>);

    fn run(&mut self, (healths, entities): Self::SystemData) {
        for (health, entity) in (&healths, &*entities).join() {
            if health.value < f32::EPSILON {
                let _ = entities.delete(entity);
            }
        }
    }
}

#[derive(Default)]
pub struct DebugHealthSystem {}

impl<'s> System<'s> for DebugHealthSystem {
    type SystemData = (
        Entities<'s>,
        ReadExpect<'s, ParentHierarchy>,
        ReadStorage<'s, Health>,
        ReadStorage<'s, Transform>,
        Write<'s, DebugLines>,
    );

    fn run(&mut self, (entities, hierarchy, healths, locals, mut debug_lines): Self::SystemData) {
        for (entity, health, local) in (&entities, &healths, &locals).join() {
            let pos = match hierarchy.parent(entity) {
                Some(parent_entity) => {
                    let parent_transform = locals.get(parent_entity).unwrap();
                    parent_transform.clone().concat(local).translation().clone()
                }
                None => local.translation().clone(),
            };
            debug_lines.draw_line(
                [pos.x, pos.y + 0.5, 0.0].into(),
                [pos.x + health.value / 100.0, pos.y + 0.5, 0.0].into(),
                [0.0, 1.0, 0.0, 1.0].into(),
            )
        }
    }
}
