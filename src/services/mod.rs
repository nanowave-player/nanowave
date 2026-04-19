pub mod nanowave_player_command;
pub mod nanowave_player_event;
pub mod nanowave_player_service;

use crate::services::nanowave_player_service::NanowavePlayerService;
use crate::{NanowavePlayerCommand, NanowavePlayerEvent, ServiceConfig};
use async_channel::{Receiver, Sender};
use chrono::{DateTime, Local};
use smol::Timer;
use std::future::pending;
use std::time::{Duration, SystemTime};

fn empty_string_fallback(value: String, fallback_value: &str) -> String {
    if value.is_empty() {
        fallback_value.to_string()
    } else {
        value
    }
}

pub fn start_services(
    config: ServiceConfig,
    rx: Receiver<NanowavePlayerCommand>,
    tx: Sender<NanowavePlayerEvent>,
) {
    let audio_device = empty_string_fallback(config.audio_device, "").clone();
    let sample_file = empty_string_fallback(config.sample_file, "").clone();

    std::thread::spawn(move || {
        smol::block_on(async move {
            // Service 1: Echo service
            smol::spawn({
                let tx = tx.clone();
                async move {
                    NanowavePlayerService::new(audio_device, sample_file)
                        .run(rx, tx)
                        .await;
                }
            })
            .detach();

            // Service 2: Logger service
            smol::spawn({
                async move {
                    loop {
                        let now = SystemTime::now();
                        let _r = tx
                            .send(NanowavePlayerEvent::Position(format_time(now)))
                            .await;
                        Timer::after(Duration::from_secs(1)).await;
                    }
                }
            })
            .detach();

            // Keep executor alive forever
            pending::<()>().await;
        });
    });
}

fn format_time(t: SystemTime) -> String {
    let datetime: DateTime<Local> = t.into();
    datetime.format("%H:%M:%S").to_string()
}
