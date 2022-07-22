use crate::render::*;
use crate::EngineContextWithoutSystemManager;

pub struct RenderManager {
    buffer_pool: Vec<Buffer>,
    common_shader_input_buffer: Buffer,
}

impl RenderManager {
    pub fn new() -> Self {
        Self {
            buffer_pool: Vec::new(),
            common_shader_input_buffer: Buffer::from_slice(&[
                0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32,
            ]),
        }
    }

    pub fn alloc_buffer(&mut self) -> Buffer {
        self.buffer_pool.pop().unwrap_or_default()
    }

    pub fn dealloc_buffer(&mut self, buffer: Buffer) {
        self.buffer_pool.push(buffer);
    }

    pub fn update_uniforms(&self, context: &EngineContextWithoutSystemManager) {
        let time_mgr = context.time_mgr();
        let screen_mgr = context.screen_mgr();

        self.common_shader_input_buffer.update(
            0,
            &[
                time_mgr.dt(),
                1f32 / time_mgr.dt(),
                time_mgr.time().elapsed().as_secs_f32(),
                1f32 / time_mgr.time().elapsed().as_secs_f32(),
                screen_mgr.width() as f32,
                screen_mgr.height() as f32,
                1f32 / screen_mgr.width() as f32,
                1f32 / screen_mgr.height() as f32,
            ],
        );
    }

    pub fn apply_common_shader_input(&self, shader: &Shader, req: &mut RenderRequest) {
        // TODO: Add shader type checking logic to alert if types have no match.

        if let Some(uniform) = shader.uniform("Common") {
            req.uniform_block(uniform.location, &self.common_shader_input_buffer);
        }
    }
}
