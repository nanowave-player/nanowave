pub mod nanowave_player;
pub mod nanowave_player_command;
pub mod nanowave_player_event;

use crate::{NanowavePlayerCommand, NanowavePlayerEvent};
use async_channel::{Receiver, Sender};
use chrono::{DateTime, Local};
use smol::Timer;
use std::future::pending;
use std::time::{Duration, SystemTime};

pub fn start_services(
    rx: Receiver<NanowavePlayerCommand>,
    tx: Sender<NanowavePlayerEvent>,
) {
    std::thread::spawn(move || {
        smol::block_on(async move {

            // Service 1: Echo service
            smol::spawn({
                let tx = tx.clone();
                async move {
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
            }).detach();

            // Service 2: Logger service
            smol::spawn({
                async move {
                    loop {
                        let now = SystemTime::now();
                        let _r = tx.send(NanowavePlayerEvent::Position(format_time(now))).await;
                        Timer::after(Duration::from_secs(1)).await;
                    }
                }
            }).detach();

            // Keep executor alive forever
            pending::<()>().await;
        });
    });
}

fn format_time(t: SystemTime) -> String {
    let datetime: DateTime<Local> = t.into();
    datetime.format("%H:%M:%S").to_string()
}
