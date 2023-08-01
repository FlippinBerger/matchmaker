pub mod fastmatchmaker;
pub mod mmrmatchmaker;

pub trait Matchmaker {
    /// matchmake runs the matchmaking algorithm for the
    /// given matchmaker by reading the players out of 
    /// the Receiver and placing them into games
    fn matchmake(&mut self);
}
