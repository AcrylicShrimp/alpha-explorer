use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::num::NonZeroU64;

#[derive(Serialize, Deserialize, Debug, Clone, Hash)]
pub struct ResourcesMeta {
    pub version: u32,
    pub resources: Vec<Resource>,
    pub resource_names: BTreeMap<String, usize>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Hash)]
pub struct Resource {
    pub uuid: ResourceUUID,
    pub name: String,
    pub ty: String,
    pub hash: ResourceHash,
    pub cipher_offset: u64,
    pub size: u64,
    pub chunks: Vec<ResourceChunk>,
    pub meta: Option<ResourceMeta>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ResourceHash {
    pub hash: String,
    pub algorithm: ResourceHashAlgorithm,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum ResourceHashAlgorithm {
    CRC32LESHA256,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ResourceChunk {
    pub id: ResourceChunkID,
    pub offset: u64,
    pub size: u64,
}

pub type ResourceChunkID = u32;

pub type ResourceUUID = NonZeroU64;

pub type ResourceMeta = BTreeMap<String, ResourceMetaValue>;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum ResourceMetaValue {
    Boolean(bool),
    Integer(i64),
    String(String),
}
