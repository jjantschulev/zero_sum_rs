use crate::agents::human::{get_int_input, ActionSelector};
use crate::game::{Action as IAction, Player as IPlayer, State};
use std::fmt::Display;
use std::fmt::Write;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Player {
    X,
    O,
}

impl Player {
    fn other(&self) -> Player {
        match self {
            Player::X => Player::O,
            Player::O => Player::X,
        }
    }
}

#[derive(Clone, Debug)]
struct Board([[Option<Player>; 7]; 6]);

#[derive(Copy, Clone, Debug)]
pub struct Action(usize);

impl IAction for Action {}
impl IPlayer for Player {}

#[derive(Clone, Debug)]
pub struct ConnectFour {
    board: Board,
    turn: Player,
    winner: Option<Player>,
}

impl ConnectFour {
    pub fn new() -> ConnectFour {
        ConnectFour {
            board: Board([[None; 7]; 6]),
            turn: Player::X,
            winner: None,
        }
    }
}

impl State<Action, Player> for ConnectFour {
    fn evaluate(&self, player: &Player) -> i64 {
        if self.winner == Some(*player) {
            i64::MAX
        } else if self.winner == Some(player.other()) {
            i64::MIN
        } else {
            self.board.get_score(player)
        }
    }

    fn next_state(&self, action: &Action) -> Self {
        let mut new_board = self.board.clone();
        for row in new_board.0.iter_mut().rev() {
            if row[action.0].is_none() {
                row[action.0] = Some(self.turn);
                break;
            }
        }
        ConnectFour {
            winner: new_board.check_winner(),
            board: new_board,
            turn: self.turn.other(),
        }
    }

    fn get_actions(&self, player: &Player) -> Vec<Action> {
        let mut actions = Vec::new();
        if self.turn == *player {
            for i in 0..7 {
                if self.board.0[0][i].is_none() {
                    actions.push(Action(i));
                }
            }
        }
        actions
    }

    fn get_winner(&self) -> Option<Player> {
        self.winner
    }

    fn get_current_player(&self) -> &Player {
        &self.turn
    }
}

impl Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Display for ConnectFour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let board = self.board.to_string();
        write!(f, "{}", board)?;

        if self.winner.is_some() {
            write!(f, "Winner: {:?}", self.winner.unwrap())?;
        } else {
            write!(f, "Turn: {:?}", self.turn)?;
        }

        Ok(())
    }
}

impl Board {
    fn check_winner(&self) -> Option<Player> {
        // Check rows
        for row in self.0.iter() {
            let mut streak = 0;
            let mut last = None;
            for loc in row.iter() {
                if loc == &last {
                    streak += 1;
                } else {
                    streak = 1;
                    last = *loc;
                }
                if streak == 4 && last.is_some() {
                    return last;
                }
            }
        }

        // Check columns
        for col in 0..7 {
            let mut streak = 0;
            let mut last = None;
            for row in self.0.iter() {
                let loc = row[col];
                if loc == last {
                    streak += 1;
                } else {
                    streak = 1;
                    last = loc;
                }
                if streak == 4 && last.is_some() {
                    return last;
                }
            }
        }

        // Check diagonals
        for row in 0..3 {
            for col in 0..4 {
                let mut streak = 0;
                let mut last = None;
                for i in 0..4 {
                    let loc = self.0[row + i][col + i];
                    if loc == last {
                        streak += 1;
                    } else {
                        streak = 1;
                        last = loc;
                    }
                    if streak == 4 && last.is_some() {
                        return last;
                    }
                }
            }
        }

        for row in 0..3 {
            for col in 3..7 {
                let mut streak = 0;
                let mut last = None;
                for i in 0..4 {
                    let loc = self.0[row + i][col - i];
                    if loc == last {
                        streak += 1;
                    } else {
                        streak = 1;
                        last = loc;
                    }
                    if streak == 4 && last.is_some() {
                        return last;
                    }
                }
            }
        }
        None
    }

    fn get_score(&self, perspective: &Player) -> i64 {
        let mut score = 0;

        // Check rows
        for row in self.0.iter() {
            let mut streak = 0;
            let mut last = None;
            for loc in row.iter() {
                if loc == &last {
                    streak += 1;
                } else {
                    streak = 1;
                    last = *loc;
                }
                if streak >= 2 {
                    if let Some(p) = last {
                        let modifier = if &p == perspective { 1 } else { -1 };
                        score += streak * streak * modifier;
                    }
                }
            }
        }

        // Check columns
        for col in 0..7 {
            let mut streak = 0;
            let mut last = None;
            for row in self.0.iter() {
                let loc = row[col];
                if loc == last {
                    streak += 1;
                } else {
                    streak = 1;
                    last = loc;
                }
                if streak >= 2 {
                    if let Some(p) = last {
                        let modifier = if &p == perspective { 1 } else { -1 };
                        score += streak * streak * modifier;
                    }
                }
            }
        }

        // Check diagonals
        for row in 0..3 {
            for col in 0..4 {
                let mut streak = 0;
                let mut last = None;
                for i in 0..4 {
                    let loc = self.0[row + i][col + i];
                    if loc == last {
                        streak += 1;
                    } else {
                        streak = 1;
                        last = loc;
                    }
                    if streak >= 2 {
                        if let Some(p) = last {
                            let modifier = if &p == perspective { 1 } else { -1 };
                            score += streak * streak * modifier;
                        }
                    }
                }
            }
        }

        for row in 0..3 {
            for col in 3..7 {
                let mut streak = 0;
                let mut last = None;
                for i in 0..4 {
                    let loc = self.0[row + i][col - i];
                    if loc == last {
                        streak += 1;
                    } else {
                        streak = 1;
                        last = loc;
                    }
                    if streak >= 2 {
                        if let Some(p) = last {
                            let modifier = if &p == perspective { 1 } else { -1 };
                            score += streak * streak * modifier;
                        }
                    }
                }
            }
        }

        return score;
    }

    fn to_string(&self) -> String {
        let mut s = String::new();
        for row in self.0.iter() {
            write!(s, "|").unwrap();
            for loc in row.iter() {
                match loc {
                    Some(Player::X) => write!(s, " X ").unwrap(),
                    Some(Player::O) => write!(s, " O ").unwrap(),
                    None => write!(s, "   ").unwrap(),
                }
            }
            writeln!(s, "|").unwrap();
        }
        writeln!(s, "|{}|", "___".repeat(7)).unwrap();
        s
    }
}

pub struct ConnectFourActionSelector;

impl ActionSelector for ConnectFourActionSelector {
    type A = Action;
    type P = Player;
    type S = ConnectFour;

    fn get_action(state: &Self::S) -> Option<Self::A> {
        print!("{}", state.board.to_string());
        println!("| 1  2  3  4  5  6  7 |");
        let index = get_int_input(1..8);
        return Some(Action(index - 1));
    }
}
