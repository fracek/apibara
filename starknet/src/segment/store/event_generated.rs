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
pub enum EventOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct Event<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Event<'a> {
  type Inner = Event<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> Event<'a> {
  pub const VT_FROM_ADDRESS: flatbuffers::VOffsetT = 4;
  pub const VT_KEYS: flatbuffers::VOffsetT = 6;
  pub const VT_DATA: flatbuffers::VOffsetT = 8;
  pub const VT_EVENT_INDEX: flatbuffers::VOffsetT = 10;
  pub const VT_TRANSACTION_INDEX: flatbuffers::VOffsetT = 12;
  pub const VT_TRANSACTION_HASH: flatbuffers::VOffsetT = 14;

  pub const fn get_fully_qualified_name() -> &'static str {
    "Event"
  }

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    Event { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
    args: &'args EventArgs<'args>
  ) -> flatbuffers::WIPOffset<Event<'bldr>> {
    let mut builder = EventBuilder::new(_fbb);
    builder.add_transaction_index(args.transaction_index);
    builder.add_event_index(args.event_index);
    if let Some(x) = args.transaction_hash { builder.add_transaction_hash(x); }
    if let Some(x) = args.data { builder.add_data(x); }
    if let Some(x) = args.keys { builder.add_keys(x); }
    if let Some(x) = args.from_address { builder.add_from_address(x); }
    builder.finish()
  }


  #[inline]
  pub fn from_address(&self) -> Option<&'a FieldElement> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<FieldElement>(Event::VT_FROM_ADDRESS, None)}
  }
  #[inline]
  pub fn keys(&self) -> Option<flatbuffers::Vector<'a, FieldElement>> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, FieldElement>>>(Event::VT_KEYS, None)}
  }
  #[inline]
  pub fn data(&self) -> Option<flatbuffers::Vector<'a, FieldElement>> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, FieldElement>>>(Event::VT_DATA, None)}
  }
  #[inline]
  pub fn event_index(&self) -> u64 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<u64>(Event::VT_EVENT_INDEX, Some(0)).unwrap()}
  }
  #[inline]
  pub fn transaction_index(&self) -> u64 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<u64>(Event::VT_TRANSACTION_INDEX, Some(0)).unwrap()}
  }
  #[inline]
  pub fn transaction_hash(&self) -> Option<&'a FieldElement> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<FieldElement>(Event::VT_TRANSACTION_HASH, None)}
  }
}

impl flatbuffers::Verifiable for Event<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<FieldElement>("from_address", Self::VT_FROM_ADDRESS, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, FieldElement>>>("keys", Self::VT_KEYS, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, FieldElement>>>("data", Self::VT_DATA, false)?
     .visit_field::<u64>("event_index", Self::VT_EVENT_INDEX, false)?
     .visit_field::<u64>("transaction_index", Self::VT_TRANSACTION_INDEX, false)?
     .visit_field::<FieldElement>("transaction_hash", Self::VT_TRANSACTION_HASH, false)?
     .finish();
    Ok(())
  }
}
pub struct EventArgs<'a> {
    pub from_address: Option<&'a FieldElement>,
    pub keys: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, FieldElement>>>,
    pub data: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, FieldElement>>>,
    pub event_index: u64,
    pub transaction_index: u64,
    pub transaction_hash: Option<&'a FieldElement>,
}
impl<'a> Default for EventArgs<'a> {
  #[inline]
  fn default() -> Self {
    EventArgs {
      from_address: None,
      keys: None,
      data: None,
      event_index: 0,
      transaction_index: 0,
      transaction_hash: None,
    }
  }
}

pub struct EventBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> EventBuilder<'a, 'b> {
  #[inline]
  pub fn add_from_address(&mut self, from_address: &FieldElement) {
    self.fbb_.push_slot_always::<&FieldElement>(Event::VT_FROM_ADDRESS, from_address);
  }
  #[inline]
  pub fn add_keys(&mut self, keys: flatbuffers::WIPOffset<flatbuffers::Vector<'b , FieldElement>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Event::VT_KEYS, keys);
  }
  #[inline]
  pub fn add_data(&mut self, data: flatbuffers::WIPOffset<flatbuffers::Vector<'b , FieldElement>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Event::VT_DATA, data);
  }
  #[inline]
  pub fn add_event_index(&mut self, event_index: u64) {
    self.fbb_.push_slot::<u64>(Event::VT_EVENT_INDEX, event_index, 0);
  }
  #[inline]
  pub fn add_transaction_index(&mut self, transaction_index: u64) {
    self.fbb_.push_slot::<u64>(Event::VT_TRANSACTION_INDEX, transaction_index, 0);
  }
  #[inline]
  pub fn add_transaction_hash(&mut self, transaction_hash: &FieldElement) {
    self.fbb_.push_slot_always::<&FieldElement>(Event::VT_TRANSACTION_HASH, transaction_hash);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> EventBuilder<'a, 'b> {
    let start = _fbb.start_table();
    EventBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<Event<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for Event<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("Event");
      ds.field("from_address", &self.from_address());
      ds.field("keys", &self.keys());
      ds.field("data", &self.data());
      ds.field("event_index", &self.event_index());
      ds.field("transaction_index", &self.transaction_index());
      ds.field("transaction_hash", &self.transaction_hash());
      ds.finish()
  }
}
