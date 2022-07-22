use crate::api::LuaManager;
use crate::asset::AssetManager;
use crate::event::{EntityEventManager, EventManager};
use crate::glyph::GlyphManager;
use crate::input::InputManager;
use crate::render::{RenderManager, ScreenManager};
use crate::system::SystemManager;
use crate::time::TimeManager;
use crate::transform::TransformManager;
use crate::ui::{UIEventManager, UIManager};
use crate::EngineError;
use legion::World;
use std::cell::{Ref, RefCell, RefMut};
use std::path::PathBuf;
use std::sync::Arc;

pub struct EngineContextWithoutSystemManager {
    world: RefCell<World>,
    time_mgr: RefCell<TimeManager>,
    input_mgr: RefCell<InputManager>,
    screen_mgr: RefCell<ScreenManager>,
    asset_mgr: RefCell<AssetManager>,
    transform_mgr: RefCell<TransformManager>,
    lua_mgr: LuaManager,
    event_mgr: EventManager,
    entity_event_mgr: RefCell<EntityEventManager>,
    glyph_mgr: RefCell<GlyphManager>,
    render_mgr: RefCell<RenderManager>,
    ui_mgr: RefCell<UIManager>,
    ui_event_mgr: RefCell<UIEventManager>,
}

impl EngineContextWithoutSystemManager {
    pub fn new(screen_width: u32, screen_height: u32, asset_mgr_base: PathBuf) -> Self {
        Self {
            world: World::default().into(),
            time_mgr: TimeManager::new().into(),
            input_mgr: InputManager::new().into(),
            screen_mgr: ScreenManager::new(screen_width, screen_height).into(),
            asset_mgr: AssetManager::new(asset_mgr_base).into(),
            transform_mgr: TransformManager::new().into(),
            lua_mgr: LuaManager::new(),
            event_mgr: EventManager::new(),
            entity_event_mgr: EntityEventManager::new().into(),
            glyph_mgr: GlyphManager::new(128f32, 8usize, 48usize, 0.5f32).into(),
            render_mgr: RenderManager::new().into(),
            ui_mgr: UIManager::new().into(),
            ui_event_mgr: UIEventManager::new().into(),
        }
    }

    pub fn world(&self) -> Ref<World> {
        self.world.borrow()
    }

    pub fn world_mut(&self) -> RefMut<World> {
        self.world.borrow_mut()
    }

    pub fn time_mgr(&self) -> Ref<TimeManager> {
        self.time_mgr.borrow()
    }

    pub fn time_mgr_mut(&self) -> RefMut<TimeManager> {
        self.time_mgr.borrow_mut()
    }

    pub fn input_mgr(&self) -> Ref<InputManager> {
        self.input_mgr.borrow()
    }

    pub fn input_mgr_mut(&self) -> RefMut<InputManager> {
        self.input_mgr.borrow_mut()
    }

    pub fn screen_mgr(&self) -> Ref<ScreenManager> {
        self.screen_mgr.borrow()
    }

    pub fn screen_mgr_mut(&self) -> RefMut<ScreenManager> {
        self.screen_mgr.borrow_mut()
    }

    pub fn asset_mgr(&self) -> Ref<AssetManager> {
        self.asset_mgr.borrow()
    }

    pub fn asset_mgr_mut(&self) -> RefMut<AssetManager> {
        self.asset_mgr.borrow_mut()
    }

    pub fn transform_mgr(&self) -> Ref<TransformManager> {
        self.transform_mgr.borrow()
    }

    pub fn transform_mgr_mut(&self) -> RefMut<TransformManager> {
        self.transform_mgr.borrow_mut()
    }

    pub fn lua_mgr(&self) -> &LuaManager {
        &self.lua_mgr
    }

    pub fn event_mgr(&self) -> &EventManager {
        &self.event_mgr
    }

    pub fn entity_event_mgr(&self) -> Ref<EntityEventManager> {
        self.entity_event_mgr.borrow()
    }

    pub fn entity_event_mgr_mut(&self) -> RefMut<EntityEventManager> {
        self.entity_event_mgr.borrow_mut()
    }

    pub fn glyph_mgr(&self) -> Ref<GlyphManager> {
        self.glyph_mgr.borrow()
    }

    pub fn glyph_mgr_mut(&self) -> RefMut<GlyphManager> {
        self.glyph_mgr.borrow_mut()
    }

    pub fn render_mgr(&self) -> Ref<RenderManager> {
        self.render_mgr.borrow()
    }

    pub fn render_mgr_mut(&self) -> RefMut<RenderManager> {
        self.render_mgr.borrow_mut()
    }

    pub fn ui_mgr(&self) -> Ref<UIManager> {
        self.ui_mgr.borrow()
    }

    pub fn ui_mgr_mut(&self) -> RefMut<UIManager> {
        self.ui_mgr.borrow_mut()
    }

    pub fn ui_event_mgr(&self) -> Ref<UIEventManager> {
        self.ui_event_mgr.borrow()
    }

    pub fn ui_event_mgr_mut(&self) -> RefMut<UIEventManager> {
        self.ui_event_mgr.borrow_mut()
    }
}

impl Drop for EngineContextWithoutSystemManager {
    fn drop(&mut self) {
        self.world_mut().clear();
    }
}

pub struct EngineContext {
    system_mgr: SystemManager,
    context: Arc<EngineContextWithoutSystemManager>,
}

impl EngineContext {
    pub fn new(
        screen_width: u32,
        screen_height: u32,
        asset_mgr_base: PathBuf,
    ) -> Result<Self, EngineError> {
        Ok(Self {
            system_mgr: SystemManager::default(),
            context: Arc::new(EngineContextWithoutSystemManager::new(
                screen_width,
                screen_height,
                asset_mgr_base,
            )),
        })
    }

    pub fn into_split(self) -> (SystemManager, Arc<EngineContextWithoutSystemManager>) {
        (self.system_mgr, self.context)
    }
}
