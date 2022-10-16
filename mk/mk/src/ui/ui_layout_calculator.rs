use crate::component::*;
use crate::engine::use_context;
use crate::transform::TransformManager;
use bitvec::prelude::*;
use specs::prelude::*;

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

    pub fn calculate_all(&mut self, entities: &[Entity], elements: &mut [crate::ui::UIElement]) {
        self.pair_flags.fill(false);

        let context = use_context();
        let mut pairs = Vec::with_capacity(elements.len());

        let mut world = context.world_mut();
        let world = &mut *world;
        let mut transform_storage = world.write_storage::<Transform>();
        let mut size_storage = world.write_storage::<Size>();
        let element_storage = world.read_storage::<UIElement>();
        let mut transform_mgr = context.transform_mgr_mut();
        let transform_mgr = &mut *transform_mgr;

        for index in 0..elements.len() {
            self.collect_all_pairs(
                &mut transform_storage,
                &element_storage,
                index as u32,
                transform_mgr,
                entities,
                elements,
                &mut pairs,
            );
        }

        for pair in pairs {
            calculate_pair(
                &mut transform_storage,
                &mut size_storage,
                transform_mgr,
                entities,
                elements,
                pair,
            );
        }
    }

    fn collect_all_pairs<'a>(
        &mut self,
        transform_storage: &WriteStorage<'a, Transform>,
        element_storage: &ReadStorage<'a, UIElement>,
        index: u32,
        transform_mgr: &TransformManager,
        entities: &[Entity],
        elements: &mut [crate::ui::UIElement],
        pairs: &mut Vec<UIElementPair>,
    ) -> bool {
        if self.pair_flags[index as usize] {
            return elements[index as usize].is_dirty();
        }
        self.pair_flags.set(index as usize, true);

        let transform = if let Some(transform) = transform_storage.get(entities[index as usize]) {
            transform_mgr.transform(transform.index())
        } else {
            return elements[index as usize].is_dirty();
        };
        let parent_index = {
            let parent_index = if let Some(parent_index) = transform.parent_index() {
                parent_index
            } else {
                return elements[index as usize].is_dirty();
            };

            if let Some(ui_element) = element_storage.get(transform_mgr.entity(parent_index)) {
                ui_element.index()
            } else {
                return elements[index as usize].is_dirty();
            }
        };

        let parent_dirty = self.collect_all_pairs(
            transform_storage,
            element_storage,
            parent_index,
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

fn calculate_pair<'a>(
    transform_storage: &mut WriteStorage<'a, Transform>,
    size_storage: &mut WriteStorage<'a, Size>,
    transform_mgr: &mut TransformManager,
    entities: &[Entity],
    elements: &mut [crate::ui::UIElement],
    pair: UIElementPair,
) {
    let parent_size = if let Some(size) = size_storage.get(entities[pair.parent as usize]) {
        size.size
    } else {
        return;
    };
    let child = &mut elements[pair.child as usize];

    let margin_left = parent_size.width * (child.anchor.min.x - 0.5f32);
    let margin_bottom = parent_size.height * (child.anchor.min.y - 0.5f32);
    let margin_right = parent_size.width * (child.anchor.max.x - 0.5f32);
    let margin_top = parent_size.height * (child.anchor.max.y - 0.5f32);

    let entity = entities[pair.child as usize];
    let transform = if let Some(transform) = transform_storage.get_mut(entity) {
        transform_mgr.transform_mut(transform.index())
    } else {
        return;
    };
    let size = if let Some(size) = size_storage.get_mut(entity) {
        size
    } else {
        return;
    };
    let width = margin_right - margin_left - child.margin.left - child.margin.right;
    let height = margin_top - margin_bottom - child.margin.bottom - child.margin.top;
    size.size = crate::structure::Size::new(width, height);
    transform.mark_as_dirty();
    transform.position = crate::structure::Vec2::new(
        margin_left + child.margin.left,
        margin_bottom + child.margin.bottom,
    );
}
