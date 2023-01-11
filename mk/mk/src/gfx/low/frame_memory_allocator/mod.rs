mod memory_allocator;
mod single_memory_allocator;

pub use memory_allocator::*;
pub use single_memory_allocator::*;

use crate::GfxContext;
use wgpu::BufferUsages;

pub struct FrameMemoryAllocator {
    vertex_allocator: MemoryAllocator,
    uniform_allocator: MemoryAllocator,
}

impl FrameMemoryAllocator {
    pub fn new() -> Self {
        Self {
            vertex_allocator: MemoryAllocator::new(BufferUsages::VERTEX),
            uniform_allocator: MemoryAllocator::new(BufferUsages::UNIFORM),
        }
    }

    pub fn release(&mut self) {
        self.vertex_allocator.release();
        self.uniform_allocator.release();
    }

    pub fn allocate_vertex_buffer<T>(
        &mut self,
        gfx_context: &GfxContext,
        contents: &[T],
    ) -> SingleAllocation
    where
        T: Sized,
    {
        self.vertex_allocator.allocate(gfx_context, contents)
    }

    pub fn allocate_uniform_buffer<T>(
        &mut self,
        gfx_context: &GfxContext,
        contents: &[T],
    ) -> SingleAllocation
    where
        T: Sized,
    {
        self.uniform_allocator.allocate(gfx_context, contents)
    }
}
