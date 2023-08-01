use std::time::{self, Instant};

#[derive(Debug)]
pub enum Quality {
    Terrible,
    Poor,
    Average,
    High,
    Excellent,
}

/// Stats attempt to guage how good matchmaking was for a given game
#[derive(Debug)]
pub struct Stats {
    /// how good was the game
    pub quality: Quality,

    // the lowest and highest rating of the players in the game
    pub mmr_range: Range<i32>,
    pub mmr_avg_t1: f32,
    pub mmr_avg_t2: f32,

    // the shortest and longest a player had to wait to get matched
    // pub player_wait_time_range: Range<time::Duration>,

    created_time: time::Instant,
    pub time_to_match: time::Duration
}


// make this generic because why not
#[derive(Debug)]
pub struct Range<T> {
    pub low: Option<T>,
    pub high: Option<T>
}

impl Stats {
    pub fn new() -> Self {
        Stats {
            quality: Quality::Average,
            mmr_range: Range { low: None, high: None},
            mmr_avg_t1: 0.,
            mmr_avg_t2: 0.,
            created_time: Instant::now(),
            time_to_match: time::Duration::new(0, 0),
        }
    }

    /// mark_end is used to snag the duration of how long it took to fill the game
    /// without exposing the created_time
    pub fn mark_end(&mut self) {
        self.time_to_match = self.created_time.elapsed();
    }
}
