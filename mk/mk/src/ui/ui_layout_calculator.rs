use crate::api::use_context;
use crate::component::Transform;
use crate::structure::{Size, Vec2};
use crate::transform::TransformManager;
use crate::ui::UIElement;
use bitvec::prelude::*;
use legion::*;

struct UIElementPair {
    pub parent: u32,
    pub child: u32,
}

#[derive(Default, Debug)]
pub struct UILayoutCalculator {
    pair_flags: BitVec,
}

impl UILayoutCalculator {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self) {
        self.pair_flags.push(false);
    }

    pub fn calculate_all(&mut self, entities: &[Entity], elements: &mut [UIElement]) {
        self.pair_flags.set_all(false);

        let context = use_context();
        let mut pairs = Vec::with_capacity(elements.len());

        let mut world = context.world_mut();
        let world = &mut *world;
        let mut transform_mgr = context.transform_mgr_mut();
        let transform_mgr = &mut *transform_mgr;

        for index in 0..elements.len() {
            self.collect_all_pairs(
                index as u32,
                world,
                transform_mgr,
                entities,
                elements,
                &mut pairs,
            );
        }

        for pair in pairs {
            calculate_pair(world, transform_mgr, entities, elements, pair);
        }
    }

    fn collect_all_pairs(
        &mut self,
        index: u32,
        world: &World,
        transform_mgr: &TransformManager,
        entities: &[Entity],
        elements: &mut [UIElement],
        pairs: &mut Vec<UIElementPair>,
    ) -> bool {
        if self.pair_flags[index as usize] {
            return elements[index as usize].is_dirty();
        }
        self.pair_flags.set(index as usize, true);

        let transform = {
            let entry = if let Ok(entry) = world.entry_ref(entities[index as usize]) {
                entry
            } else {
                return elements[index as usize].is_dirty();
            };
            if let Ok(transform) = entry.get_component::<Transform>() {
                transform_mgr.transform(transform.index())
            } else {
                return elements[index as usize].is_dirty();
            }
        };
        let parent_index = {
            let parent_index = if let Some(parent_index) = transform.parent_index() {
                parent_index
            } else {
                return elements[index as usize].is_dirty();
            };
            let entry = if let Ok(entry) = world.entry_ref(transform_mgr.entity(parent_index)) {
                entry
            } else {
                return elements[index as usize].is_dirty();
            };
            if let Ok(element) = entry.get_component::<crate::component::UIElement>() {
                element.index()
            } else {
                return elements[index as usize].is_dirty();
            }
        };

        let parent_dirty = self.collect_all_pairs(
            parent_index,
            world,
            transform_mgr,
            entities,
            elements,
            pairs,
        );
        let dirty = parent_dirty || elements[index as usize].is_dirty();

        if dirty {
            elements[index as usize].mark_as_dirty();
            pairs.push(UIElementPair {
                parent: parent_index,
                child: index,
            });
        }

        dirty
    }
}

fn calculate_pair(
    world: &mut World,
    transform_mgr: &mut TransformManager,
    entities: &[Entity],
    elements: &mut [UIElement],
    pair: UIElementPair,
) {
    let parent_size = {
        let entry = if let Ok(entry) = world.entry_ref(entities[pair.parent as usize]) {
            entry
        } else {
            return;
        };
        if let Ok(size) = entry.get_component::<crate::component::Size>() {
            Size::new(size.width, size.height)
        } else {
            return;
        }
    };
    let child = &mut elements[pair.child as usize];

    let margin_left = parent_size.width * (child.anchor.min.x - 0.5f32);
    let margin_top = parent_size.height * (0.5f32 - child.anchor.min.y);
    let margin_right = parent_size.width * (child.anchor.max.x - 0.5f32);
    let margin_bottom = parent_size.height * (0.5f32 - child.anchor.max.y);

    let mut entry = if let Ok(entry) = world.entry_mut(entities[pair.child as usize]) {
        entry
    } else {
        return;
    };
    let transform = if let Ok(transform) = entry.get_component::<Transform>() {
        transform_mgr.transform_mut(transform.index())
    } else {
        return;
    };
    let size = if let Ok(size) = entry.get_component_mut::<crate::component::Size>() {
        size
    } else {
        return;
    };
    let width = margin_right - margin_left - child.margin.left - child.margin.right;
    let height = margin_top - margin_bottom - child.margin.bottom - child.margin.top;
    size.width = width;
    size.height = height;
    transform.mark_as_dirty();
    transform.position = Vec2::new(
        margin_left + child.margin.left + width * 0.5f32,
        margin_top - child.margin.top - height * 0.5f32,
    );
}
