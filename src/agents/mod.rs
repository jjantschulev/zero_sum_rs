pub mod human;
pub mod max_n;
pub mod random;

use crate::game::{Action, Player, State};

pub trait Agent<A: Action, P: Player, S: State<A, P>>: Sized {
    fn get_action(&self, state: &S) -> Option<A>;
}
