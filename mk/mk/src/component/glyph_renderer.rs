use crate::render::{Color, Layer, LuaRcFont, LuaRcShader, Shader};
use crate::structure::Size;
use codegen::{Animation, LuaComponent};
use fontdue::layout::{
    CoordinateSystem, HorizontalAlign, Layout, LayoutSettings, TextStyle, VerticalAlign, WrapStyle,
};
use fontdue::Font;
use mlua::prelude::*;
use std::sync::Arc;

#[derive(Clone)]
pub struct GlyphRendererConfig {
    pub horizontal_align: HorizontalAlign,
    pub vertical_align: VerticalAlign,
    pub wrap_style: WrapStyle,
    pub wrap_hard_breaks: bool,
}

impl Default for GlyphRendererConfig {
    fn default() -> Self {
        Self {
            horizontal_align: HorizontalAlign::Left,
            vertical_align: VerticalAlign::Top,
            wrap_style: WrapStyle::Word,
            wrap_hard_breaks: true,
        }
    }
}

impl<'lua> FromLua<'lua> for GlyphRendererConfig {
    fn from_lua(value: LuaValue<'lua>, _lua: &'lua Lua) -> LuaResult<Self> {
        let table = match value {
            LuaValue::Table(table) => table,
            _ => {
                return Err(
                    format!("the type {} must be a {}", "GlyphRendererConfig", "table")
                        .to_lua_err(),
                );
            }
        };

        Ok(Self {
            horizontal_align: if table.contains_key("horizontal_align")? {
                match table.get::<_, String>("horizontal_align")?.as_str() {
                    "left" => HorizontalAlign::Left,
                    "center" => HorizontalAlign::Center,
                    "right" => HorizontalAlign::Right,
                    value => {
                        return Err(format!(
                            "the string '{}' is not valid type {}",
                            value, "HorizontalAlign",
                        )
                        .to_lua_err());
                    }
                }
            } else {
                HorizontalAlign::Left
            },
            vertical_align: if table.contains_key("vertical_align")? {
                match table.get::<_, String>("vertical_align")?.as_str() {
                    "top" => VerticalAlign::Top,
                    "middle" => VerticalAlign::Middle,
                    "bottom" => VerticalAlign::Bottom,
                    value => {
                        return Err(format!(
                            "the string '{}' is not valid type {}",
                            value, "VerticalAlign",
                        )
                        .to_lua_err());
                    }
                }
            } else {
                VerticalAlign::Top
            },
            wrap_style: if table.contains_key("wrap_style")? {
                match table.get::<_, String>("wrap_style")?.as_str() {
                    "letter" => WrapStyle::Letter,
                    "word" => WrapStyle::Word,
                    value => {
                        return Err(format!(
                            "the string '{}' is not valid type {}",
                            value, "WrapStyle",
                        )
                        .to_lua_err());
                    }
                }
            } else {
                WrapStyle::Word
            },
            wrap_hard_breaks: if table.contains_key("wrap_style")? {
                table.get("wrap_hard_breaks")?
            } else {
                true
            },
        })
    }
}

impl<'lua> ToLua<'lua> for GlyphRendererConfig {
    fn to_lua(self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        let table = lua.create_table()?;
        table.set(
            "horizontal_align",
            match self.horizontal_align {
                HorizontalAlign::Left => "left",
                HorizontalAlign::Center => "center",
                HorizontalAlign::Right => "right",
            },
        )?;
        table.set(
            "vertical_align",
            match self.vertical_align {
                VerticalAlign::Top => "top",
                VerticalAlign::Middle => "middle",
                VerticalAlign::Bottom => "bottom",
            },
        )?;
        table.set(
            "wrap_style",
            match self.wrap_style {
                WrapStyle::Letter => "letter",
                WrapStyle::Word => "word",
            },
        )?;
        table.set("wrap_hard_breaks", self.wrap_hard_breaks)?;
        Ok(LuaValue::Table(table))
    }
}

#[derive(Animation, LuaComponent)]
pub struct GlyphRenderer {
    pub layer: Layer,
    pub order: isize,
    pub color: Color,
    #[lua_userdata(LuaRcShader)]
    pub shader: Arc<Shader>,
    #[lua_userdata(LuaRcFont)]
    #[lua_userfunc(set=lua_set_font)]
    // NOTE: Support the userfunc to the animation derive macro too.
    font: Arc<Font>,
    #[lua_userfunc(set=lua_set_font_size)]
    font_size: f32,
    pub thickness: f32,
    pub smoothness: f32,
    #[lua_userfunc(set=lua_set_config)]
    config: GlyphRendererConfig,
    #[lua_userfunc(set=lua_set_text)]
    text: String,
    #[lua_hidden]
    layout: Layout,
    // #[lua_readonly]
    // #[lua_userfunc(get=lua_get_lines)]
    // lines: PhantomData<u32>,
}

impl GlyphRenderer {
    pub fn new(
        shader: Arc<Shader>,
        font: Arc<Font>,
        font_size: f32,
        thickness: f32,
        smoothness: f32,
    ) -> Self {
        Self {
            layer: Layer::default(),
            order: 0,
            color: Color::white(),
            shader,
            font,
            font_size,
            thickness,
            smoothness,
            config: GlyphRendererConfig::default(),
            text: String::with_capacity(32),
            layout: Layout::new(CoordinateSystem::PositiveYUp),
            // lines: PhantomData,
        }
    }

    pub fn font(&self) -> &Arc<Font> {
        &self.font
    }

    pub fn font_size(&self) -> f32 {
        self.font_size
    }

    pub fn config(&self) -> &GlyphRendererConfig {
        &self.config
    }

    pub fn text(&self) -> &str {
        self.text.as_str()
    }

    pub fn layout(&self) -> &Layout {
        &self.layout
    }

    pub fn font_and_layout(&mut self) -> (&Arc<Font>, &mut Layout) {
        (&self.font, &mut self.layout)
    }

    pub fn set_font(&mut self, font: Arc<Font>) {
        self.font = font;
        self.layout.clear();
        self.layout.append(
            &[self.font.as_ref()],
            &TextStyle::new(self.text.as_str(), self.font_size, 0),
        );
    }

    pub fn set_font_size(&mut self, font_size: f32) {
        self.font_size = font_size;
        self.layout.clear();
        self.layout.append(
            &[self.font.as_ref()],
            &TextStyle::new(self.text.as_str(), self.font_size, 0),
        );
    }

    pub fn set_config(&mut self, config: GlyphRendererConfig) {
        self.config = config;
        self.layout.reset(&LayoutSettings {
            x: 0f32,
            y: 0f32,
            max_width: None,
            max_height: None,
            horizontal_align: self.config.horizontal_align,
            vertical_align: self.config.vertical_align,
            wrap_style: self.config.wrap_style,
            wrap_hard_breaks: self.config.wrap_hard_breaks,
        });
        self.layout.append(
            &[self.font.as_ref()],
            &TextStyle::new(self.text.as_str(), self.font_size, 0),
        );
    }

    pub fn set_text(&mut self, text: String) {
        self.text = text;
        self.layout.clear();
        self.layout.append(
            &[self.font.as_ref()],
            &TextStyle::new(self.text.as_str(), self.font_size, 0),
        );
    }

    pub fn compute_size(&self) -> Size {
        let mut width = 0f32;

        for glyph in self.layout.glyphs() {
            width = width.max(glyph.x + glyph.width as f32);
        }

        Size::new(width, self.layout.height())
    }

    fn lua_set_font(&mut self, value: LuaValue, lua: &Lua) -> LuaResult<()> {
        self.set_font(<_>::from(LuaRcFont::from_lua(value, lua)?));
        Ok(())
    }

    fn lua_set_font_size(&mut self, value: LuaValue, lua: &Lua) -> LuaResult<()> {
        self.set_font_size(f32::from_lua(value, lua)?);
        Ok(())
    }

    fn lua_set_config(&mut self, value: LuaValue, lua: &Lua) -> LuaResult<()> {
        self.set_config(GlyphRendererConfig::from_lua(value, lua)?);
        Ok(())
    }

    fn lua_set_text(&mut self, value: LuaValue, lua: &Lua) -> LuaResult<()> {
        self.set_text(String::from_lua(value, lua)?);
        Ok(())
    }

    // fn lua_get_lines<'lua>(&self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
    //     self.layout.lines().to_lua(lua)
    // }
}
