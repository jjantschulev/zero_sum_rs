use std::fmt::Display;
use std::fmt::Write;

use crate::agents::human::get_int_input;
use crate::{
    agents::human::ActionSelector,
    game::{Action as IAction, Player as IPlayer, State},
};

pub struct TicTacToe {
    board: [[Option<Player>; 3]; 3],
    turn: Player,
}

impl TicTacToe {
    pub fn new() -> TicTacToe {
        TicTacToe {
            board: [[None; 3]; 3],
            turn: Player::X,
        }
    }

    fn is_win(&self, player: Player) -> bool {
        let mut win = false;
        for i in 0..3 {
            win = win
                || (self.board[i][0] == Some(player)
                    && self.board[i][1] == Some(player)
                    && self.board[i][2] == Some(player));
            win = win
                || (self.board[0][i] == Some(player)
                    && self.board[1][i] == Some(player)
                    && self.board[2][i] == Some(player));
        }
        win = win
            || (self.board[0][0] == Some(player)
                && self.board[1][1] == Some(player)
                && self.board[2][2] == Some(player));
        win = win
            || (self.board[0][2] == Some(player)
                && self.board[1][1] == Some(player)
                && self.board[2][0] == Some(player));
        win
    }

    fn to_string(&self, f: &mut String, print_actions_labels: bool) -> std::fmt::Result {
        let mut move_index = 0;
        for i in 0..3 {
            let start = if i == 0 { "┏" } else { "┣" };
            let end = if i == 0 { "┓" } else { "┫" };
            let middle = if i == 0 { "┳" } else { "╋" };
            writeln!(f, "{}━━━{}━━━{}━━━{}", start, middle, middle, end)?;
            // writeln!(f, "┃   ┃   ┃   ┃")?;
            for j in 0..3 {
                write!(f, "┃ ")?;
                match self.board[i][j] {
                    Some(Player::X) => write!(f, "X")?,
                    Some(Player::O) => write!(f, "O")?,
                    None => {
                        if print_actions_labels {
                            write!(f, "{}", move_index)?;
                            move_index += 1;
                        } else {
                            write!(f, " ")?;
                        }
                    }
                }
                write!(f, " ")?;
            }
            writeln!(f, "┃")?;
            // writeln!(f, "┃   ┃   ┃   ┃")?;
            if (i + 1) == 3 {
                writeln!(f, "┗━━━┻━━━┻━━━┛")?;
                writeln!(f, "Turn: {:?}", self.get_current_player())?;
            }
        }
        Ok(())
    }

    fn get_number_of_moves(&self) -> usize {
        let mut count = 0;
        for i in 0..3 {
            for j in 0..3 {
                if self.board[i][j].is_none() {
                    count += 1;
                }
            }
        }
        count
    }
}

impl Display for TicTacToe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        self.to_string(&mut s, false)?;
        write!(f, "{}", s)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Action(usize, usize);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Player {
    X,
    O,
}

impl Player {
    pub fn other(&self) -> Player {
        match self {
            Player::X => Player::O,
            Player::O => Player::X,
        }
    }
}

impl IAction for Action {}
impl IPlayer for Player {}

impl Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

impl State<Action, Player> for TicTacToe {
    fn evaluate(&self, player: &Player) -> i64 {
        if self.is_win(*player) {
            return 1;
        } else if self.is_win(player.other()) {
            return -1;
        } else {
            return 0;
        }
    }

    fn get_actions(&self, _player: &Player) -> Vec<Action> {
        let mut actions = Vec::new();
        for i in 0..3 {
            for j in 0..3 {
                if self.board[i][j].is_none() {
                    actions.push(Action(i, j));
                }
            }
        }
        actions
    }

    fn next_state(&self, action: &Action) -> Self {
        let mut new_board = self.board;
        new_board[action.0][action.1] = Some(self.turn);
        TicTacToe {
            board: new_board,
            turn: self.turn.other(),
        }
    }

    fn is_terminal(&self) -> bool {
        self.is_win(self.turn) || self.is_win(self.turn.other()) || self.get_number_of_moves() == 0
    }

    fn get_current_player(&self) -> &Player {
        &self.turn
    }
}

pub struct TicTacToeActionSelector;

impl ActionSelector for TicTacToeActionSelector {
    type A = Action;
    type P = Player;
    type S = TicTacToe;

    fn get_action(state: &Self::S) -> Option<Self::A> {
        let mut f = String::new();
        state.to_string(&mut f, true).unwrap();
        println!("{}", f);
        let index = get_int_input(0..state.get_number_of_moves());
        return state.get_actions_for_current().get(index).cloned();
    }
}
