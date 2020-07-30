//! Defines types to describe a TM.

use std::{
    convert::TryInto,
    fmt,
    sync::Arc,
};

use crate::CellValue;


/// A N-state turing machine operating on a binary tape.
#[derive(Clone, Copy)]
pub struct Tm<const N: usize> {
    pub states: [State; N],
}

impl<const N: usize> Tm<N> {
    /// Returns the first transition that will be executed (`states[0].on_0`).
    pub fn start_action(&self) -> Action {
        self.states[0].on_0
    }
}

impl<const N: usize> fmt::Debug for Tm<N> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_map()
            .entries(self.states.iter().enumerate())
            .finish()
    }
}


/// A state of a TM.
#[derive(Clone, Copy)]
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

    pub fn actions(&self) -> impl Iterator<Item = Action> {
        use std::iter;

        iter::once(self.on_0).chain(iter::once(self.on_1))
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
#[derive(Clone, Copy)]
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
    pub fn new(next_state: NextState, write: CellValue, movement: Move) -> Self {
        let encoded_state = match next_state {
            NextState::HaltState => HALT_STATE,
            NextState::State(v) => v,
        };
        let encoded_movement = match movement {
            Move::Left => 0,
            Move::Right => 1,
        };

        let encoded = (encoded_state << 2) | encoded_movement << 1 | write.0 as u8;
        Self { encoded }
    }

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


/// Iterator that generates all combinations of TMs with `N` states.
///
/// The number of TMs is `(((N * 2 + 1) * 2)^2)^N`. For small N:
/// - 1: 36
/// - 2: 10_000
/// - 3: 7_529_536
/// - 4: 11_019_960_576
/// - 5: 26_559_922_791_424
///
/// It is fairly cheap to clone this iterator.
#[derive(Clone)]
pub struct AllTmCombinations<const N: usize> {
    // The depth is iteration state and basically determines which digits of
    // `indices` is increased next. An `i8` is sufficient as this is bounded by
    // `N` (which will never be as high as 127).
    depth: u8,

    /// This lists all possible states. This is pregenerated in `new`.
    all_states: Arc<[State]>,

    /// This is the current state of iteration. It's basically a N digit number
    /// with base `all_states.len()`. We use `u16` to have dense memory:
    /// `all_states.len()` luckily doesn't grow that fast. Even for N=20 it is
    /// way below `u16::max_value`.
    indices: [u16; N],

    /// The states that were yielded as part of a `Tm` the last time `next`
    /// was called.
    last_states: [State; N],

    /// Just counts down how many elements are still remaining. This is only
    /// for the `ExactSizeIterator` impl.
    remaining: u64,
}

impl<const N: usize> AllTmCombinations<N> {
    pub fn new() -> Self {
        // We create vectors of all X, starting with movements and state ids.
        let all_movements = [Move::Left, Move::Right];
        let all_writes = [CellValue(false), CellValue(true)];
        let all_state_ids = (0..N).map(|s| NextState::State(s as u8));

        // `all_actions` has the length (N * 2 + 1) * 2.
        let all_actions = all_state_ids.clone()
            .flat_map(|next_state| all_movements.iter().map(move |&m| (next_state, m)))
            // Add the halting state with arbitrary movement
            .chain(Some((NextState::HaltState, Move::Left)))
            .flat_map(|(next_state, movement)| {
                all_writes.iter().map(move |&write| Action::new(next_state, write, movement))
            })
            .collect::<Vec<_>>();

        // `all_states` has the length ((N * 2 + 1) * 2)^2.
        let all_states = all_actions.iter().flat_map(|&on_0| {
            all_actions.iter().map(move |&on_1| State { on_0, on_1 })
        }).collect::<Vec<_>>();


        // Sadly, we can't create this array immediately (limitation in const
        // generics). So we have to create a vector first and then convert it
        // into an array.
        let dummy_state = all_states[0];
        let last_states = vec![dummy_state; N][..].try_into().unwrap();
        let indices = vec![0; N][..].try_into().unwrap();

        let total_tms = (all_states.len() as u64).pow(N as u32);

        Self {
            all_states: all_states.into(),
            last_states,
            indices,
            depth: 0,
            remaining: total_tms,
        }
    }

    /// Splits off a new iterator that iterates over the next `num_items` TMs,
    /// while the `self` iterator is advanced by `num_items`.
    pub fn split_off(&mut self, num_items: u64) -> Self {
        // Prepare out iterator
        let mut out = self.clone();
        out.remaining = num_items;

        // Advance this iterator
        if num_items >= self.remaining {
            self.remaining = 0;
        } else {
            self.remaining -= num_items;

            // Subtract from `indices`, the base `num_states` N-digit number.
            let num_states = self.all_states.len() as u64;
            let mut subtract = num_items;
            let mut i = N - 1;
            while subtract > 0 {
                self.indices[i] += (subtract % num_states) as u16;
                if self.indices[i] >= num_states as u16 {
                    // Handle overflow: add "carry" to `subtract`
                    subtract += num_states;
                    self.indices[i] -= num_states as u16;
                }

                // Go to the next digit
                subtract /= num_states;

                // This will never underflow because we made sure `num_items <
                // self.remaining`.
                i -= 1;
            }
        }

        out
    }
}

impl<const N: usize> Iterator for AllTmCombinations<N> {
    type Item = Tm<N>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }

        loop {
            // This `unsafe` is fine: `self.indices` always has the length `N`.
            // And `self.depth` is *only* increased in this method below. And
            // that clearly only happens if `depth` is not `N - 1`. And since
            // we increment `depth` only 1 at a time, this means, `depth`
            // becomes at most `N - 1`, making this index safe.
            let idx = unsafe { self.indices.get_unchecked_mut(self.depth as usize) };
            let state_idx = *idx;
            *idx += 1;

            match self.all_states.get(state_idx as usize) {
                None => {
                    // We exhausted the current "digit" and have to track back.
                    *idx = 0;
                    self.depth -= 1;
                }

                Some(state) => {
                    // If not, we update the temporary storage and, if we are
                    // at max depth, return the current state.

                    // This unsafe call is fine. See the `get_unchecked_mut`
                    // above for more information. The same logic applies here.
                    unsafe {
                        *self.last_states.get_unchecked_mut(self.depth as usize) = *state;
                    }

                    if self.depth == N as u8 - 1 {
                        self.remaining -= 1;
                        return Some(Tm { states: self.last_states });
                    } else {
                        self.depth += 1;
                    }
                }
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remaining as usize, Some(self.remaining as usize))
    }
}

impl<const N: usize> ExactSizeIterator for AllTmCombinations<N> {}
