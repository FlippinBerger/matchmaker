use std::sync::mpsc::{self, Sender, Receiver};
use std::thread;

use crate::service::{FastestMatchmaker, Matchmaker};
use player::{Player, spawn_players};

pub mod game;
pub mod player;
pub mod service;
pub mod stats;

pub fn run() {
    let (sender, receiver): (Sender<Player>, Receiver<Player>) = mpsc::channel();

    // TODO break this out into smarter player creation
    for _ in 0..10 {
        let sender = sender.clone();
        thread::spawn(move || spawn_players(sender));
    }

    // drop the original sender that didn't do any work to close the channel
    drop(sender);

    // create the chosen matchmaker and matchmake
    let mut fast_mm = FastestMatchmaker::new(receiver);
    fast_mm.matchmake();

    // make sure everything looks right for quick testing right now
    println!("{:?}", fast_mm);
    println!("Num games: {}", fast_mm.games.len());
    println!("Num players: {}", fast_mm.games[0].players.len());
}

