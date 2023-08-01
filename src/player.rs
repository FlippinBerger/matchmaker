use std::sync::mpsc::Sender;

#[derive(Debug)]
pub struct Player {
    pub mmr: i32,
    pub name: String,

    // streak negative for losses, positive for wins
    pub streak: i32,
}

pub fn spawn_players(sender: Sender<Player>) {
   let p = Player {
        mmr: 1000,
        name: String::from("flippin"),
        streak: 6,
    };

    if let Err(e) = sender.send(p) {
        println!("Fucked up the send: {e}");
    }
}
