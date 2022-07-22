use crate::render::SpriteError;
use glutin::error::OsError as OSError;
use glutin::ContextError;
use glutin::CreationError as ContextCreationError;
use mlua::prelude::LuaError;
use std::io::Error as IOError;

#[derive(Debug)]
pub enum EngineError {
    ContextError(ContextError),
    // WindowError(WindowError),
    SpriteError(SpriteError),
    LuaError(LuaError),
    EventPumpError(String),
    IOError(IOError),
    OSError(OSError),
    ContextCreationError(ContextCreationError),
}

impl From<ContextError> for EngineError {
    fn from(err: ContextError) -> EngineError {
        EngineError::ContextError(err)
    }
}

// impl From<WindowError> for EngineError {
//     fn from(err: WindowError) -> EngineError {
//         EngineError::WindowError(err)
//     }
// }

impl From<LuaError> for EngineError {
    fn from(err: LuaError) -> EngineError {
        EngineError::LuaError(err)
    }
}

impl From<IOError> for EngineError {
    fn from(err: IOError) -> EngineError {
        EngineError::IOError(err)
    }
}

impl From<OSError> for EngineError {
    fn from(err: OSError) -> EngineError {
        EngineError::OSError(err)
    }
}

impl From<ContextCreationError> for EngineError {
    fn from(err: ContextCreationError) -> EngineError {
        EngineError::ContextCreationError(err)
    }
}
