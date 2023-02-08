use tokio::sync::mpsc::Receiver;

use super::LogEntry;
pub async fn insert(
    mut logentryrecv: Receiver<LogEntry>,
    masterlog: &mut Vec<LogEntry>,
    mut termstate: i64,
    commitindex: i64,
) {
    loop {
        match logentryrecv.recv().await {
            None => {}
            Some(log_entry) => {
                if termstate < log_entry.term {
                    //Instantly get the new term
                    termstate = log_entry.term;
                } else if termstate > log_entry.term {
                    //Ignore requests from old leaders
                    println!("{:#?}", masterlog);
                    continue;
                }
                if log_entry.prevlogindex == -1 {
                    masterlog.push(log_entry);
                    println!("{:#?}", masterlog);
                    continue;
                }
                match masterlog.iter().nth(log_entry.prevlogindex as usize) {
                    None => {
                        //Meaning the previous index doesnt exists, simply println!("{:#?}",masterlog);continue
                        println!("{:#?}", masterlog);
                        continue;
                    }
                    _ => {}
                }
                //Len is fine, need to check term of previndex
                if masterlog
                    .iter()
                    .nth(log_entry.prevlogindex as usize)
                    .unwrap()
                    .term
                    != log_entry.prevlogterm
                {
                    //If not right, pop this entry and everything after
                    //The leader nextIndex for this follower must decrement
                    loop {
                        if masterlog.len() as i64 == log_entry.prevlogindex {
                            break;
                        }
                        masterlog.pop().unwrap();
                    }
                    //To self : Only during first ping it is possible that a leader might send a very old
                    //index, after that it only decrements by one!
                    println!("{:#?}", masterlog);
                    continue;
                }
                masterlog.push(log_entry);
                // state::newcommit(log_entry.leadercommit,commitindex);
                println!("{:#?}", masterlog);
                continue;
            }
        }
    }
}
