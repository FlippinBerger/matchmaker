use std::collections::HashMap;
use std::sync::mpsc::Receiver;

use crate::game::Game;
use crate::player::Player;
use crate::matchmaker::Matchmaker;
use crate::stats::Stats;

#[derive(Debug)]
pub struct MMRMatchmaker {
    pub games: Vec<Game>,
    rx: Receiver<Player>,
    pub stats: Stats,
    game_map: HashMap<i32, Game>,
}

impl MMRMatchmaker {
    pub fn new(rx: Receiver<Player>, stats: Stats) -> Self {
        Self {
            games: vec![],
            rx,
            stats,
            game_map: HashMap::new(),
        }
    }
}

impl Matchmaker for MMRMatchmaker {
    fn matchmake(&mut self) {
        while let Ok(player) = self.rx.recv() {
            let mmr_bracket = player.mmr / 1000;
            let game_res = self.game_map.get_mut(&mmr_bracket);

            match game_res {
                Some(game) => {
                    if game.team1.len() < 5 {
                        game.team1.push(player);
                    } else if game.team2.len() < 5 {
                        game.team2.push(player);
                    }

                    if game.team2.len() == 5 {
                        let game = self.game_map.remove(&mmr_bracket).unwrap();
                        self.games.push(game);
                    }
                },
                None => {
                    let mut game = Game::new();
                    game.team1.push(player);
                    self.game_map.insert(mmr_bracket, game);
                }
            };
        } 
    }
}
