use std::time::SystemTime;
use async_channel::{Receiver, Sender};
use crate::services::format_time;
use crate::services::nanowave_player_command::NanowavePlayerCommand;
use crate::services::nanowave_player_event::NanowavePlayerEvent;

pub struct NanowavePlayerService {
    
}

impl NanowavePlayerService {
    pub fn new() -> NanowavePlayerService {
        Self {}
    }

    pub async fn run(&self, rx: Receiver<NanowavePlayerCommand>, tx: Sender<NanowavePlayerEvent>) {
        loop {
            while let Ok(cmd) = rx.recv().await {
                println!("Command received...");
                match cmd {
                    NanowavePlayerCommand::PlayTest(msg) => {
                        println!("PlayTest received: {}", msg);
                        let response = NanowavePlayerEvent::OutputText(format!("{}: {}", format_time(SystemTime::now()), msg).into());
                        tx.send(response).await.unwrap();
                    }
                }
            }
        }
    }
    
}