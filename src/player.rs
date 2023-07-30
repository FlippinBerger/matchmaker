pub struct Player {
    pub mmr: i32,
    pub name: String,

    // streak negative for losses, positive for wins
    pub streak: i32,
}
