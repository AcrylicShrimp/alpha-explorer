use super::single_memory_allocator::{SingleAllocation, SingleMemoryAllocator};
use crate::GfxContext;
use wgpu::{BufferAddress, BufferDescriptor, BufferUsages};

pub struct MemoryAllocator {
    index: usize,
    allocators: Vec<SingleMemoryAllocator>,
    usage: BufferUsages,
}

impl MemoryAllocator {
    const PAGE_SIZE: BufferAddress = 1 * 1024 * 1024;

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

    pub fn allocate<T>(&mut self, gfx_context: &GfxContext, contents: &[T]) -> SingleAllocation {
        #[cfg(debug_assertions)]
        {
            use std::mem::size_of;
            debug_assert!(((size_of::<T>() * contents.len()) as BufferAddress) < Self::PAGE_SIZE);
        }

        if self.allocators.is_empty() {
            return self
                .add_allocator(gfx_context)
                .allocate(gfx_context, contents)
                .unwrap();
        }

        if let Some(allocation) = self.allocators[self.index].allocate(gfx_context, contents) {
            return allocation;
        }

        self.index += 1;
        return self
            .add_allocator(gfx_context)
            .allocate(gfx_context, contents)
            .unwrap();
    }

    fn add_allocator(&mut self, gfx_context: &GfxContext) -> &mut SingleMemoryAllocator {
        self.allocators.push(SingleMemoryAllocator::new(
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
