use super::Agent;
use crate::game::{Action, Player, State};

pub struct MinimaxAgent {
    depth: usize,
}

impl MinimaxAgent {
    pub fn new(depth: usize) -> MinimaxAgent {
        MinimaxAgent { depth }
    }

    fn best_action<A: Action, P: Player, S: State<A, P>>(
        state: &S,
        depth: usize,
    ) -> Option<(A, S)> {
        state
            .get_actions_for_current()
            .into_iter()
            .map(|a| {
                let end_state = MinimaxAgent::eval_move_recursive(state, &a, depth);
                (a, end_state)
            })
            .max_by_key(|(_, end_state)| end_state.evaluate(state.get_current_player()))
    }

    fn eval_move_recursive<A: Action, P: Player, S: State<A, P>>(
        state: &S,
        action: &A,
        depth: usize,
    ) -> S {
        let next = state.next_state(action);
        if depth == 0 || next.is_terminal() {
            return next;
        }
        if let Some((_, state)) = MinimaxAgent::best_action(&next, depth - 1) {
            return state;
        } else {
            return next;
        }
    }
}

impl<A: Action, P: Player, S: State<A, P>> Agent<A, P, S> for MinimaxAgent {
    fn get_action(&self, state: &S) -> Option<A> {
        MinimaxAgent::best_action(state, self.depth).map(|(a, s)| {
            println!(
                "MinimaxAgent: expectation: {}",
                s.evaluate(s.get_current_player())
            );
            a
        })
    }
}

/*
best action for the current player (move_taker)
list all possible moves for move_taker
    for each move, evaluate the next state
    if the state is terminal evaluate it from move_taker perspective
    if the state is not terminal.
        we need the next player to take their best action.
        And then we want it to know the result of that action from move_taker perspective
        and we returm that.
choose the best action based off of the score from move_taker perspective


best action for the current player (move_taker)
but we need to also get evaluation for previous player
list all possible moves for move_taker
    for each move, evaluate the next state
    if the state is terminal evaluate it from move_taker perspective AND previous player perspective

    if the state is not terminal.
        we need the next player to take their best action.
        And then we want it to know the result of that action from move_taker perspective
        and we returm that.
*/
