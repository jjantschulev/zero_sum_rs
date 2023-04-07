use std::{fmt::Display, marker::PhantomData, ops::Range};

use super::Agent;
use crate::game::{Action, Player, State};

pub struct HumanAgent<T: ActionSelector> {
    _selector: PhantomData<T>,
}

impl<T: ActionSelector> HumanAgent<T> {
    pub fn new() -> HumanAgent<T> {
        HumanAgent {
            _selector: PhantomData,
        }
    }
}

impl<T: ActionSelector> Agent<T::A, T::P, T::S> for HumanAgent<T> {
    fn get_action(&self, state: &T::S) -> Option<T::A> {
        T::get_action(state)
    }
}

pub trait ActionSelector {
    type A: Action + Display;
    type P: Player;
    type S: State<Self::A, Self::P>;

    fn get_action(state: &Self::S) -> Option<Self::A>;
}

pub struct DefaultActionSelector<A, P, S> {
    _action: PhantomData<A>,
    _player: PhantomData<P>,
    _state: PhantomData<S>,
}

impl<A: Action + Display, P: Player, S: State<A, P> + Display> ActionSelector
    for DefaultActionSelector<A, P, S>
{
    type A = A;
    type P = P;
    type S = S;

    fn get_action(state: &S) -> Option<A> {
        println!("Current state:");
        println!("{}", state);
        let avaliable = state.get_actions_for_current();
        println!("Available actions:");
        for (i, a) in avaliable.iter().enumerate() {
            println!("{}: {}", i, a);
        }
        return avaliable.get(get_int_input(0..avaliable.len())).cloned();
    }
}

pub fn get_int_input(range: Range<usize>) -> usize {
    let mut input = String::new();
    loop {
        println!(
            "Enter a number between {} and {}: ",
            range.start,
            range.end - 1
        );

        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        if let Ok(i) = input.parse::<usize>() {
            if range.contains(&i) {
                return i;
            }
        }
        println!("Invalid input: {}", input);
    }
}
