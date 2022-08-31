use super::EntityBuilderParam;
use crate::render::Layer;
use rhai::{EvalAltResult, INT};

pub struct CameraParams {
    pub layer: Layer,
    pub order: isize,
}

impl EntityBuilderParam for CameraParams {
    fn from_table(mut table: rhai::Map) -> Result<Self, Box<EvalAltResult>> {
        Ok(Self {
            layer: table
                .remove("layer")
                .ok_or_else(|| "the field 'layer' is not specified")?
                .try_cast()
                .ok_or_else(|| "the field 'layer' is not valid type")?,
            order: table
                .remove("order")
                .ok_or_else(|| "the field 'order' is not specified")?
                .try_cast::<INT>()
                .ok_or_else(|| "the field 'order' is not valid type")? as _,
        })
    }
}
