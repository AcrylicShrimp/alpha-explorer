use crate::GfxContext;
use std::{mem::size_of, ops::Range, sync::Arc};
use wgpu::{Buffer, BufferAddress, BufferSlice, CommandEncoderDescriptor};

pub struct SingleMemoryAllocator {
    buffer: Arc<Buffer>,
    used: BufferAddress,
}

impl SingleMemoryAllocator {
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
        contents: &[T],
    ) -> Option<SingleAllocation>
    where
        T: Sized,
    {
        self.allocate_slice((size_of::<T>() * contents.len()) as BufferAddress)
            .map(|allocation| {
                let (_lhs, contents, _rhs) = unsafe { contents.align_to() };
                debug_assert!(_lhs.len() == 0);
                debug_assert!(_rhs.len() == 0);

                // let mut encoder = gfx_context
                //     .device
                //     .create_command_encoder(&CommandEncoderDescriptor {
                //         ..Default::default()
                //     });
                //     encoder.

                // gfx_context.queue.

                gfx_context
                    .queue
                    .write_buffer(&allocation.buffer, allocation.offset(), contents);
                allocation
            })
    }

    fn allocate_slice(&mut self, size: BufferAddress) -> Option<SingleAllocation> {
        if self.buffer.size() - self.used < size {
            return None;
        }

        let allocation = SingleAllocation::new(self.buffer.clone(), self.used, size);
        self.used += size;
        Some(allocation)
    }
}

#[derive(Clone)]
pub struct SingleAllocation {
    buffer: Arc<Buffer>,
    offset: u32,
    size: u32,
}

impl SingleAllocation {
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
}
