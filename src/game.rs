use std::{
    fmt::{Debug, Display},
    marker::PhantomData,
};

use crate::agents::Agent;

pub struct Game<A: Action, P: Player, S: State<A, P>> {
    _action: PhantomData<A>,
    _player: PhantomData<P>,
    state: S,
    players: Vec<Box<dyn Fn(&S) -> Option<A>>>,
}

impl<A: Action, P: Player, S: State<A, P>> Game<A, P, S> {
    pub fn new(state: S) -> Game<A, P, S> {
        Game {
            _action: PhantomData,
            _player: PhantomData,
            state,
            players: Vec::new(),
        }
    }

    pub fn add_player(&mut self, player: impl Agent<A, P, S> + 'static) {
        self.players
            .push(Box::new(move |state| player.get_action(state)));
    }

    pub fn get_state(&self) -> &S {
        &self.state
    }

    pub fn play(&mut self) {
        while !self.state.is_terminal() {
            let player = self.players.remove(0);
            if let Some(action) = player(&self.state) {
                let next_state = self.state.next_state(&action);
                self.state = next_state;
                self.players.push(player);
            }
        }
        println!("Game over!");
        if let Some(winner) = self.state.get_winner() {
            println!("Winner: {:?}", winner);
        } else {
            println!("Draw! (there are no actions left for the current player)");
        }
    }
}

impl<A: Action, P: Player, S: State<A, P> + Display> Game<A, P, S> {
    pub fn print(&self) {
        println!("\nState: \n{}\n", self.state,);
    }
}

pub trait State<A: Action, P: Player>: Sized {
    fn evaluate(&self, player: &P) -> i64;
    fn get_actions(&self, player: &P) -> Vec<A>;
    fn get_current_player(&self) -> &P;
    fn next_state(&self, action: &A) -> Self;
    fn get_winner(&self) -> Option<P>;

    fn is_terminal(&self) -> bool {
        self.get_winner().is_some() || self.get_actions_for_current().len() == 0
    }

    fn get_actions_for_current(&self) -> Vec<A> {
        self.get_actions(&self.get_current_player())
    }
}

pub trait Player: Sized + Clone + Debug {}

pub trait Action: Sized + Clone {}
