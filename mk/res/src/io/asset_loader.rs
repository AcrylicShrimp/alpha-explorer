use crate::io::read_file_all;
use crate::{chunk_to_filename, Resource};
use aes::cipher::generic_array::GenericArray;
use aes::cipher::{FromBlockCipher, NewBlockCipher, StreamCipher, StreamCipherSeek};
use aes::{Aes256, Aes256Ctr};
use argon2::{hash_raw, Config, Error as Argon2Error};
use downcast_rs::{impl_downcast, Downcast};
use std::collections::HashMap;
use std::error::Error;
use std::fs::OpenOptions;
use std::io::Error as IOError;
use std::path::{Path, PathBuf};
use std::sync::Arc;

#[derive(Debug)]
pub enum ResourceLoadError {
    UnknownResourceType,
    IOError(IOError),
    DecoderError(DecoderError),
    BasePathNotFound(IOError),
    CannotOpenResourceFile(IOError),
    KeySaltHashError(Argon2Error),
}

impl From<IOError> for ResourceLoadError {
    fn from(err: IOError) -> Self {
        Self::IOError(err)
    }
}

impl From<DecoderError> for ResourceLoadError {
    fn from(err: DecoderError) -> Self {
        Self::DecoderError(err)
    }
}

pub struct ResourceLoader {
    key: Vec<u8>,
    nonce: Vec<u8>,
    base_path: PathBuf,
    decoders: HashMap<String, Box<dyn ResourceDecoder>>,
}

impl ResourceLoader {
    pub fn new(
        key: impl AsRef<[u8]>,
        salt: impl AsRef<[u8]>,
        base_path: impl AsRef<Path>,
    ) -> Result<Self, ResourceLoadError> {
        let base_path = base_path
            .as_ref()
            .canonicalize()
            .map_err(|err| ResourceLoadError::BasePathNotFound(err))?;
        let first_chunk = OpenOptions::new()
            .read(true)
            .open(base_path.join(chunk_to_filename(0)))?;
        let key_nonce = &mut [0u8; 48];
        read_file_all(&first_chunk, 0, key_nonce)?;

        let mut config = Config::default();
        config.hash_length = 48;
        let hash = hash_raw(key.as_ref(), salt.as_ref(), &config)
            .map_err(|err| ResourceLoadError::KeySaltHashError(err))?;

        key_nonce[0] ^= hash[0];
        key_nonce[1] ^= hash[1];
        key_nonce[2] ^= hash[2];
        key_nonce[3] ^= hash[3];
        key_nonce[4] ^= hash[4];
        key_nonce[5] ^= hash[5];
        key_nonce[6] ^= hash[6];
        key_nonce[7] ^= hash[7];
        key_nonce[8] ^= hash[8];
        key_nonce[9] ^= hash[9];
        key_nonce[10] ^= hash[10];
        key_nonce[11] ^= hash[11];
        key_nonce[12] ^= hash[12];
        key_nonce[13] ^= hash[13];
        key_nonce[14] ^= hash[14];
        key_nonce[15] ^= hash[15];
        key_nonce[16] ^= hash[16];
        key_nonce[17] ^= hash[17];
        key_nonce[18] ^= hash[18];
        key_nonce[19] ^= hash[19];
        key_nonce[20] ^= hash[20];
        key_nonce[21] ^= hash[21];
        key_nonce[22] ^= hash[22];
        key_nonce[23] ^= hash[23];
        key_nonce[24] ^= hash[24];
        key_nonce[25] ^= hash[25];
        key_nonce[26] ^= hash[26];
        key_nonce[27] ^= hash[27];
        key_nonce[28] ^= hash[28];
        key_nonce[29] ^= hash[29];
        key_nonce[30] ^= hash[30];
        key_nonce[31] ^= hash[31];
        key_nonce[32] ^= hash[32];
        key_nonce[33] ^= hash[33];
        key_nonce[34] ^= hash[34];
        key_nonce[35] ^= hash[35];
        key_nonce[36] ^= hash[36];
        key_nonce[37] ^= hash[37];
        key_nonce[38] ^= hash[38];
        key_nonce[39] ^= hash[39];
        key_nonce[40] ^= hash[40];
        key_nonce[41] ^= hash[41];
        key_nonce[42] ^= hash[42];
        key_nonce[43] ^= hash[43];
        key_nonce[44] ^= hash[44];
        key_nonce[45] ^= hash[45];
        key_nonce[46] ^= hash[46];
        key_nonce[47] ^= hash[47];

        Ok(Self {
            key: key_nonce[..32].to_vec(),
            nonce: key_nonce[32..].to_vec(),
            base_path,
            decoders: HashMap::new(),
        })
    }

    pub fn add_decoder(&mut self, decoder: Box<dyn ResourceDecoder>) {
        self.decoders.insert(decoder.ty().to_owned(), decoder);
    }

    pub fn load(&self, res: &Resource) -> Result<Arc<dyn BaseResource>, ResourceLoadError> {
        let decoder = self
            .decoders
            .get(&res.ty)
            .ok_or(ResourceLoadError::UnknownResourceType)?;

        let mut read = 0;
        let mut content = vec![0u8; res.size as usize];

        for chunk in &res.chunks {
            let chunk_path = self.base_path.join(chunk_to_filename(chunk.id));
            let chunk_file = OpenOptions::new()
                .read(true)
                .open(chunk_path)
                .map_err(|err| ResourceLoadError::CannotOpenResourceFile(err))?;
            read_file_all(
                &chunk_file,
                chunk.offset,
                &mut content[read..read + chunk.size as usize],
            )?;
            read += chunk.size as usize;
        }

        let mut cipher = Aes256Ctr::from_block_cipher(
            Aes256::new(GenericArray::from_slice(&self.key)),
            GenericArray::from_slice(&self.nonce),
        );
        cipher.seek(res.cipher_offset);
        cipher.apply_keystream(&mut content);

        Ok(decoder.decode(content)?)
    }
}

pub type DecoderError = Box<dyn Error>;

pub trait ResourceDecoder {
    fn ty(&self) -> &str;
    fn decode(&self, content: Vec<u8>) -> Result<Arc<dyn BaseResource>, DecoderError>;
}

pub trait BaseResource: Downcast {
    fn ty(&self) -> &str;
}

impl_downcast!(BaseResource);
