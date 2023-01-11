// use super::AbstractTypedEventBus;
// use parking_lot::Mutex;
// use specs::prelude::*;
// use std::{
//     any::{Any, TypeId},
//     collections::HashMap,
//     hash::Hash,
//     sync::Arc,
// };

// pub struct EntityEventManager {
//     per_entity: HashMap<EntityEventKey, Mutex<Vec<_>>>,
// }

// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// struct EntityEventKey(Entity, TypeId);

// impl EntityEventKey {
//     pub fn new<T>(entity: Entity) -> Self
//     where
//         T: Any,
//     {
//         return Self(entity, TypeId::of::<T>());
//     }

//     pub fn entity(self) -> Entity {
//         self.0
//     }

//     pub fn type_id(self) -> TypeId {
//         self.1
//     }
// }

// impl Hash for EntityEventKey {
//     fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
//         self.0.hash(state);
//         self.1.hash(state);
//     }
// }
