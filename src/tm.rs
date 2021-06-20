//! Defines types to describe a TM.

use std::fmt;

use crate::tape::CellValue;


/// An N-state turing machine operating on a binary tape.
///
/// This type can only represent TMs up to N=6 states.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Tm<const N: usize> {
    /// Every state is encoded using 10 bits, starting with the least
    /// significant bits. I.e. the N * 10 least significant bits are used. All
    /// remaining (upper) bits are unused but have to be 0. This also trivially
    /// means that only TMs up to N=6 are representable.
    pub encoded: u64,
}

impl<const N: usize> Tm<N> {
    pub fn from_id(id: u64) -> Option<Self> {
        // Check for too many states
        if (id >> (N * 10)) != 0 {
            return None;
        }

        // Make sure all actions transition to an actual state.
        for action in (0..2 * N).map(|i| (id >> (5 * i)) & 0b11111) {
            if (action >> 2) > N as u64 {
                return None;
            }
        }

        Some(Self::new_unchecked(id))
    }

    pub fn new_unchecked(encoded: u64) -> Self {
        Self {
            encoded,
        }
    }

    pub fn start_action(self) -> Action<N> {
        self.state(0).on_0()
    }

    pub fn state(self, index: u8) -> State<N> {
        debug_assert!(index < N as u8, "index out of bounds!");
        State {
            encoded: (self.encoded >> (index * 10)) as u16 & 0b11111_11111
        }
    }
}

impl<const N: usize> fmt::Debug for Tm<N> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        struct Key(char);
        impl fmt::Debug for Key {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        write!(f, "Tm ")?;
        f.debug_map()
            .entries((0..N as u8).map(|i| (Key(state_name::<N>(i)), self.state(i))))
            .finish()
    }
}


/// A state of a TM.
#[derive(Clone, Copy)]
pub struct State<const N: usize> {
    /// Only the lower 10 bits are used. The lowest 5 bits are encoding the
    /// action if a 0 is read from the tape, the next 5 for when a 1 is read.
    /// The upper 6 bits have to be 0!
    encoded: u16,
}

impl<const N: usize> State<N> {
    pub fn on_0(self) -> Action<N> {
        Action {
            encoded: self.encoded as u8 & 0b11111,
        }
    }

    pub fn on_1(self) -> Action<N> {
        Action {
            encoded: (self.encoded >> 5) as u8,
        }
    }

    /// Returns the action for the given cell value.
    pub fn action_for(self, value: CellValue) -> Action<N> {
        match value.0 {
            false => self.on_0(),
            true => self.on_1(),
        }
    }
}

impl<const N: usize> fmt::Debug for State<N> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ 0 → {:?}, 1 → {:?} }}", self.on_0(), self.on_1())
    }
}


/// The `next_state` value of an action.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NextState {
    /// The special halting state.
    HaltState,
    /// A normal state defined by the index.
    State(u8),
}

/// Everything that happens in one step of simulation.
#[derive(Clone, Copy)]
pub struct Action<const N: usize> {
    /// This encodes the full action like follows:
    /// - Bit 0: the inverted value that is written to the tape, i.e. if this
    ///   bit is 0, 1 is written to the tape, and vica versa.
    /// - Bit 1: the head movement, where 0 is left and 1 is right
    /// - Bit 2-5: a four bit number <= N representing the next state. If this
    ///   is N, this action will transition to the halt state.
    /// - Bit 6 and 7 have to be 0!
    encoded: u8,
}

impl<const N: usize> Action<N> {
    /// The next state value of this transition.
    pub fn next_state(&self) -> NextState {
        let v = self.encoded >> 2;
        if v == N as u8 {
            NextState::HaltState
        } else {
            NextState::State(v)
        }
    }

    /// The value that is written to the tape.
    pub fn write_value(&self) -> CellValue {
        CellValue((self.encoded & 1) == 0)
    }

    /// How the reading/writing head moves.
    pub fn movement(&self) -> Move {
        if (self.encoded & 0b10) == 0 {
            Move::Left
        } else {
            Move::Right
        }
    }

    /// Returns `true` if the next state is the halt state.
    pub fn will_halt(&self) -> bool {
        (self.encoded >> 2) == N as u8
    }
}

impl<const N: usize> fmt::Debug for Action<N> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let direction = if self.movement() == Move::Left { "l" } else { "r" };
        let write = if self.write_value().0 { "1" } else { "0" };
        let state = state_name::<N>(self.encoded >> 2);

        write!(f, "{}{}{}", state, direction, write)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Move {
    Left,
    Right,
}

fn state_name<const N: usize>(id: u8) -> char {
    if id == N as u8 {
        'H'
    } else {
        ['A', 'B', 'C', 'D', 'E', 'F'][id as usize]
    }
}
