use crate::asset_loader::{BaseResource, DecoderError, ResourceDecoder};
use brotli::DecompressorWriter;
use std::io::Write;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TextDecoderOutput {
    pub content: Vec<u8>,
}

impl BaseResource for TextDecoderOutput {
    fn ty(&self) -> &str {
        "text"
    }
}

pub struct TextDecoder;

impl ResourceDecoder for TextDecoder {
    fn ty(&self) -> &str {
        "text"
    }

    fn decode(&self, content: Vec<u8>) -> Result<Arc<dyn BaseResource>, DecoderError> {
        let mut result = vec![];

        {
            let mut writer = DecompressorWriter::new(&mut result, 4096);
            writer.write_all(&content)?;
        }

        Ok(Arc::new(TextDecoderOutput { content: result }))
    }
}
