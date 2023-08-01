use std::sync::mpsc::{self, Sender, Receiver};
use std::thread;

use player::{Player, spawn_players};
use crate::matchmaker::Matchmaker;
use crate::matchmaker::{
    fastmatchmaker::FastestMatchmaker,
    mmrmatchmaker::MMRMatchmaker,
};
use crate::stats::Stats;

pub mod game;
pub mod player;
pub mod matchmaker;
pub mod stats;

pub fn run() {
    let (tx, rx): (Sender<Player>, Receiver<Player>) = mpsc::channel();

    let statistics = Stats::new();

    // TODO break this out into smarter player creation
    for _ in 0..10 {
        let tx = tx.clone();
        thread::spawn(move || spawn_players(tx));
    }

    // drop the original sender that didn't do any work to close the channel
    drop(tx);

    // create the chosen matchmaker and matchmake
    let mut fast_mm = FastestMatchmaker::new(rx, statistics);
    fast_mm.matchmake();

    // make sure everything looks right for quick testing right now
    println!("{:?}", fast_mm);
    println!("Num games: {}", fast_mm.games.len());
    println!("Num players: {}", fast_mm.games[0].team1.len() + fast_mm.games[0].team2.len());

    println!();
    println!("{:?}", fast_mm.games[0].stats);
}

