use crate::player::Player;
use crate::stats::{Stats, Quality};

#[derive(Debug)]
pub struct Game {
    // pub players: Vec<Player>,
    pub team1: Vec<Player>,
    pub team2: Vec<Player>,
    pub stats: Stats,
}

pub enum Team {
    One,
    Two
}

impl Game {
    pub fn new() -> Self {
        Self {
            team1: vec![],
            team2: vec![],
            stats: Stats::new()
        } 
    }

    pub fn get_average_mmr(&self, team: Team) -> f32 {
        let players = match team {
            Team::One => &self.team1,
            Team::Two => &self.team2,
        };

        let sum: i32 = players.iter().map(|player| player.mmr).sum();

        sum as f32 / players.len() as f32
    }

    pub fn update_mmr_range(&mut self, p: &Player) {
        if let Some(low) = self.stats.mmr_range.low {
            if p.mmr < low {
                self.stats.mmr_range.low = Some(p.mmr);
            }
        }

        if let Some(high) = self.stats.mmr_range.low {
            if p.mmr > high {
                self.stats.mmr_range.high = Some(p.mmr);
            }
        }

        if self.stats.mmr_range.low.is_none() {
            self.stats.mmr_range.low = Some(p.mmr);
        }

        if self.stats.mmr_range.high.is_none() {
            self.stats.mmr_range.high = Some(p.mmr);
        }
    }

    pub fn calculate_stats(&mut self) {
        self.stats.mark_end();

        self.stats.mmr_avg_t1 = self.get_average_mmr(Team::One);
        self.stats.mmr_avg_t2 = self.get_average_mmr(Team::Two);

        let diff = self.stats.mmr_avg_t1 - self.stats.mmr_avg_t2;

        self.stats.quality = match diff.abs() as u32 {
            0..=200 => Quality::Excellent,
            201..=400 => Quality::High,
            401..=600 => Quality::Average,
            601..=800 => Quality::Poor,
            _ => Quality::Terrible,
        };
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    // #[test]
    // fn average_mmr() {
    //     let game = Game {
    //         players: vec![
    //            Player {
    //                name: String::from("chris"),
    //                mmr: 10,
    //                streak: 0,
    //            },
    //            Player {
    //                name: String::from("chris"),
    //                mmr: 20,
    //                streak: 0
    //            }
    //         ],
    //         stats: Stats::new(),
    //     };
    //     assert_eq!(15., game.get_average_mmr());


    //     let game = Game {
    //         players: vec![
    //            Player {
    //                name: String::from("chris"),
    //                mmr: 10,
    //                streak: 0,
    //            }
    //         ],
    //         stats: Stats::new(),
    //     };
    //     assert_eq!(10., game.get_average_mmr());

    //     let game = Game {
    //         players: vec![
    //            Player {
    //                name: String::from("chris"),
    //                mmr: 10,
    //                streak: 0,
    //            },
    //            Player {
    //                name: String::from("chris"),
    //                mmr: 20,
    //                streak: 0,
    //            },
    //            Player {
    //                name: String::from("chris"),
    //                mmr: 35,
    //                streak: 0,
    //            }
    //         ],
    //         stats: Stats::new(),
    //     };
    //     assert_eq!("21.67", format!("{:.2}", game.get_average_mmr()));
    // }
}
