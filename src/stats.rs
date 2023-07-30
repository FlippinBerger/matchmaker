use std::time;

pub struct Stats {
    /// how good was the game
    pub quality: f32,

    // the lowest and highest rating of the players in the game
    pub mmr_range: Range<i32>,

    // the shortest and longest a player had to wait to get matched
    pub player_wait_time_range: Range<time::Duration>,

    pub time_to_match: time::Duration
}


// make this generic because why not
pub struct Range<T> {
    low: T,
    high: T
}

impl Stats {
    pub fn get_mmr_diff(self) -> i32 {
        self.mmr_range.high - self.mmr_range.low
    }
}
