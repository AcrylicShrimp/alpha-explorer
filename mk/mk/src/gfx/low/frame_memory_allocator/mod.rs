mod device_memory_allocator;
mod host_memory_allocator;
mod single_device_memory_allocator;
mod single_host_memory_allocator;

pub use device_memory_allocator::*;
pub use host_memory_allocator::*;
pub use single_device_memory_allocator::*;
pub use single_host_memory_allocator::*;

use crate::GfxContext;
use std::{mem::replace, num::NonZeroU64};
use wgpu::{util::StagingBelt, BufferAddress, BufferUsages, CommandEncoder};

pub struct FrameMemoryAllocator {
    encoder: CommandEncoder,
    staging_belt: StagingBelt,
    host_allocator: HostMemoryAllocator,
    vertex_allocator: DeviceMemoryAllocator,
    uniform_allocator: DeviceMemoryAllocator,
}

impl FrameMemoryAllocator {
    pub fn new(gfx_context: &GfxContext) -> Self {
        Self {
            encoder: gfx_context
                .device
                .create_command_encoder(&Default::default()),
            staging_belt: StagingBelt::new(8 * DeviceMemoryAllocator::PAGE_SIZE),
            host_allocator: HostMemoryAllocator::new(),
            vertex_allocator: DeviceMemoryAllocator::new(BufferUsages::VERTEX),
            uniform_allocator: DeviceMemoryAllocator::new(BufferUsages::UNIFORM),
        }
    }

    pub fn submit(&mut self, gfx_context: &GfxContext) -> CommandEncoder {
        self.staging_belt.finish();
        replace(
            &mut self.encoder,
            gfx_context
                .device
                .create_command_encoder(&Default::default()),
        )
    }

    pub fn release(&mut self) {
        self.staging_belt.recall();
        self.host_allocator.release();
        self.vertex_allocator.release();
        self.uniform_allocator.release();
    }

    pub fn allocate_host_buffer<T>(&mut self, contents: &[T]) -> HostAllocation
    where
        T: Sized,
    {
        self.host_allocator.allocate(contents)
    }

    pub fn allocate_host_buffer_without_contents(&mut self, size: usize) -> HostAllocation {
        self.host_allocator.allocate_without_contents(size)
    }

    pub fn allocate_vertex_buffer<T>(
        &mut self,
        gfx_context: &GfxContext,
        contents: &[T],
    ) -> DeviceAllocation
    where
        T: Sized,
    {
        self.vertex_allocator.allocate(
            gfx_context,
            &mut self.encoder,
            &mut self.staging_belt,
            contents,
        )
    }

    pub fn allocate_vertex_buffer_without_contents(
        &mut self,
        gfx_context: &GfxContext,
        size: BufferAddress,
    ) -> DeviceAllocation {
        self.vertex_allocator
            .allocate_without_contents(gfx_context, size)
    }

    pub fn allocate_uniform_buffer<T>(
        &mut self,
        gfx_context: &GfxContext,
        contents: &[T],
    ) -> DeviceAllocation
    where
        T: Sized,
    {
        self.uniform_allocator.allocate(
            gfx_context,
            &mut self.encoder,
            &mut self.staging_belt,
            contents,
        )
    }

    pub fn write_device_allocation<T>(
        &mut self,
        gfx_context: &GfxContext,
        allocation: &DeviceAllocation,
        contents: &[T],
    ) where
        T: Sized,
    {
        let (_lhs, contents, _rhs) = unsafe { contents.align_to() };
        debug_assert!(_lhs.len() == 0);
        debug_assert!(_rhs.len() == 0);
        self.staging_belt
            .write_buffer(
                &mut self.encoder,
                allocation.buffer(),
                allocation.offset(),
                NonZeroU64::new(contents.len() as BufferAddress).unwrap(),
                &gfx_context.device,
            )
            .copy_from_slice(contents);
    }
}
