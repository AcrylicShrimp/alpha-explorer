use crate::engine::use_context;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct UIElement {
    index: u32,
}

impl UIElement {
    pub fn new(index: u32) -> Self {
        Self { index }
    }

    pub fn index(&self) -> u32 {
        self.index
    }

    pub fn with_ref<R>(&self, f: impl FnOnce(&crate::ui::UIElement) -> R) -> R {
        let ui_mgr = use_context().ui_mgr();
        f(ui_mgr.element(self.index))
    }

    pub fn with_mut<R>(&self, f: impl FnOnce(&mut crate::ui::UIElement) -> R) -> R {
        let mut ui_mgr = use_context().ui_mgr_mut();
        f(ui_mgr.element_mut(self.index))
    }
}
