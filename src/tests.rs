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
