use std::{
    array::LengthAtMost32,
    convert::TryInto,
    fmt,
};

use crate::{CellId, CellValue};


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

#[derive(Clone, Copy)]
pub struct State {
    pub on_0: Action,
    pub on_1: Action,
}

impl State {
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

#[derive(Clone, Copy)]
pub struct Action {
    // As the busy beaver problem is almost basically impossible for N=6, `u8`
    // able to refer to 256 states is more than enough.
    pub next_state: u8,
    pub movement: Move,
}

impl fmt::Debug for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let letter = if self.movement == Move::Left { "l" } else { "r" };
        write!(f, "{: >2}{}", self.next_state, letter)
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Move {
    Left,
    Right,
}


/// Generates all combinations of TMs with `N` states.
///
/// The resulting vector has the length `((2 * N)^2)^N`. For small N:
/// - 1: 4
/// - 2: 256
/// - 3: 46_656
/// - 4: 16_777_216
pub fn gen_all_tms<const N: usize>() -> Vec<Tm<{N}>>
where
    [State; N]: LengthAtMost32,
 {
    // We create vectors of all X, starting with movements and state ids.
    let all_movements = [Move::Left, Move::Right];
    let all_state_ids = (0..N).map(|s| s as u8);

    // `all_actions` has the length 2 * N.
    let all_actions = all_state_ids.clone().flat_map(|next_state| {
        all_movements.iter().map(move |&movement| Action { next_state, movement })
    }).collect::<Vec<_>>();

    // `all_states` has the length (2 * N)^2.
    let all_states = all_actions.iter().flat_map(|&on_0| {
        all_actions.iter().map(move |&on_1| State { on_0, on_1 })
    }).collect::<Vec<_>>();

    // The next step would ideally be done recursively with a helper function
    // that starts at `N=1` and every other invocation is solved recursively.
    // However, expressions in const generics are not yet implemented, so we
    // can't do that.

    // This is temporary storage for the states of a TM. Sadly, we can't create
    // this array immediately (limitation in const generics). So we have to
    // create a vector first and then convert it into an array.
    let dummy_state = all_states[0];
    let mut tmp_states: [_; N] = vec![dummy_state; N][..].try_into().unwrap();

    // We iterate over all possible states N times, creating the cross product
    // of all states. This array holds all iterators.
    let mut iters = vec![all_states.iter(); N];

    // The depth says at which iterator we are currently pulling.
    let mut depth: i64 = 0;
    let mut out = Vec::new();
    while depth >= 0 {
        match iters[depth as usize].next() {
            // If the iterator is exhausted, we go a step back and reset the
            // iterator at this depth.
            None => {
                iters[depth as usize] = all_states.iter();
                depth -= 1;
            }

            // If not, we update the temporary storage and, if we are at max
            // depth, push the current state.
            Some(state) => {
                tmp_states[depth as usize] = *state;

                if depth == N as i64 - 1 {
                    out.push(Tm { states: tmp_states });
                } else {
                    depth += 1;
                }
            }
        }
    }

    out
}
