use rhai::{Dynamic, EvalAltResult, Module, FLOAT, INT};

macro_rules! to_global {
    ($module:ident, $f:expr) => {{
        let hash = $f;
        $module.update_fn_namespace(hash, rhai::FnNamespace::Global);
    }};
}

#[allow(dead_code)]
fn extract_int(dynamic: Dynamic) -> Result<INT, Box<EvalAltResult>> {
    if dynamic.is::<INT>() {
        Ok(dynamic.as_int().unwrap())
    } else if dynamic.is::<FLOAT>() {
        Ok(dynamic.as_float().unwrap() as INT)
    } else {
        Err(format!("expected i64 or f32, got {}", dynamic.type_name()).into())
    }
}

#[allow(dead_code)]
fn extract_float(dynamic: Dynamic) -> Result<FLOAT, Box<EvalAltResult>> {
    if dynamic.is::<INT>() {
        Ok(dynamic.as_int().unwrap() as FLOAT)
    } else if dynamic.is::<FLOAT>() {
        Ok(dynamic.as_float().unwrap())
    } else {
        Err(format!("expected i64 or f32, got {}", dynamic.type_name()).into())
    }
}

trait ModuleType {
    fn register(module: &mut Module);
}

trait OptionToDynamic {
    fn to_dynamic(self) -> Dynamic;
}

impl<T> OptionToDynamic for Option<T>
where
    T: 'static + Clone + Send + Sync,
{
    fn to_dynamic(self) -> Dynamic {
        match self {
            Some(value) => Dynamic::from(value),
            None => Dynamic::from(()),
        }
    }
}

trait DynamicToOption {
    fn to_option<T>(self) -> Option<T>
    where
        T: 'static + Clone + Send + Sync;
}

impl DynamicToOption for Dynamic {
    fn to_option<T>(self) -> Option<T>
    where
        T: 'static + Clone + Send + Sync,
    {
        if self.is::<()>() {
            None
        } else {
            Some(self.cast())
        }
    }
}

pub mod asset;
pub mod audio;
pub mod component;
pub mod entity;
pub mod event;
pub mod render;
pub mod screen;
pub mod structure;
pub mod time;
pub mod ui;

pub fn build_module() -> Module {
    let mut module = Module::new();
    module.set_id("__builtin__");

    asset::AssetModule::register(&mut module);
    audio::AudioModule::register(&mut module);
    component::ComponentModule::register(&mut module);
    entity::EntityModule::register(&mut module);
    event::EventModule::register(&mut module);
    render::RenderModule::register(&mut module);
    screen::ScreenModule::register(&mut module);
    structure::StructureModule::register(&mut module);
    time::TimeModule::register(&mut module);
    ui::UIModule::register(&mut module);

    module
}
