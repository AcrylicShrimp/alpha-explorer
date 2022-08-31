use crate::script::api::ModuleType;

#[derive(Debug, Clone, Copy)]
pub struct PreUpdate {
    pub dt: f64,
}

impl ModuleType for PreUpdate {
    fn register(module: &mut rhai::Module) {
        impl_event_type!(module, PreUpdate);

        module.set_getter_fn("dt", |this: &mut Self| Ok(this.dt));
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Update {
    pub dt: f64,
}

impl ModuleType for Update {
    fn register(module: &mut rhai::Module) {
        impl_event_type!(module, Update);

        module.set_getter_fn("dt", |this: &mut Self| Ok(this.dt));
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PostUpdate {
    pub dt: f64,
}

impl ModuleType for PostUpdate {
    fn register(module: &mut rhai::Module) {
        impl_event_type!(module, PostUpdate);

        module.set_getter_fn("dt", |this: &mut Self| Ok(this.dt));
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PreRender {
    pub dt: f64,
}

impl ModuleType for PreRender {
    fn register(module: &mut rhai::Module) {
        impl_event_type!(module, PreRender);

        module.set_getter_fn("dt", |this: &mut Self| Ok(this.dt));
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PostRender {
    pub dt: f64,
}

impl ModuleType for PostRender {
    fn register(module: &mut rhai::Module) {
        impl_event_type!(module, PostRender);

        module.set_getter_fn("dt", |this: &mut Self| Ok(this.dt));
    }
}
