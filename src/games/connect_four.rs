// #[derive(Copy, Clone)]
// struct World<RedAgent: Agent, YellowAgent: Agent> {
//     board: Board,
//     turn: Player,
//     winner: Option<Player>,
//     red_agent: RedAgent,
//     yellow_agent: YellowAgent,
// }

// impl<RedAgent: Agent, YellowAgent: Agent> World<RedAgent, YellowAgent> {
//     fn new(red_agent: RedAgent, yellow_agent: YellowAgent) -> World<RedAgent, YellowAgent> {
//         World {
//             board: [[None; 7]; 6],
//             turn: Player::Red,
//             winner: None,
//             red_agent: red_agent,
//             yellow_agent: yellow_agent,
//         }
//     }

//     fn play(&mut self, col: usize) {
//         for row in self.board.iter_mut().rev() {
//             if row[col].is_none() {
//                 row[col] = Some(self.turn);
//                 break;
//             }
//         }
//         self.turn = self.turn.other();

//         self.check_winner();
//     }

//     fn check_winner(&mut self) {
//         // Check rows
//         for row in self.board.iter() {
//             let mut streak = 0;
//             let mut last = None;
//             for loc in row.iter() {
//                 if loc == &last {
//                     streak += 1;
//                 } else {
//                     streak = 1;
//                     last = *loc;
//                 }
//                 if streak == 4 && last.is_some() {
//                     self.winner = last;
//                     return;
//                 }
//             }
//         }

//         // Check columns
//         for col in 0..7 {
//             let mut streak = 0;
//             let mut last = None;
//             for row in self.board.iter() {
//                 let loc = row[col];
//                 if loc == last {
//                     streak += 1;
//                 } else {
//                     streak = 1;
//                     last = loc;
//                 }
//                 if streak == 4 && last.is_some() {
//                     self.winner = last;
//                     return;
//                 }
//             }
//         }

//         // Check diagonals
//         for row in 0..3 {
//             for col in 0..4 {
//                 let mut streak = 0;
//                 let mut last = None;
//                 for i in 0..4 {
//                     let loc = self.board[row + i][col + i];
//                     if loc == last {
//                         streak += 1;
//                     } else {
//                         streak = 1;
//                         last = loc;
//                     }
//                     if streak == 4 && last.is_some() {
//                         self.winner = last;
//                         return;
//                     }
//                 }
//             }
//         }

//         for row in 0..3 {
//             for col in 3..7 {
//                 let mut streak = 0;
//                 let mut last = None;
//                 for i in 0..4 {
//                     let loc = self.board[row + i][col - i];
//                     if loc == last {
//                         streak += 1;
//                     } else {
//                         streak = 1;
//                         last = loc;
//                     }
//                     if streak == 4 && last.is_some() {
//                         self.winner = last;
//                         return;
//                     }
//                 }
//             }
//         }

//     }

//     fn play_best_move(&mut self, depth: usize) {
//         if let Some((col, _)) = self.get_best_move(depth) {
//             self.play(col);
//         }
//     }

//     fn get_moves(&self) -> [bool; 7] {
//         let mut moves = [false; 7];
//         for (i, col) in self.board.iter().next().unwrap().iter().enumerate() {
//             if col.is_none() {
//                 moves[i] = true;
//             }
//         }
//         return moves;
//     }

//     fn get_best_move(&self, depth: usize) -> Option<(usize, i32)> {
//         let mut best_move = None;
//         let mut best_score = i32::MIN;
//         for (i, valid) in self.get_moves().iter().enumerate() {
//             if *valid {
//                 let score = self.eval_move_recursive(self.turn, i, depth);
//                 if score > best_score {
//                     best_score = score;
//                     best_move = Some(i);
//                 }
//             }
//         }
//         if let Some(m) = best_move {
//      Some(            (m, best_score))
//         } else {None}
//     }

//     fn eval_move(&self, perspective: Player, col: usize) -> i32 {
//         let mut new_world = self.clone();
//         new_world.play(col);
//         new_world.evaluate(perspective)
//     }

//     fn eval_move_recursive(&self, perspective: Player, col: usize, depth: usize) -> i32 {
//         let mut new_world = self.clone();
//         new_world.play(col);
//         new_world.evaluate_recursive(perspective, depth)
//     }

//     fn evaluate(&self, perspective: Player) -> i32 {
//         match self.winner {
//             Some(player) => if player == perspective { i32::MAX } else { i32::MIN },
//             None => self.heuristic(perspective),
//         }
//     }

//     fn evaluate_recursive(&self, perspective: Player, depth: usize) -> i32 {
//         match self.winner {
//             Some(player) => if player == perspective { i32::MAX } else { i32::MIN },
//             None => {
//                 if depth == 0 {
//                     return self.heuristic(perspective);
//                 } else if let Some((_, score)) = self.get_best_move(depth - 1) {
//                     return score;
//                 } else {
//                     return 0;
//                 }
//             },
//         }
//     }

//     fn heuristic(&self, perspective: Player) -> i32 {
//         let mut score = 0;

//         // Check rows
//         for row in self.board.iter() {
//             let mut streak = 0;
//             let mut last = None;
//             for loc in row.iter() {
//                 if loc == &last {
//                     streak += 1;
//                 } else {
//                     streak = 1;
//                     last = *loc;
//                 }
//                 if streak >= 2 {
//                     if let Some(p) = last {
//                         let modifier = if p == perspective { 1 } else { -1 };
//                         score += streak * streak * modifier;
//                     }
//                 }
//             }
//         }

//         // Check columns
//         for col in 0..7 {
//             let mut streak = 0;
//             let mut last = None;
//             for row in self.board.iter() {
//                 let loc = row[col];
//                 if loc == last {
//                     streak += 1;
//                 } else {
//                     streak = 1;
//                     last = loc;
//                 }
//                 if streak >= 2 {
//                     if let Some(p) = last {
//                         let modifier = if p == perspective { 1 } else { -1 };
//                         score += streak * streak * modifier;
//                     }
//                 }
//             }
//         }

//         // Check diagonals
//         for row in 0..3 {
//             for col in 0..4 {
//                 let mut streak = 0;
//                 let mut last = None;
//                 for i in 0..4 {
//                     let loc = self.board[row + i][col + i];
//                     if loc == last {
//                         streak += 1;
//                     } else {
//                         streak = 1;
//                         last = loc;
//                     }
//                     if streak >= 2 {
//                         if let Some(p) = last {
//                             let modifier = if p == perspective { 1 } else { -1 };
//                             score += streak * streak * modifier;
//                         }
//                     }
//                 }
//             }
//         }

//         for row in 0..3 {
//             for col in 3..7 {
//                 let mut streak = 0;
//                 let mut last = None;
//                 for i in 0..4 {
//                     let loc = self.board[row + i][col - i];
//                     if loc == last {
//                         streak += 1;
//                     } else {
//                         streak = 1;
//                         last = loc;
//                     }
//                     if streak >= 2 {
//                         if let Some(p) = last {
//                             let modifier = if p == perspective { 1 } else { -1 };
//                             score += streak * streak * modifier;
//                         }
//                     }
//                 }
//             }
//         }

//         return score;
//     }

//     fn print(&self) {
//         for row in self.board.iter() {
//             print!("|");
//             for loc in row.iter() {
//                 match loc {
//                     Some(Player::Red) => print!(" R "),
//                     Some(Player::Yellow) => print!(" Y "),
//                     None => print!("   "),
//                 }
//             }
//             println!("|");
//         }
//         println!("|{}|", "___".repeat(7));
//         if let Some(winner) = self.winner {
//             println!("{} wins!", match winner {
//                 Player::Red => "Red",
//                 Player::Yellow => "Yellow",
//             });
//         } else {
//             println!("{}'s turn", match self.turn {
//                 Player::Red => "Red",
//                 Player::Yellow => "Yellow",
//             });
//         }
//         println!("");
//     }
// }

// type Board = [[Option<Player>; 7]; 6];

// #[derive(Copy, Clone, PartialEq)]
// enum Player {
//     Red,
//     Yellow,
// }

// impl Player {
//     fn other(&self) -> Player {
//         match self {
//             Player::Red => Player::Yellow,
//             Player::Yellow => Player::Red,
//         }
//     }
// }
