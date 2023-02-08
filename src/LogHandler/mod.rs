#![allow(non_snake_case)]

#[derive(Debug)]
pub struct LogEntry{
    pub leaderip : String, 
    pub prevlogindex : i64,
    pub prevlogterm : i64,
    pub leadercommit : i64,
    pub term : i64,
    pub entry : String
}
pub mod Log;
