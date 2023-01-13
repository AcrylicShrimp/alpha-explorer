use super::single_host_memory_allocator::{HostAllocation, SingleHostMemoryAllocator};

pub struct HostMemoryAllocator {
    index: usize,
    allocators: Vec<SingleHostMemoryAllocator>,
}

impl HostMemoryAllocator {
    pub const PAGE_SIZE: usize = 1 * 1024 * 1024;

    pub fn new() -> Self {
        Self {
            index: 0,
            allocators: Vec::new(),
        }
    }

    pub fn release(&mut self) {
        self.index = 0;

        for allocator in &mut self.allocators {
            allocator.release();
        }
    }

    pub fn allocate<T>(&mut self, contents: &[T]) -> HostAllocation
    where
        T: Sized,
    {
        #[cfg(debug_assertions)]
        {
            use std::mem::size_of;
            debug_assert!(size_of::<T>() * contents.len() < Self::PAGE_SIZE);
        }

        let allocator = if let Some(allocator) = self.allocators.get_mut(self.index) {
            allocator
        } else {
            self.add_allocator()
        };

        if let Some(allocation) = allocator.allocate(contents) {
            return allocation;
        }

        self.index += 1;
        self.allocate(contents)
    }

    pub fn allocate_without_contents(&mut self, size: usize) -> HostAllocation {
        #[cfg(debug_assertions)]
        {
            debug_assert!(size < Self::PAGE_SIZE);
        }

        let allocator = if let Some(allocator) = self.allocators.get_mut(self.index) {
            allocator
        } else {
            self.add_allocator()
        };

        if let Some(allocation) = allocator.allocate_without_contents(size) {
            return allocation;
        }

        self.index += 1;
        self.allocate_without_contents(size)
    }

    fn add_allocator(&mut self) -> &mut SingleHostMemoryAllocator {
        self.allocators
            .push(SingleHostMemoryAllocator::new(vec![0u8; Self::PAGE_SIZE]));
        self.allocators.last_mut().unwrap()
    }
}
