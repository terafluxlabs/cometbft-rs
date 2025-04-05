// This file is @generated by prost-build.
/// Message is the top-level message type for the statesync service.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Message {
    /// The message type.
    #[prost(oneof = "message::Sum", tags = "1, 2, 3, 4")]
    pub sum: ::core::option::Option<message::Sum>,
}
/// Nested message and enum types in `Message`.
pub mod message {
    /// The message type.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Sum {
        #[prost(message, tag = "1")]
        SnapshotsRequest(super::SnapshotsRequest),
        #[prost(message, tag = "2")]
        SnapshotsResponse(super::SnapshotsResponse),
        #[prost(message, tag = "3")]
        ChunkRequest(super::ChunkRequest),
        #[prost(message, tag = "4")]
        ChunkResponse(super::ChunkResponse),
    }
}
/// SnapshotsRequest is sent to request a snapshot.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SnapshotsRequest {}
/// SnapshotsResponse contains the snapshot metadata.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SnapshotsResponse {
    #[prost(uint64, tag = "1")]
    pub height: u64,
    #[prost(uint32, tag = "2")]
    pub format: u32,
    #[prost(uint32, tag = "3")]
    pub chunks: u32,
    #[prost(bytes = "vec", tag = "4")]
    pub hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "5")]
    pub metadata: ::prost::alloc::vec::Vec<u8>,
}
/// ChunkRequest is sent to request a chunk.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChunkRequest {
    #[prost(uint64, tag = "1")]
    pub height: u64,
    #[prost(uint32, tag = "2")]
    pub format: u32,
    #[prost(uint32, tag = "3")]
    pub index: u32,
}
/// ChunkResponse contains a chunk of the snapshot.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChunkResponse {
    #[prost(uint64, tag = "1")]
    pub height: u64,
    #[prost(uint32, tag = "2")]
    pub format: u32,
    #[prost(uint32, tag = "3")]
    pub index: u32,
    #[prost(bytes = "vec", tag = "4")]
    pub chunk: ::prost::alloc::vec::Vec<u8>,
    #[prost(bool, tag = "5")]
    pub missing: bool,
}
