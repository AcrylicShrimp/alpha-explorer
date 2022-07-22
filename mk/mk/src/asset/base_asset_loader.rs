use downcast_rs::{impl_downcast, Downcast};
use std::any::TypeId;

pub trait BaseAssetLoader
where
    Self: Downcast,
{
    fn asset_type_id(&self) -> TypeId;
}

impl_downcast!(BaseAssetLoader);
