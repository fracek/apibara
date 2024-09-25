//! A segment is a collection of fragments from different blocks.

use rkyv::{util::AlignedVec, Archive, Deserialize, Serialize};

use crate::Cursor;

#[derive(Archive, Serialize, Deserialize, Debug)]
pub struct FragmentData<T> {
    pub cursor: Cursor,
    pub data: T,
}

#[derive(Archive, Serialize, Deserialize, Debug)]
pub struct Segment<T> {
    /// The first block in the segment.
    pub first_block: Cursor,
    /// The segment body.
    pub data: Vec<T>,
}

#[derive(Debug)]
/// A segment ready to be written to the storage.
pub struct SerializedSegment {
    pub name: String,
    pub data: AlignedVec,
}
