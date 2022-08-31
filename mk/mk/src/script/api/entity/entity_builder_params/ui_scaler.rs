use super::EntityBuilderParam;
use crate::{component::UIScaleMode, structure::Size};
use rhai::EvalAltResult;

pub struct UIScalerParams {
    pub mode: UIScaleMode,
    pub reference_size: Size,
}

impl EntityBuilderParam for UIScalerParams {
    fn from_table(mut table: rhai::Map) -> Result<Self, Box<EvalAltResult>> {
        Ok(Self {
            mode: table
                .remove("mode")
                .ok_or_else(|| "the field 'mode' is not specified")?
                .try_cast()
                .ok_or_else(|| "the field 'mode' is not valid type")?,
            reference_size: table
                .remove("reference_size")
                .ok_or_else(|| "the field 'reference_size' is not specified")?
                .try_cast()
                .ok_or_else(|| "the field 'reference_size' is not valid type")?,
        })
    }
}
