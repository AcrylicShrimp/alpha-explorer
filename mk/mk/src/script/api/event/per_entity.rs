use legion::Entity;
use std::{any::Any, sync::Arc};

pub trait PerEntityParam
where
    Self: Any + Send + Sync,
{
}

impl<T> PerEntityParam for T where T: Any + Send + Sync {}

#[derive(Clone)]
pub struct PerEntity {
    pub entity: Entity,
    pub event: String,
    pub param: Arc<dyn PerEntityParam>,
}
