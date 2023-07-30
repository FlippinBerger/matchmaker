use rdnit::game::Game;
use rdnit::player::Player;

// TODO add some command line parsing here and then put the drivers for each path
// as a separate run somewhere in lib

fn main() {
    let g = Game{players: vec![Player{mmr: 100, name: String::from("chris"), streak: -1}]};

    println!("Length: {}", g.players.len());
}
