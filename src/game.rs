use crate::player::Player;

pub struct Game {
    pub players: Vec<Player>,
}

impl Game {
    pub fn get_average_mmr(&self) -> f32 {
        let sum: i32 = self.players.iter().map(|player| player.mmr).sum();

        sum as f32 / self.players.len() as f32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn average_mmr() {
        let game = Game {
           players: vec![
               Player {
                   name: String::from("chris"),
                   mmr: 10,
                   streak: 0,
               },
               Player {
                   name: String::from("chris"),
                   mmr: 20,
                   streak: 0
               }
           ] 
        };

        assert_eq!(15., game.get_average_mmr());
    }
}
