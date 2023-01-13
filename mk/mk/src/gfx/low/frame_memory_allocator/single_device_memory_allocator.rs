use crate::GfxContext;
use std::{mem::size_of, num::NonZeroU64, ops::Range, sync::Arc};
use wgpu::{util::StagingBelt, Buffer, BufferAddress, BufferSlice, CommandEncoder};

pub struct SingleDeviceMemoryAllocator {
    buffer: Arc<Buffer>,
    used: BufferAddress,
}

impl SingleDeviceMemoryAllocator {
    pub fn new(buffer: Buffer) -> Self {
        Self {
            buffer: Arc::new(buffer),
            used: 0,
        }
    }

    pub fn release(&mut self) {
        self.used = 0;
    }

    pub fn allocate<T>(
        &mut self,
        gfx_context: &GfxContext,
        encoder: &mut CommandEncoder,
        staging_belt: &mut StagingBelt,
        contents: &[T],
    ) -> Option<DeviceAllocation>
    where
        T: Sized,
    {
        self.allocate_without_contents((size_of::<T>() * contents.len()) as BufferAddress)
            .map(|allocation| {
                let (_lhs, contents, _rhs) = unsafe { contents.align_to() };
                debug_assert!(_lhs.len() == 0);
                debug_assert!(_rhs.len() == 0);

                let mut view = staging_belt.write_buffer(
                    encoder,
                    &allocation.buffer,
                    allocation.offset(),
                    NonZeroU64::new(allocation.size()).unwrap(),
                    &gfx_context.device,
                );
                view.copy_from_slice(contents);
                allocation
            })
    }

    pub fn allocate_without_contents(&mut self, size: BufferAddress) -> Option<DeviceAllocation> {
        if self.buffer.size() - self.used < size {
            return None;
        }

        let allocation = DeviceAllocation::new(self.buffer.clone(), self.used, size);
        self.used += size;
        Some(allocation)
    }
}

#[derive(Clone)]
pub struct DeviceAllocation {
    buffer: Arc<Buffer>,
    offset: u32,
    size: u32,
}

impl DeviceAllocation {
    pub fn new(buffer: Arc<Buffer>, offset: BufferAddress, size: BufferAddress) -> Self {
        debug_assert!(offset <= u32::MAX as BufferAddress);
        debug_assert!(size <= u32::MAX as BufferAddress);
        Self {
            buffer,
            offset: offset as u32,
            size: size as u32,
        }
    }

    pub fn buffer(&self) -> &Arc<Buffer> {
        &self.buffer
    }

    pub fn offset(&self) -> BufferAddress {
        self.offset as BufferAddress
    }

    pub fn size(&self) -> BufferAddress {
        self.size as BufferAddress
    }

    pub fn range(&self) -> Range<BufferAddress> {
        self.offset()..self.offset() + self.size()
    }

    pub fn as_slice(&self) -> BufferSlice {
        self.buffer.slice(self.range())
    }

    pub fn as_slice_instanced(&self, instances: usize) -> BufferSlice {
        self.buffer
            .slice(self.offset()..self.offset() + self.size() * instances as u64)
    }
}
