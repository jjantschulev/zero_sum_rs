use rand::seq::SliceRandom;

use super::Agent;
use crate::game::{Action, Player, State};
pub struct RandomAgent {}

impl RandomAgent {
    pub fn new() -> RandomAgent {
        RandomAgent {}
    }
}

impl<A: Action, P: Player, S: State<A, P>> Agent<A, P, S> for RandomAgent {
    fn get_action(&self, state: &S) -> Option<A> {
        let mut rng = rand::thread_rng();
        state.get_actions_for_current().choose(&mut rng).cloned()
    }
}
