use std::sync::mpsc::Receiver;

use crate::game::Game;
use crate::player::Player;

pub trait Matchmaker {
    /// matchmake runs the matchmaking algorithm for the
    /// given matchmaker by reading the players out of 
    /// the Receiver and placing them into games
    fn matchmake(&mut self);
}

#[derive(Debug)]
pub struct FastestMatchmaker {
    receiver: Receiver<Player>,
    pub games: Vec<Game>,
}

impl FastestMatchmaker {
    pub fn new(r: Receiver<Player>) -> Self {
        Self {
            receiver: r,
            games: vec![],
        }
    }
}

impl Matchmaker for FastestMatchmaker {
    fn matchmake(&mut self) {
        let mut player_num = 0;

        let mut game = Game::new();

        loop {
            if let Ok(player) = self.receiver.recv() {
                game.players.push(player);

                if player_num == 9 {
                    self.games.push(game);
                    game = Game::new();
                }

                player_num = (player_num + 1) % 10;
            } else {
                break;
            }
        }
    }
}
