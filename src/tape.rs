//! Defines the tape on which TMs are operating.

use std::{
    cmp::max,
    mem,
    ops::Range,
};





/// The index of a cell. All TM start with `0` as the active cell. This type
/// exists to use strong typing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct CellId(pub i64);

/// The binary value of a cell. This type exists to use strong typing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CellValue(pub bool);


type BucketType = u64;
const BITS_PER_BUCKET: usize = mem::size_of::<BucketType>() * 8;

/// The infinite tape of a TM. The cells are binary and can thus hold the
/// values '0' or '1'. All cells are initialized to 0.
pub struct Tape {
    /// The bits stored on the tape. This functions as a bit vector.
    data: Vec<BucketType>,

    /// The initial cell of the TM (cell 0) is stored at bit `offset` of the
    /// `data` vector. This number is always positive, but we store a `i64`
    /// instead of `u64` because the number also has to fit into that datatype.
    offset: i64,

    /// The range outside of which nothing was every written. As invariant,
    /// this range must be completely represented by `data`. In other words:
    /// - `written.range.start + offset >= 0`
    /// - `written.range.end + offset < data.len() * BITS_PER_BUCKET`
    written_range: Range<CellId>,
}

impl Tape {
    /// Creates a new infinite tape.
    pub fn new() -> Self {
        Self {
            data: vec![0],
            offset: 32,
            written_range: CellId(0)..CellId(0),
        }
    }

    /// Returns the range in which cells have already been written. Not all
    /// cells are written within this range, but there are no cells outside
    /// this range that have not been written to yet.
    pub fn written_range(&self) -> Range<CellId> {
        self.written_range.clone()
    }

    /// Return the value of the given cell.
    pub fn get(&self, id: CellId) -> CellValue {
        // If a cell is requested outside the range that has ever be written
        // to, we know its a binary 0.
        if !self.written_range.contains(&id) {
            return CellValue(false);
        }

        // We can cast because the `written_range` invariant guarantees the
        // result is >= 0.
        let (bucket_idx, bit_in_bucket) = self.lookup_bucket(id);

        CellValue((self.data[bucket_idx] & (1 << bit_in_bucket)) != 0)
    }

    /// Write a new value into the given cell.
    pub fn write(&mut self, id: CellId, value: CellValue) {
        let bit_idx = self.offset + id.0;
        let stored_bits = self.data.len() * BITS_PER_BUCKET;
        let grow_by_bits = match () {
            // Just add 2 for some extra room.
            () if bit_idx < 0 => Some(-bit_idx as usize + 2),
            () if bit_idx as usize >= stored_bits => Some(bit_idx as usize - stored_bits + 2),
            () => None,
        };

        // Check if we need to grow.
        if let Some(grow_by_bits) = grow_by_bits {
            // Make sure we at least double our capacity to avoid repeated
            // reallocations.
            let grow_by_bits = max(grow_by_bits, stored_bits);

            // Add 1 to compensate for rounding down of integer division.
            let grow_by_buckets = (grow_by_bits / BITS_PER_BUCKET) + 1;

            let mut new_data = vec![0; self.data.len() + grow_by_buckets];

            if bit_idx < 0 {
                // We grew left
                new_data[grow_by_buckets..grow_by_buckets + self.data.len()]
                    .copy_from_slice(&self.data);
                self.offset += (grow_by_buckets * BITS_PER_BUCKET) as i64;
            } else {
                // We grew right
                new_data[0..self.data.len()].copy_from_slice(&self.data);
            }

            self.data = new_data;
        }

        // At this point we know that the bit we want to access is actually
        // stored in `data`.
        let (bucket_idx, bit_in_bucket) = self.lookup_bucket(id);
        if value.0 {
            self.data[bucket_idx] |= 1 << bit_in_bucket;
        } else {
            self.data[bucket_idx] &= !(1 << bit_in_bucket);
        }

        // Adjust `written_range`
        if self.written_range.start > id {
            self.written_range.start = id;
        }
        if self.written_range.end <= id {
            self.written_range.end = CellId(id.0 + 1);
        }
    }

    fn lookup_bucket(&self, id: CellId) -> (usize, usize) {
        let bit_idx = (self.offset + id.0) as usize;
        let bucket_idx = bit_idx / BITS_PER_BUCKET;
        let bit_in_bucket = bit_idx % BITS_PER_BUCKET;

        (bucket_idx, bit_in_bucket)
    }
}



#[cfg(test)]
mod tests {
    use super::{CellId, CellValue, Tape};


    #[test]
    fn empty_tape() {
        let tape = Tape::new();

        for i in -200..200 {
            assert_eq!(tape.get(CellId(i)), CellValue(false), "at cell {}", i);
        }

        assert_eq!(tape.get(CellId(-123_456)), CellValue(false));
        assert_eq!(tape.get(CellId(8_764_243)), CellValue(false));
        assert_eq!(tape.written_range(), CellId(0)..CellId(0));
    }

    #[test]
    fn write_at_0() {
        let mut tape = Tape::new();

        tape.write(CellId(0), CellValue(false));
        assert_eq!(tape.written_range(), CellId(0)..CellId(1));
        for i in -200..200 {
            assert_eq!(tape.get(CellId(i)), CellValue(false), "at cell {}", i);
        }

        tape.write(CellId(0), CellValue(true));
        assert_eq!(tape.written_range(), CellId(0)..CellId(1));
        assert_eq!(tape.get(CellId(0)), CellValue(true));
        for i in (-200..0).chain(1..200) {
            assert_eq!(tape.get(CellId(i)), CellValue(false), "at cell {}", i);
        }
    }

    #[test]
    fn write_far_away() {
        let mut tape = Tape::new();

        tape.write(CellId(10), CellValue(true));
        assert_eq!(tape.written_range(), CellId(0)..CellId(11));
        assert_eq!(tape.get(CellId(10)), CellValue(true));
        for i in (-200..10).chain(11..200) {
            assert_eq!(tape.get(CellId(i)), CellValue(false), "at cell {}", i);
        }

        tape.write(CellId(-5), CellValue(true));
        assert_eq!(tape.written_range(), CellId(-5)..CellId(11));
        assert_eq!(tape.get(CellId(10)), CellValue(true));
        assert_eq!(tape.get(CellId(-5)), CellValue(true));
        for i in (-200..-5).chain(-4..10).chain(11..200) {
            assert_eq!(tape.get(CellId(i)), CellValue(false), "at cell {}", i);
        }

        tape.write(CellId(-4_321), CellValue(true));
        assert_eq!(tape.written_range(), CellId(-4_321)..CellId(11));
        assert_eq!(tape.get(CellId(10)), CellValue(true));
        assert_eq!(tape.get(CellId(-5)), CellValue(true));
        assert_eq!(tape.get(CellId(-4_321)), CellValue(true));
        for i in (-6_000..-4_321).chain(-4320..-5).chain(-4..10).chain(11..6_000) {
            assert_eq!(tape.get(CellId(i)), CellValue(false), "at cell {}", i);
        }

        tape.write(CellId(56_789), CellValue(true));
        assert_eq!(tape.written_range(), CellId(-4_321)..CellId(56_789 + 1));
        assert_eq!(tape.get(CellId(10)), CellValue(true));
        assert_eq!(tape.get(CellId(-5)), CellValue(true));
        assert_eq!(tape.get(CellId(-4_321)), CellValue(true));
        assert_eq!(tape.get(CellId(56_789)), CellValue(true));
        for i in (-100_000..-4_321)
            .chain(-4320..-5)
            .chain(-4..10)
            .chain(11..56_789)
            .chain(56_789 + 1..100_000)
        {
            assert_eq!(tape.get(CellId(i)), CellValue(false), "at cell {}", i);
        }
    }
}
