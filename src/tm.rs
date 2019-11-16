//! Defines types to describe a TM.

use std::{
    array::LengthAtMost32,
    convert::TryInto,
    fmt,
};

use crate::CellValue;


/// A N-state turing machine operating on a binary tape.
#[derive(Clone, Copy)]
pub struct Tm<const N: usize> {
    pub states: [State; N],
}

impl<const N: usize> fmt::Debug for Tm<{N}> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_map()
            .entries(self.states.iter().enumerate())
            .finish()
    }
}

/// Special constant.
pub const HALT_STATE: u8 = u8::max_value();

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

/// Everything that happens in one step of simulation.
#[derive(Clone, Copy)]
pub struct Action {
    /// As the busy beaver problem is almost basically impossible for N=6, `u8`
    /// able to refer to 256 states is more than enough. `255` is the halting
    /// state.
    pub next_state: u8,

    /// The value that is written to the tape.
    pub write: CellValue,

    /// How the reading/writing head moves.
    pub movement: Move,
}

impl Action {
    pub fn will_halt(&self) -> bool {
        self.next_state == HALT_STATE
    }
}

impl fmt::Debug for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let letter = if self.movement == Move::Left { "l" } else { "r" };
        let write = if self.write.0 { "1" } else { "0" };

        if self.will_halt() {
            write!(f, " {}H", write)
        } else {
            write!(f, "{}{}{}", write, letter, self.next_state)
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
pub struct AllTmCombinations<const N: usize> {
    /// This lists all possible states. This is pregenerated in `new`.
    all_states: Vec<State>,

    ///
    last_states: [State; N],

    /// This is the current state of iteration. It's basically a N digit number
    /// with base `all_states.len()`.
    indices: [usize; N],

    // The depth is also iteration state and basically determines which digits
    // of `indices` is increased next.
    depth: i64,

    /// Just counts down how many elements are still remaining. This is only
    /// for the `ExactSizeIterator` impl.
    remaining: u64,
}

impl<const N: usize> AllTmCombinations<{N}>
where
    [State; N]: LengthAtMost32,
    [usize; N]: LengthAtMost32,
{
    pub fn new() -> Self {
        // We create vectors of all X, starting with movements and state ids.
        let all_movements = [Move::Left, Move::Right];
        let all_writes = [CellValue(false), CellValue(true)];
        let all_state_ids = (0..N).map(|s| s as u8);

        // `all_actions` has the length (N * 2 + 1) * 2.
        let all_actions = all_state_ids.clone()
            .flat_map(|next_state| all_movements.iter().map(move |&m| (next_state, m)))
            .chain(Some((HALT_STATE, Move::Left))) // add the halting state with arbitrary movement
            .flat_map(|(next_state, movement)| {
                all_writes.iter().map(move |&write| Action { next_state, write, movement })
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
            all_states,
            last_states,
            indices,
            depth: 0,
            remaining: total_tms,
        }
    }
}


impl<const N: usize> Iterator for AllTmCombinations<{N}> {
    type Item = Tm<{N}>;
    fn next(&mut self) -> Option<Self::Item> {


        while self.depth >= 0 {
            let state_idx = self.indices[self.depth as usize];
            self.indices[self.depth as usize] += 1;

            match self.all_states.get(state_idx) {
                None => {
                    // We exhausted the current "digit" and have to track back.
                    self.indices[self.depth as usize] = 0;
                    self.depth -= 1;
                }

                Some(state) => {
                    // If not, we update the temporary storage and, if we are
                    // at max depth, return the current state.
                    self.last_states[self.depth as usize] = *state;

                    if self.depth == N as i64 - 1 {
                        self.remaining -= 1;
                        return Some(Tm { states: self.last_states });
                    } else {
                        self.depth += 1;
                    }
                }
            }
        }

        None
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remaining as usize, Some(self.remaining as usize))
    }
}

impl<const N: usize> ExactSizeIterator for AllTmCombinations<{N}> {}
