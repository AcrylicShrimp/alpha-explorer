use std::{cell::RefCell, mem::size_of, ops::Range, sync::Arc};

pub struct SingleHostMemoryAllocator {
    buffer: Arc<RefCell<Vec<u8>>>,
    used: usize,
}

impl SingleHostMemoryAllocator {
    pub fn new(buffer: Vec<u8>) -> Self {
        Self {
            buffer: Arc::new(RefCell::new(buffer)),
            used: 0,
        }
    }

    pub fn release(&mut self) {
        self.used = 0;
    }

    pub fn allocate<T>(&mut self, contents: &[T]) -> Option<HostAllocation>
    where
        T: Sized,
    {
        self.allocate_without_contents(size_of::<T>() * contents.len())
            .map(|allocation| {
                allocation.copy_from_slice(contents, 0);
                allocation
            })
    }

    pub fn allocate_without_contents(&mut self, size: usize) -> Option<HostAllocation> {
        if self.buffer.borrow().len() - self.used < size {
            return None;
        }

        let allocation = HostAllocation::new(self.buffer.clone(), self.used, size);
        self.used += size;
        Some(allocation)
    }
}

#[derive(Clone)]
pub struct HostAllocation {
    buffer: Arc<RefCell<Vec<u8>>>,
    offset: u32,
    size: u32,
}

impl HostAllocation {
    pub fn new(buffer: Arc<RefCell<Vec<u8>>>, offset: usize, size: usize) -> Self {
        debug_assert!(offset <= u32::MAX as usize);
        debug_assert!(size <= u32::MAX as usize);
        Self {
            buffer,
            offset: offset as u32,
            size: size as u32,
        }
    }

    pub fn buffer(&self) -> &Arc<RefCell<Vec<u8>>> {
        &self.buffer
    }

    pub fn offset(&self) -> usize {
        self.offset as usize
    }

    pub fn size(&self) -> usize {
        self.size as usize
    }

    pub fn range(&self) -> Range<usize> {
        self.offset()..self.offset() + self.size()
    }

    pub fn copy_from_slice<T>(&self, src: &[T], dst_offset: usize)
    where
        T: Sized,
    {
        let (_lhs, contents, _rhs) = unsafe { src.align_to() };
        debug_assert!(_lhs.len() == 0);
        debug_assert!(_rhs.len() == 0);
        let offset = (self.offset as usize) + dst_offset;
        self.buffer.borrow_mut().as_mut_slice()[offset..offset + contents.len()]
            .copy_from_slice(contents);
    }

    pub fn copy_from_allocation(&self, src: &Self, dst_offset: usize) {
        if Arc::ptr_eq(&self.buffer, &src.buffer) {
            self.buffer
                .borrow_mut()
                .copy_within(src.range(), self.offset() + dst_offset);
        } else {
            self.buffer.borrow_mut().as_mut_slice()
                [self.offset() + dst_offset..self.offset() + dst_offset + src.size()]
                .copy_from_slice(&src.buffer.borrow().as_slice()[src.range()]);
        }
    }
}
