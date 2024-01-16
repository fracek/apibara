// automatically generated by the FlatBuffers compiler, do not modify
// @generated
extern crate alloc;
extern crate flatbuffers;
use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::mem;
use core::cmp::Ordering;
use self::flatbuffers::{EndianScalar, Follow};
use super::*;
// struct FieldElement, aligned to 1
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq)]
pub struct FieldElement(pub [u8; 32]);
impl Default for FieldElement { 
  fn default() -> Self { 
    Self([0; 32])
  }
}
impl core::fmt::Debug for FieldElement {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    f.debug_struct("FieldElement")
      .field("value", &self.value())
      .finish()
  }
}

impl flatbuffers::SimpleToVerifyInSlice for FieldElement {}
impl<'a> flatbuffers::Follow<'a> for FieldElement {
  type Inner = &'a FieldElement;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    <&'a FieldElement>::follow(buf, loc)
  }
}
impl<'a> flatbuffers::Follow<'a> for &'a FieldElement {
  type Inner = &'a FieldElement;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    flatbuffers::follow_cast_ref::<FieldElement>(buf, loc)
  }
}
impl<'b> flatbuffers::Push for FieldElement {
    type Output = FieldElement;
    #[inline]
    unsafe fn push(&self, dst: &mut [u8], _written_len: usize) {
        let src = ::core::slice::from_raw_parts(self as *const FieldElement as *const u8, Self::size());
        dst.copy_from_slice(src);
    }
}

impl<'a> flatbuffers::Verifiable for FieldElement {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.in_buffer::<Self>(pos)
  }
}

impl<'a> FieldElement {
  #[allow(clippy::too_many_arguments)]
  pub fn new(
    value: &[u8; 32],
  ) -> Self {
    let mut s = Self([0; 32]);
    s.set_value(value);
    s
  }

  pub const fn get_fully_qualified_name() -> &'static str {
    "FieldElement"
  }

  pub fn value(&'a self) -> flatbuffers::Array<'a, u8, 32> {
    // Safety:
    // Created from a valid Table for this object
    // Which contains a valid array in this slot
    unsafe { flatbuffers::Array::follow(&self.0, 0) }
  }

  pub fn set_value(&mut self, items: &[u8; 32]) {
    // Safety:
    // Created from a valid Table for this object
    // Which contains a valid array in this slot
    unsafe { flatbuffers::emplace_scalar_array(&mut self.0, 0, items) };
  }

}
