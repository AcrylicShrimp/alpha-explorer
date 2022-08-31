use super::EntityBuilderParam;
use crate::ui::{UIAnchor, UIMargin};
use rhai::{EvalAltResult, INT};

pub struct UIElementParams {
    pub anchor: UIAnchor,
    pub margin: UIMargin,
    pub is_interactible: Option<bool>,
    pub order_index: u32,
}

impl EntityBuilderParam for UIElementParams {
    fn from_table(mut table: rhai::Map) -> Result<Self, Box<EvalAltResult>> {
        Ok(Self {
            anchor: table
                .remove("anchor")
                .ok_or_else(|| "the field 'anchor' is not specified")?
                .try_cast()
                .ok_or_else(|| "the field 'anchor' is not valid type")?,
            margin: table
                .remove("margin")
                .ok_or_else(|| "the field 'margin' is not specified")?
                .try_cast()
                .ok_or_else(|| "the field 'margin' is not valid type")?,
            is_interactible: table
                .remove("is_interactible")
                .map(|is_interactible| {
                    is_interactible
                        .try_cast()
                        .ok_or_else(|| "the field 'is_interactible' is not valid type")
                })
                .transpose()?,
            order_index: table
                .remove("order_index")
                .ok_or_else(|| "the field 'order_index' is not specified")?
                .try_cast::<INT>()
                .ok_or_else(|| "the field 'order_index' is not valid type")?
                as _,
        })
    }
}
