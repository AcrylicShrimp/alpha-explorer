use crate::writer::{
    EncodedResource, EncoderError, ResourceEncoder, ResourceEncoderDirectoryManager,
};
use crate::{ResourceMeta, ResourceMetaValue, ResourceUUID};
use brotli::CompressorWriter;
use memmap2::Mmap;
use std::io::Write;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TextEncoder {
    pub q: u32,
    pub lgwin: u32,
}

impl TextEncoder {
    pub fn new(q: u32, lgwin: u32) -> Self {
        Self { q, lgwin }
    }
}

impl Default for TextEncoder {
    fn default() -> Self {
        Self { q: 11, lgwin: 22 }
    }
}

impl ResourceEncoder for TextEncoder {
    fn ty(&self) -> &str {
        "text"
    }

    fn encode(
        &self,
        dir_mgr: &dyn ResourceEncoderDirectoryManager,
        uuid: ResourceUUID,
        src: Mmap,
    ) -> Result<EncodedResource, EncoderError> {
        let mut file = dir_mgr.alloc_tmp_file(uuid)?;

        {
            let mut writer = CompressorWriter::new(&mut file, 4096, self.q, self.lgwin);
            writer.write_all(&src)?;
        }

        let content = unsafe { Mmap::map(&file) }?;
        let mut meta = ResourceMeta::new();
        meta.insert("compression".to_owned(), ResourceMetaValue::Boolean(true));
        meta.insert(
            "algorithm".to_owned(),
            ResourceMetaValue::String("brotli".to_owned()),
        );
        meta.insert("q".to_owned(), ResourceMetaValue::Integer(self.q as _));
        meta.insert(
            "lgwin".to_owned(),
            ResourceMetaValue::Integer(self.lgwin as _),
        );
        meta.insert(
            "size_before".to_owned(),
            ResourceMetaValue::Integer(src.len() as _),
        );
        meta.insert(
            "size_after".to_owned(),
            ResourceMetaValue::Integer(content.len() as _),
        );

        Ok(EncodedResource {
            meta: Some(meta),
            content,
        })
    }
}
