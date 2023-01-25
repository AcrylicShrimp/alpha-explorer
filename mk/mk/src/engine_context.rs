use crate::asset::AssetManager;
use crate::audio::AudioManager;
use crate::component::register_components;
use crate::event::{EntityEventManager, EventManager};
use crate::gfx::{GlyphManager, RenderManager, ScreenManager};
use crate::input::InputManager;
use crate::script::ScriptManager;
use crate::time::TimeManager;
use crate::transform::TransformManager;
use crate::ui::{UIEventManager, UIManager};
use crate::GfxContext;
use anyhow::{Context, Result};
use specs::prelude::*;
use std::cell::{Ref, RefCell, RefMut};
use std::path::PathBuf;

pub struct EngineContext {
    world: RefCell<World>,
    time_mgr: RefCell<TimeManager>,
    input_mgr: RefCell<InputManager>,
    screen_mgr: RefCell<ScreenManager>,
    audio_mgr: AudioManager,
    asset_mgr: RefCell<AssetManager>,
    transform_mgr: RefCell<TransformManager>,
    event_mgr: EventManager,
    entity_event_mgr: EntityEventManager,
    script_mgr: ScriptManager,
    glyph_mgr: RefCell<GlyphManager>,
    render_mgr: RefCell<RenderManager>,
    ui_mgr: RefCell<UIManager>,
    ui_event_mgr: RefCell<UIEventManager>,
}

impl EngineContext {
    pub fn new(
        gfx_context: GfxContext,
        screen_width: u32,
        screen_height: u32,
        asset_mgr_base: PathBuf,
    ) -> Result<Self> {
        let mut world = World::new();

        register_components(&mut world);

        Ok(Self {
            world: world.into(),
            time_mgr: TimeManager::new().into(),
            input_mgr: InputManager::new().into(),
            screen_mgr: ScreenManager::new(screen_width, screen_height).into(),
            audio_mgr: AudioManager::new(),
            asset_mgr: AssetManager::new(asset_mgr_base).into(),
            transform_mgr: TransformManager::new().into(),
            event_mgr: EventManager::new(),
            entity_event_mgr: EntityEventManager::new(),
            script_mgr: ScriptManager::new()
                .with_context(|| "failed to initialize script manager")?
                .into(),
            glyph_mgr: GlyphManager::new(128f32, 8usize, 48usize, 0.5f32).into(),
            render_mgr: RenderManager::new(gfx_context).into(),
            ui_mgr: UIManager::new().into(),
            ui_event_mgr: UIEventManager::new().into(),
        })
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

    pub fn audio_mgr(&self) -> &AudioManager {
        &self.audio_mgr
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

    pub fn event_mgr(&self) -> &EventManager {
        &self.event_mgr
    }

    pub fn entity_event_mgr(&self) -> &EntityEventManager {
        &self.entity_event_mgr
    }

    pub fn script_mgr(&self) -> &ScriptManager {
        &self.script_mgr
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
