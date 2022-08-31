use crate::script::api::ModuleType;

pub type ComponentDiagnostic = super::Component<crate::component::Diagnostic>;

impl ModuleType for ComponentDiagnostic {
    fn register(module: &mut rhai::Module) {
        module.set_custom_type::<Self>("ComponentDiagnostic");

        to_global!(
            module,
            module.set_native_fn("is_exists", |this: &mut Self| { Ok(this.is_exists()) })
        );

        to_global!(
            module,
            module.set_native_fn("to_string", |this: &mut Self| Ok(format!(
                "ComponentDiagnostic(entity={:?}, is_exists={})",
                this.entity,
                this.is_exists()
            )))
        );
        to_global!(
            module,
            module.set_native_fn("to_debug", |this: &mut Self| Ok(format!(
                "ComponentDiagnostic(entity={:?}, is_exists={})",
                this.entity,
                this.is_exists()
            )))
        );
    }
}
