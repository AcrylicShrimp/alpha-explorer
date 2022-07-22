use super::UILayoutCalculator;
use crate::api::use_context;
use crate::component::{Camera, Size, Transform, UIScaleMode, UIScaler};
use crate::structure::Vec2;
use crate::ui::UIElement;
use legion::{Entity, EntityStore, IntoQuery};
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

    pub fn alloc(&mut self, entity: Entity) -> u32 {
        if let Some(index) = self.removed_indices.pop() {
            self.elements[index as usize] = UIElement::default();
            self.entities[index as usize] = entity;
            self.prev_order_indices[index as usize] = 0;
            return index;
        }

        let index = self.elements.len() as u32;
        self.elements.push(UIElement::default());
        self.entities.push(entity);
        self.prev_order_indices.push(0);
        self.layout_calculator.push();
        index
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
        x: f32,
        y: f32,
        camera: Option<(&Transform, &Camera)>,
    ) -> Option<Entity> {
        let context = use_context();
        let world = context.world();
        let screen_mgr = context.screen_mgr();
        let transform_mgr = context.transform_mgr();

        let camera_x = x - screen_mgr.width() as f32 * 0.5f32;
        let camera_y = -y + screen_mgr.height() as f32 * 0.5f32;

        // TODO: Capsulate below matrix calculations.

        let camera_to_world = match camera {
            Some(camera) => {
                let camera_transform_index = camera.0.index();
                transform_mgr
                    .transform_world_matrix(camera_transform_index)
                    .clone()
            }
            None => [
                1.0f32, 0.0f32, 0.0f32, 0.0f32, 1.0f32, 0.0f32, 0.0f32, 0.0f32, 1.0f32,
            ],
        };

        for (_, indices) in self.ordered_indices.iter().rev() {
            for &index in indices.iter().rev() {
                let element = &self.elements[index as usize];
                if !element.is_interactible() {
                    continue;
                }

                let entry = match world.entry_ref(self.entities[index as usize]) {
                    Ok(entry) => entry,
                    Err(_) => continue,
                };
                let transform = match entry.get_component::<Transform>() {
                    Ok(transform) => transform,
                    Err(_) => continue,
                };
                let size = match entry.get_component::<Size>() {
                    Ok(size) => size,
                    Err(_) => continue,
                };
                let transform = transform_mgr.transform(transform.index());
                let mut world_to_local = [0f32; 9];
                let mut camera_to_local = [0f32; 6];

                transform.to_matrix_inverse(&mut world_to_local);
                camera_to_local[0] = camera_to_world[0] * world_to_local[0]
                    + camera_to_world[1] * world_to_local[3]
                    + camera_to_world[2] * world_to_local[6];
                camera_to_local[1] = camera_to_world[0] * world_to_local[1]
                    + camera_to_world[1] * world_to_local[4]
                    + camera_to_world[2] * world_to_local[7];
                camera_to_local[2] = camera_to_world[3] * world_to_local[0]
                    + camera_to_world[4] * world_to_local[3]
                    + camera_to_world[5] * world_to_local[6];
                camera_to_local[3] = camera_to_world[3] * world_to_local[1]
                    + camera_to_world[4] * world_to_local[4]
                    + camera_to_world[5] * world_to_local[7];
                camera_to_local[4] = camera_to_world[6] * world_to_local[0]
                    + camera_to_world[7] * world_to_local[3]
                    + camera_to_world[8] * world_to_local[6];
                camera_to_local[5] = camera_to_world[6] * world_to_local[1]
                    + camera_to_world[7] * world_to_local[4]
                    + camera_to_world[8] * world_to_local[7];

                let local_x = camera_x * camera_to_local[0]
                    + camera_y * camera_to_local[2]
                    + camera_to_local[4];
                let local_y = camera_x * camera_to_local[1]
                    + camera_y * camera_to_local[3]
                    + camera_to_local[5];

                if 0f32 <= local_x
                    && local_x <= size.width
                    && -size.height <= local_y
                    && local_y <= 0f32
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
                <(
                    &UIScaler,
                    &Transform,
                    &mut Size,
                    &mut crate::component::UIElement,
                )>::query()
                .iter_mut(world)
                .for_each(move |(scaler, transform, size, element)| {
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
                    let transform = transform_mgr.transform_mut(transform.index());
                    transform.mark_as_dirty();
                    transform.position = Vec2::new(0f32, 0f32);
                    size.width = new_size.width;
                    size.height = new_size.height;
                    elements[element.index() as usize].mark_as_dirty()
                });
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
