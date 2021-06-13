//! Defines types to describe a TM.

use std::{
    convert::TryInto,
    fmt,
    marker::PhantomData,
};
use core::arch::x86_64::__m128i;
use bytemuck::{Pod, Zeroable};

use crate::CellValue;


/// A N-state turing machine operating on a binary tape.
#[derive(Clone, Copy)]
pub struct Tm<const N: usize> {
    encoded: __m128i,
    _dummy: PhantomData<[(); N]>,
}

impl<const N: usize> Tm<N> {
    pub fn new(encoded: __m128i) -> Self {
        Self {
            encoded,
            _dummy: PhantomData,
        }
    }

    pub fn encoded(&self) -> &__m128i {
        &self.encoded
    }

    /// Returns the first transition that will be executed (`states[0].on_0`).
    pub fn start_action(&self) -> Action {
        self.states()[0].on_0
    }

    pub fn states(&self) -> [State; N] {
        let arr: &[State; 8] = bytemuck::cast_ref(&self.encoded);
        arr[..N].try_into().unwrap()
    }
}

impl<const N: usize> PartialEq for Tm<N> {
    fn eq(&self, other: &Self) -> bool {
        use core::arch::x86_64::{_mm_movemask_epi8, _mm_cmpeq_epi8};

        unsafe {
            _mm_movemask_epi8(_mm_cmpeq_epi8(self.encoded, other.encoded)) == 0xFFFF
        }
    }
}

impl<const N: usize> fmt::Debug for Tm<N> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_map()
            .entries(self.states().iter().enumerate())
            .finish()
    }
}


/// A state of a TM.
#[derive(Clone, Copy, Pod, Zeroable)]
#[repr(C)]
pub struct State {
    pub on_0: Action,
    pub on_1: Action,
}

impl State {
    /// Returns the action for the given cell value.
    pub fn action_for(&self, value: CellValue) -> Action {
        match value.0 {
            false => self.on_0,
            true => self.on_1,
        }
    }
}

impl fmt::Debug for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "State {{ 0 → {:?}, 1 → {:?} }}", self.on_0, self.on_1)
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

impl NextState {
    pub fn is_halt_state(&self) -> bool {
        *self == NextState::HaltState
    }

    pub fn as_normal_state(&self) -> Option<u8> {
        match self {
            NextState::HaltState => None,
            NextState::State(v) => Some(*v)
        }
    }
}

/// Special constant.
const HALT_STATE: u8 = 63;

/// Everything that happens in one step of simulation.
#[derive(Clone, Copy, Pod, Zeroable)]
#[repr(transparent)]
pub struct Action {
    /// This encodes the full action like follows:
    /// - Bit 0: what is written to the cell in this transition
    /// - Bit 1: the head movement, where 0 is left and 1 is right
    /// - Bit 2-7: the next state. With these 6 bits we can encode 2^6 = 64
    ///   states which is more than enough. The busy beaver problem is already
    ///   very hard for N=6.
    encoded: u8,
}

impl Action {
    /// The next state value of this transition.
    pub fn next_state(&self) -> NextState {
        let v = self.encoded >> 2;
        if v == HALT_STATE {
            NextState::HaltState
        } else {
            NextState::State(v)
        }
    }

    /// The value that is written to the tape.
    pub fn write_value(&self) -> CellValue {
        CellValue((self.encoded & 1) == 1)
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
        self.next_state().is_halt_state()
    }
}

impl fmt::Debug for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let letter = if self.movement() == Move::Left { "l" } else { "r" };
        let write = if self.write_value().0 { "1" } else { "0" };

        if self.will_halt() {
            write!(f, "{}_H", write)
        } else {
            let state_id = self.next_state().as_normal_state().unwrap();
            write!(f, "{}{}{}", write, letter, state_id)
        }
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Move {
    Left,
    Right,
}

