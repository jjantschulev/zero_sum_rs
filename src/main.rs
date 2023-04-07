use zero_sum::{
    agents::{human::HumanAgent, minimax::MinimaxAgent, random::RandomAgent},
    game::Game,
    games::tictactoe::{TicTacToe, TicTacToeActionSelector},
};

fn main() {
    let mut game = Game::new(TicTacToe::new());

    game.add_player(HumanAgent::<TicTacToeActionSelector>::new());
    game.add_player(MinimaxAgent::new(9));
    game.add_player(RandomAgent::new());

    game.play();

    game.print();
}
