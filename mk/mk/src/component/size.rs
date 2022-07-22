use codegen::LuaComponentNoWrapper;

#[derive(LuaComponentNoWrapper, Debug, Clone, Copy, PartialEq)]
pub struct Size {
    #[lua_hidden]
    index: u32,
    pub width: f32,
    pub height: f32,
}

impl Size {
    pub fn new(index: u32) -> Self {
        Self {
            index,
            width: 0f32,
            height: 0f32,
        }
    }

    pub fn index(&self) -> u32 {
        self.index
    }
}
