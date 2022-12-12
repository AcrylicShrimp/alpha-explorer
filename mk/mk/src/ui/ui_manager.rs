use super::UILayoutCalculator;
use crate::component::{Camera, Size, Transform, UIScaler};
use crate::engine::use_context;
use crate::structure::{Mat33Ref, Vec2, Vec3};
use crate::ui::{UIElement, UIScaleMode};
use specs::prelude::*;
use std::collections::btree_map::Entry;
use std::collections::BTreeMap;

#[derive(Debug)]
pub struct UIManager {
    elements: Vec<UIElement>,
    entities: Vec<Entity>,
    prev_order_indices: Vec<u32>,
    ordered_indices: BTreeMap<u32, Vec<u32>>,
    removed_indices: Vec<u32>,
    layout_calculator: UILayoutCalculator,
}

impl UIManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn begin_alloc(&mut self) -> u32 {
        if let Some(index) = self.removed_indices.pop() {
            self.elements[index as usize] = UIElement::default();
            self.prev_order_indices[index as usize] = 0;
            return index;
        }

        let index = self.elements.len() as u32;
        self.elements.push(UIElement::default());
        self.prev_order_indices.push(0);
        self.layout_calculator.push();
        index
    }

    pub fn fin_alloc(&mut self, index: u32, entity: Entity) {
        let index = index as usize;
        if index < self.entities.len() {
            self.entities[index] = entity;
        } else {
            self.entities.push(entity);
        }
    }

    pub fn dealloc(&mut self, index: u32) {
        if let Entry::Occupied(mut entry) = self
            .ordered_indices
            .entry(self.prev_order_indices[index as usize])
        {
            if let Some(index) = entry.get().iter().position(|&element| element == index) {
                entry.get_mut().swap_remove(index);
            }

            if entry.get().is_empty() {
                entry.remove_entry();
            }
        }

        self.elements[index as usize].reset_flags();
        self.removed_indices.push(index);
    }

    pub fn element(&self, index: u32) -> &UIElement {
        &self.elements[index as usize]
    }

    pub fn element_mut(&mut self, index: u32) -> &mut UIElement {
        &mut self.elements[index as usize]
    }

    pub fn entity(&self, index: u32) -> Entity {
        self.entities[index as usize]
    }

    pub fn raycast_element(
        &self,
        point_in_screen: Vec2,
        camera: Option<(&Transform, &Camera)>,
    ) -> Option<Entity> {
        let context = use_context();
        let world = context.world();
        let screen_mgr = context.screen_mgr();
        let transform_mgr = context.transform_mgr();

        let point_in_camera = Vec3::new(
            point_in_screen.x - screen_mgr.width() as f32 * 0.5f32,
            -point_in_screen.y + screen_mgr.height() as f32 * 0.5f32,
            1f32,
        );
        let camera_to_world = match camera {
            Some(camera) => {
                let camera_transform_index = camera.0.index();
                transform_mgr.transform_world_matrix(camera_transform_index)
            }
            None => Mat33Ref::identity(),
        };

        for (_, indices) in self.ordered_indices.iter().rev() {
            for &index in indices.iter().rev() {
                let element = &self.elements[index as usize];
                if !element.is_interactible() {
                    continue;
                }

                let entity = self.entities[index as usize];
                let transform_index =
                    if let Some(transform) = world.read_storage::<Transform>().get(entity) {
                        transform.index()
                    } else {
                        continue;
                    };
                let size = if let Some(size) = world.read_storage::<Size>().get(entity) {
                    size.size
                } else {
                    continue;
                };

                let world_to_local = transform_mgr
                    .transform_world_matrix(transform_index)
                    .inversed();
                let point_in_local = point_in_camera * camera_to_world * world_to_local;

                if 0f32 <= point_in_local.x
                    && point_in_local.x <= size.width
                    && -size.height <= point_in_local.y
                    && point_in_local.y <= 0f32
                {
                    return Some(self.entities[index as usize]);
                }
            }
        }

        None
    }

    pub fn update_elements(&mut self) {
        {
            let context = use_context();
            let screen_mgr = context.screen_mgr();

            if screen_mgr.is_dirty() {
                let elements = self.elements.as_mut_slice();
                let mut world = context.world_mut();
                let world = &mut *world;
                let mut transform_mgr = context.transform_mgr_mut();
                let transform_mgr = &mut *transform_mgr;

                for (scaler, transform, size, element) in (
                    &world.read_storage::<UIScaler>(),
                    &world.read_storage::<Transform>(),
                    &mut world.write_storage::<Size>(),
                    &mut world.write_storage::<crate::component::UIElement>(),
                )
                    .join()
                {
                    let new_size = match scaler.mode {
                        UIScaleMode::Constant => scaler.reference_size,
                        UIScaleMode::Stretch => crate::structure::Size::new(
                            screen_mgr.width() as f32,
                            screen_mgr.height() as f32,
                        ),
                        UIScaleMode::Fit => {
                            let screen_width = screen_mgr.width() as f32;
                            let screen_height = screen_mgr.height() as f32;
                            let scale = Vec2::new(
                                screen_width / scaler.reference_size.width,
                                screen_height / scaler.reference_size.height,
                            );
                            let scale = f32::min(scale.x, scale.y);
                            let new_size = scaler.reference_size * scale;
                            new_size
                        }
                        UIScaleMode::Fill => {
                            let screen_width = screen_mgr.width() as f32;
                            let screen_height = screen_mgr.height() as f32;
                            let scale = Vec2::new(
                                screen_width / scaler.reference_size.width,
                                screen_height / scaler.reference_size.height,
                            );
                            let scale = f32::max(scale.x, scale.y);
                            let new_size = scaler.reference_size * scale;
                            new_size
                        }
                        UIScaleMode::MatchWidth => {
                            let screen_width = screen_mgr.width() as f32;
                            let scale = screen_width / scaler.reference_size.width;
                            let new_size = scaler.reference_size * scale;
                            new_size
                        }
                        UIScaleMode::MatchHeight => {
                            let screen_height = screen_mgr.height() as f32;
                            let scale = screen_height / scaler.reference_size.height;
                            let new_size = scaler.reference_size * scale;
                            new_size
                        }
                    };
                    let transform_index = transform.index();
                    transform_mgr.hierarchy_mut().set_dirty(transform_index);

                    let transform = transform_mgr.allocator_mut().transform_mut(transform_index);
                    transform.position = Vec2::new(0f32, 0f32);
                    size.size = new_size;
                    elements[element.index() as usize].mark_as_dirty();
                }
            }
        }

        for (index, element) in self.elements.iter_mut().enumerate() {
            if !element.is_dirty() {
                continue;
            }

            if let Entry::Occupied(mut entry) =
                self.ordered_indices.entry(self.prev_order_indices[index])
            {
                if let Some(index) = entry
                    .get()
                    .iter()
                    .position(|&element| element == index as u32)
                {
                    entry.get_mut().swap_remove(index);
                }

                if entry.get().is_empty() {
                    entry.remove_entry();
                }
            }

            let order_index = element.order_index();
            self.ordered_indices
                .entry(order_index)
                .or_default()
                .push(index as u32);
            self.prev_order_indices[index] = order_index;
        }

        self.layout_calculator
            .calculate_all(&self.entities, &mut self.elements);

        for element in &mut self.elements {
            element.reset_dirty();
        }
    }
}

impl Default for UIManager {
    fn default() -> Self {
        Self {
            elements: Vec::with_capacity(1024),
            entities: Vec::with_capacity(1024),
            prev_order_indices: Vec::with_capacity(1024),
            ordered_indices: BTreeMap::new(),
            removed_indices: Vec::with_capacity(1024),
            layout_calculator: UILayoutCalculator::default(),
        }
    }
}
