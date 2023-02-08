mod LogHandler; 
use LogHandler::*;
use tokio::sync::{mpsc,mpsc::Receiver, mpsc::Sender};

#[tokio::main]
async fn main() {
    let mut masterlog : Vec<LogHandler::LogEntry> = Vec::new();
    let mut termstate : i64 = -1;
    let mut commitindex : i64 = -1;
    let (logentrysend, logentryrecv) : (Sender<LogEntry>,Receiver<LogEntry>) = mpsc::channel(1000);
    tokio::spawn(async move{
        Log::insert(logentryrecv,&mut masterlog,termstate,commitindex).await;
    });
    tokio::spawn(async move{
       runtime(logentrysend).await; 
    }).await.unwrap();
}

async fn runtime(logentrysend : Sender<LogEntry>){
    //-----Such log entries must come from leader----
    //[TODO] move these to testing
    let logentry1 = LogHandler::LogEntry{
        leaderip : String::from("0.0.0.0:8080"),
        prevlogindex : -1,
        prevlogterm : -1,
        leadercommit : -1,
        term : 1,
        entry : String::from("SET name hellworld")
    };
    let logentry2 = LogHandler::LogEntry{
        leaderip : String::from("0.0.0.0:8080"),
        prevlogindex : 0,
        prevlogterm : 1,
        leadercommit : 0,
        term : 1,
        entry : String::from("SET name2 hellworld")
    };
    let logentry3 = LogHandler::LogEntry{
        leaderip : String::from("0.0.0.0:8080"),
        prevlogindex : 1,
        prevlogterm : 1,
        leadercommit : 0, //Uncommited case
        term : 1,
        entry : String::from("SET name3 hellworld")
    };
    let logentry4 = LogHandler::LogEntry{
        leaderip : String::from("0.0.0.0:8080"),
        prevlogindex : 2,
        prevlogterm : 1,
        leadercommit : 1, //Late commit, should not commit in index2
        term : 1,
        entry : String::from("SET name3 hellworld")
    };
    let logentry5 = LogHandler::LogEntry{
        leaderip : String::from("0.0.0.0:8080"),
        prevlogindex : 4, //Missed log entry, local log does not have this log
        prevlogterm : 1,
        leadercommit : 3, //Should not execute this commit cus fault detected
        term : 1,
        entry : String::from("SET name3 hellworld") //Should not be in log
    };
    let logentry6 = LogHandler::LogEntry{
        leaderip : String::from("0.0.0.0:8080"),
        prevlogindex : 2, 
        prevlogterm : 2, //Wrong term should return false and delete every log entry after
        leadercommit : 3, //Should not execute this commit cus fault detected
        term : 3, //Should update term
        entry : String::from("SET name3 hellworld")
    };
    logentrysend.send(logentry1).await.unwrap();
    logentrysend.send(logentry2).await.unwrap();
    logentrysend.send(logentry3).await.unwrap();
    logentrysend.send(logentry4).await.unwrap();
    logentrysend.send(logentry5).await.unwrap();
    logentrysend.send(logentry6).await.unwrap();
}
