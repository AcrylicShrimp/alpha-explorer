use super::single_device_memory_allocator::{DeviceAllocation, SingleDeviceMemoryAllocator};
use crate::GfxContext;
use wgpu::{util::StagingBelt, BufferAddress, BufferDescriptor, BufferUsages, CommandEncoder};

pub struct DeviceMemoryAllocator {
    index: usize,
    allocators: Vec<SingleDeviceMemoryAllocator>,
    usage: BufferUsages,
}

impl DeviceMemoryAllocator {
    pub const PAGE_SIZE: BufferAddress = 1 * 1024 * 1024;

    pub fn new(usage: BufferUsages) -> Self {
        Self {
            index: 0,
            allocators: Vec::new(),
            usage,
        }
    }

    pub fn release(&mut self) {
        self.index = 0;

        for allocator in &mut self.allocators {
            allocator.release();
        }
    }

    pub fn allocate<T>(
        &mut self,
        gfx_context: &GfxContext,
        encoder: &mut CommandEncoder,
        staging_belt: &mut StagingBelt,
        contents: &[T],
    ) -> DeviceAllocation
    where
        T: Sized,
    {
        #[cfg(debug_assertions)]
        {
            use std::mem::size_of;
            debug_assert!(((size_of::<T>() * contents.len()) as BufferAddress) < Self::PAGE_SIZE);
        }

        let allocator = if let Some(allocator) = self.allocators.get_mut(self.index) {
            allocator
        } else {
            self.add_allocator(gfx_context)
        };

        if let Some(allocation) = allocator.allocate(gfx_context, encoder, staging_belt, contents) {
            return allocation;
        }

        self.index += 1;
        self.allocate(gfx_context, encoder, staging_belt, contents)
    }

    pub fn allocate_without_contents(
        &mut self,
        gfx_context: &GfxContext,
        size: BufferAddress,
    ) -> DeviceAllocation {
        #[cfg(debug_assertions)]
        {
            debug_assert!(size < Self::PAGE_SIZE);
        }

        let allocator = if let Some(allocator) = self.allocators.get_mut(self.index) {
            allocator
        } else {
            self.add_allocator(gfx_context)
        };

        if let Some(allocation) = allocator.allocate_without_contents(size) {
            return allocation;
        }

        self.index += 1;
        self.allocate_without_contents(gfx_context, size)
    }

    fn add_allocator(&mut self, gfx_context: &GfxContext) -> &mut SingleDeviceMemoryAllocator {
        self.allocators.push(SingleDeviceMemoryAllocator::new(
            gfx_context.device.create_buffer(&BufferDescriptor {
                label: None,
                size: Self::PAGE_SIZE,
                usage: BufferUsages::COPY_DST | self.usage,
                mapped_at_creation: false,
            }),
        ));
        self.allocators.last_mut().unwrap()
    }
}
