use std::sync::mpsc::Receiver;

use crate::game::Game;
use crate::player::Player;
use crate::stats::Stats;

pub trait Matchmaker {
    /// matchmake runs the matchmaking algorithm for the
    /// given matchmaker by reading the players out of 
    /// the Receiver and placing them into games
    fn matchmake(&mut self);
}

#[derive(Debug)]
pub struct FastestMatchmaker {
    pub games: Vec<Game>,
    rx: Receiver<Player>,
    pub stats: Stats,
}

impl FastestMatchmaker {
    pub fn new(rx: Receiver<Player>, stats: Stats) -> Self {
        Self {
            games: vec![],
            rx,
            stats,
        }
    }
}

impl Matchmaker for FastestMatchmaker {
    fn matchmake(&mut self) {
        let mut player_num = 0;

        let mut game = Game::new();

        loop {
            if let Ok(player) = self.rx.recv() {
                game.update_mmr_range(&player);

                if game.team1.len() < 5 {
                    game.team1.push(player);
                } else {
                    game.team2.push(player);
                }

                if player_num == 9 {
                    game.calculate_stats();
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
