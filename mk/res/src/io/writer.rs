use crate::{
    chunk_to_filename, Resource, ResourceChunk, ResourceHash, ResourceHashAlgorithm, ResourceMeta,
    ResourceUUID, ResourcesMeta,
};
use aes::cipher::generic_array::GenericArray;
use aes::cipher::{FromBlockCipher, NewBlockCipher, StreamCipher};
use aes::{Aes256, Aes256Ctr};
use argon2::{hash_raw, Config, Error as Argon2Error};
use byteorder::{ByteOrder, LittleEndian};
use crc32fast::Hasher as Crc32Hasher;
use memmap2::{Mmap, MmapOptions};
use rand::prelude::*;
use rand::{Error as RandError, Fill};
use rayon::prelude::*;
use sha256::digest_bytes as sha256_digest_bytes;
use std::cmp::min;
use std::collections::HashMap;
use std::error::Error;
use std::fs::{create_dir_all, metadata, remove_dir_all, File, OpenOptions};
use std::io::{Error as IOError, ErrorKind as IOErrorKind};
use std::mem::size_of;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub enum ResourceWriteError {
    UnknownResourceType,
    CannotPrepareDirectory(IOError),
    CannotOpenResourceFile(IOError),
    CannotMapResourceFile(IOError),
    CannotCreateChunkFile(IOError),
    CannotMapChunkFile(IOError),
    CannotCleanupTempDirectory(IOError),
    CipherKeyGenError(RandError),
    KeySaltHashError(Argon2Error),
    EncoderError(EncoderError),
}

impl From<EncoderError> for ResourceWriteError {
    fn from(err: EncoderError) -> Self {
        Self::EncoderError(err.into())
    }
}

pub struct WritingResource<'a> {
    pub name: &'a str,
    pub ty: &'a str,
    pub path: &'a Path,
}

#[derive(Default)]
pub struct ResourceWriter {
    encoders: HashMap<String, Box<dyn ResourceEncoder>>,
}

impl ResourceWriter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_encoder(&mut self, ty: String, encoder: Box<dyn ResourceEncoder>) {
        self.encoders.insert(ty, encoder);
    }

    pub fn write<'a>(
        &self,
        key: impl AsRef<[u8]>,
        salt: impl AsRef<[u8]>,
        chunk_size: Option<u64>,
        base_path: impl AsRef<Path>,
        res: impl AsRef<[WritingResource<'a>]>,
    ) -> Result<ResourcesMeta, ResourceWriteError> {
        let dir_mgr = ResourceEncoderDirectoryManagerImpl::from_base_dir(base_path.as_ref())
            .map_err(|err| ResourceWriteError::CannotPrepareDirectory(err))?;
        let resources = res
            .as_ref()
            .par_iter()
            .enumerate()
            .map(|(index, res)| {
                let encoder = self
                    .encoders
                    .get(res.ty)
                    .ok_or(ResourceWriteError::UnknownResourceType)?;

                let file = OpenOptions::new()
                    .read(true)
                    .open(res.path)
                    .map_err(|err| ResourceWriteError::CannotOpenResourceFile(err))?;
                let content = unsafe { Mmap::map(&file) }
                    .map_err(|err| ResourceWriteError::CannotMapResourceFile(err))?;

                let uuid = unsafe { ResourceUUID::new_unchecked((index + 1) as u64) };
                let hash = ResourceHash {
                    hash: {
                        let mut hasher = Crc32Hasher::new();
                        hasher.update(&content);

                        let buffer = &mut [0u8; size_of::<u32>()];
                        LittleEndian::write_u32(buffer, hasher.finalize());

                        sha256_digest_bytes(buffer)
                    },
                    algorithm: ResourceHashAlgorithm::CRC32LESHA256,
                };
                let encoded = encoder.encode(&dir_mgr, uuid, content)?;

                Ok((uuid, hash, encoded))
            })
            .collect::<Result<Vec<_>, ResourceWriteError>>()?;

        let mut config = Config::default();
        config.hash_length = 48;
        let hash = hash_raw(key.as_ref(), salt.as_ref(), &config)
            .map_err(|err| ResourceWriteError::KeySaltHashError(err))?;

        let key = &mut [0u8; 32];
        let nonce = &mut [0u8; 16];

        {
            let mut rng = thread_rng();
            key.try_fill(&mut rng)
                .map_err(|err| ResourceWriteError::CipherKeyGenError(err))?;
            nonce
                .try_fill(&mut rng)
                .map_err(|err| ResourceWriteError::CipherKeyGenError(err))?;
        }

        let secure_key = &mut key.clone();
        let secure_nonce = &mut nonce.clone();

        secure_key[0] ^= hash[0];
        secure_key[1] ^= hash[1];
        secure_key[2] ^= hash[2];
        secure_key[3] ^= hash[3];
        secure_key[4] ^= hash[4];
        secure_key[5] ^= hash[5];
        secure_key[6] ^= hash[6];
        secure_key[7] ^= hash[7];
        secure_key[8] ^= hash[8];
        secure_key[9] ^= hash[9];
        secure_key[10] ^= hash[10];
        secure_key[11] ^= hash[11];
        secure_key[12] ^= hash[12];
        secure_key[13] ^= hash[13];
        secure_key[14] ^= hash[14];
        secure_key[15] ^= hash[15];
        secure_key[16] ^= hash[16];
        secure_key[17] ^= hash[17];
        secure_key[18] ^= hash[18];
        secure_key[19] ^= hash[19];
        secure_key[20] ^= hash[20];
        secure_key[21] ^= hash[21];
        secure_key[22] ^= hash[22];
        secure_key[23] ^= hash[23];
        secure_key[24] ^= hash[24];
        secure_key[25] ^= hash[25];
        secure_key[26] ^= hash[26];
        secure_key[27] ^= hash[27];
        secure_key[28] ^= hash[28];
        secure_key[29] ^= hash[29];
        secure_key[30] ^= hash[30];
        secure_key[31] ^= hash[31];

        secure_nonce[0] ^= hash[32];
        secure_nonce[1] ^= hash[33];
        secure_nonce[2] ^= hash[34];
        secure_nonce[3] ^= hash[35];
        secure_nonce[4] ^= hash[36];
        secure_nonce[5] ^= hash[37];
        secure_nonce[6] ^= hash[38];
        secure_nonce[7] ^= hash[39];
        secure_nonce[8] ^= hash[40];
        secure_nonce[9] ^= hash[41];
        secure_nonce[10] ^= hash[42];
        secure_nonce[11] ^= hash[43];
        secure_nonce[12] ^= hash[44];
        secure_nonce[13] ^= hash[45];
        secure_nonce[14] ^= hash[46];
        secure_nonce[15] ^= hash[47];

        let mut cipher = Aes256Ctr::from_block_cipher(
            Aes256::new(GenericArray::from_slice(key)),
            GenericArray::from_slice(nonce),
        );

        let mut chunk = 0;
        let mut chunk_offset = 0;
        let mut total_written = 0;

        let total_size = {
            let mut size = 48;

            for (_, _, encoded) in resources.iter() {
                size += encoded.content.len() as u64;
            }

            size
        };
        let chunk_size = {
            let chunk_size = chunk_size.unwrap_or(0);

            if chunk_size == 0 {
                total_size
            } else {
                chunk_size
            }
        };

        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(base_path.as_ref().join(chunk_to_filename(chunk)))
            .map_err(|err| ResourceWriteError::CannotCreateChunkFile(err))?;
        file.set_len(chunk_size)
            .map_err(|err| ResourceWriteError::CannotCreateChunkFile(err))?;
        let mut chunk_content = unsafe { MmapOptions::new().map_mut(&file) }
            .map_err(|err| ResourceWriteError::CannotMapChunkFile(err))?;

        let mut write_to_chunk =
            |content: &[u8],
             apply_cipher: bool|
             -> Result<((u64, u64), Vec<ResourceChunk>), ResourceWriteError> {
                let mut chunks = vec![];
                let mut content_offset = 0;

                while content_offset < content.len() {
                    if chunk_offset == chunk_size {
                        chunk += 1;
                        chunk_offset = 0;
                        file = OpenOptions::new()
                            .read(true)
                            .write(true)
                            .create(true)
                            .truncate(true)
                            .open(base_path.as_ref().join(chunk_to_filename(chunk)))
                            .map_err(|err| ResourceWriteError::CannotCreateChunkFile(err))?;
                        file.set_len(min(total_size - chunk as u64 * chunk_size, chunk_size))
                            .map_err(|err| ResourceWriteError::CannotCreateChunkFile(err))?;
                        chunk_content = unsafe { MmapOptions::new().map_mut(&file) }
                            .map_err(|err| ResourceWriteError::CannotMapChunkFile(err))?;
                    }

                    let len = min(
                        content.len() - content_offset,
                        (chunk_size - chunk_offset) as usize,
                    );
                    let range = &mut chunk_content[{
                        let chunk_offset = chunk_offset as usize;
                        chunk_offset..chunk_offset + len
                    }];

                    range.copy_from_slice(&content[..len]);

                    if apply_cipher {
                        cipher.apply_keystream(range);
                    }

                    chunks.push(ResourceChunk {
                        id: chunk,
                        offset: chunk_offset,
                        size: len as _,
                    });
                    chunk_offset += len as u64;
                    content_offset += len;
                }

                let cipher_offset_size = (total_written, content.len() as u64);

                if apply_cipher {
                    total_written += cipher_offset_size.1;
                }

                Ok((cipher_offset_size, chunks))
            };

        write_to_chunk(secure_key, false)?;
        write_to_chunk(secure_nonce, false)?;

        let resources = resources
            .into_iter()
            .enumerate()
            .map(|(index, (uuid, hash, encoded))| {
                let ((cipher_offset, size), chunks) = write_to_chunk(&encoded.content, true)?;
                Ok(Resource {
                    uuid,
                    name: res.as_ref()[index].name.to_owned(),
                    ty: res.as_ref()[index].ty.to_owned(),
                    hash,
                    cipher_offset,
                    size,
                    chunks,
                    meta: encoded.meta,
                })
            })
            .collect::<Result<Vec<_>, ResourceWriteError>>()?;
        let resource_names = resources
            .iter()
            .enumerate()
            .map(|(index, res)| (res.name.clone(), index))
            .collect();
        let meta = ResourcesMeta {
            version: 1,
            resources,
            resource_names,
        };

        dir_mgr
            .remove_tmp_dir()
            .map_err(|err| ResourceWriteError::CannotCleanupTempDirectory(err))?;
        Ok(meta)
    }
}

pub trait ResourceEncoder: Send + Sync {
    fn ty(&self) -> &str;
    fn encode(
        &self,
        dir_mgr: &dyn ResourceEncoderDirectoryManager,
        uuid: ResourceUUID,
        src: Mmap,
    ) -> Result<EncodedResource, EncoderError>;
}

pub trait ResourceEncoderDirectoryManager {
    fn alloc_tmp_file(&self, uuid: ResourceUUID) -> Result<File, IOError>;
}

pub struct ResourceEncoderDirectoryManagerImpl {
    base_dir: PathBuf,
    base_tmp_dir: PathBuf,
}

impl ResourceEncoderDirectoryManagerImpl {
    pub fn from_base_dir(base_dir: &Path) -> Result<Self, IOError> {
        if let Ok(meta) = metadata(base_dir) {
            if meta.is_dir() {
                remove_dir_all(base_dir)?;
            } else {
                return Err(IOError::from(IOErrorKind::AlreadyExists));
            }
        }

        create_dir_all(base_dir)?;

        let base_dir = base_dir.canonicalize()?;
        let base_tmp_dir = base_dir.join(".tmp");

        create_dir_all(&base_tmp_dir)?;

        Ok(Self {
            base_dir,
            base_tmp_dir,
        })
    }

    pub fn base_dir(&self) -> &Path {
        &self.base_dir
    }

    pub fn base_tmp_dir(&self) -> &Path {
        &self.base_tmp_dir
    }

    pub fn remove_tmp_dir(&self) -> Result<(), IOError> {
        remove_dir_all(&self.base_tmp_dir)?;
        Ok(())
    }
}

impl ResourceEncoderDirectoryManager for ResourceEncoderDirectoryManagerImpl {
    fn alloc_tmp_file(&self, uuid: ResourceUUID) -> Result<File, IOError> {
        OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(self.base_tmp_dir.join(format!("{}.tmp", uuid)))
    }
}

pub type EncoderError = Box<dyn Error + Send + Sync>;

#[derive(Debug)]
pub struct EncodedResource {
    pub meta: Option<ResourceMeta>,
    pub content: Mmap,
}
