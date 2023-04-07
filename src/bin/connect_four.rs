use zero_sum::{
    agents::{human::HumanAgent, minimax::MinimaxAgent},
    game::Game,
    games::connect_four::{ConnectFour, ConnectFourActionSelector},
};

fn main() {
    let mut game = Game::new(ConnectFour::new());
    game.add_player(HumanAgent::<ConnectFourActionSelector>::new());
    // game.add_player(MinimaxAgent::new(8));
    game.add_player(MinimaxAgent::new(7));

    game.play();
}
