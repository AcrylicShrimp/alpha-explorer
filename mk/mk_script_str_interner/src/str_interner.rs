use crate::{str_chunk::StrChunk, StrIdx};
use rustc_hash::FxHashMap;
use std::{cmp::max, ptr::copy_nonoverlapping, slice::from_raw_parts, str::from_utf8_unchecked};

#[derive(Debug)]
pub struct StrInterner {
    chunks: Vec<StrChunk>,
    strings: Vec<&'static str>,
    reversed: FxHashMap<&'static str, StrIdx>,
}

impl StrInterner {
    pub fn with_prefilled(strs: &[&'static str]) -> Self {
        let mut strings = vec![""];
        strings.extend(strs.iter());

        Self {
            chunks: Vec::with_capacity(4),
            strings,
            reversed: strs.iter().copied().zip((1..).map(StrIdx::new)).collect(),
        }
    }

    pub fn get_str(&self, idx: StrIdx) -> &'static str {
        self.strings[u32::from(idx) as usize]
    }

    pub fn intern<S: AsRef<str>>(&mut self, str: S) -> StrIdx {
        let str = str.as_ref();

        if let Some(&reverse) = self.reversed.get(str) {
            return reverse;
        }

        let ptr = self
            .chunks
            .last_mut()
            .and_then(|chunk| chunk.alloc(str.len()))
            .unwrap_or_else(|| {
                let mut chunk = StrChunk::with_capacity(max(4096, str.len()));
                let ptr = chunk.alloc(str.len()).unwrap();
                self.chunks.push(chunk);
                ptr
            });

        unsafe {
            copy_nonoverlapping(str.as_ptr(), ptr, str.len());
        }

        let str = unsafe { from_utf8_unchecked(from_raw_parts(ptr, str.len())) };
        let str_idx = StrIdx::new(self.strings.len() as _);
        self.strings.push(str);
        self.reversed.insert(str, str_idx);
        str_idx
    }
}
