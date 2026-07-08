pub mod nanowave_player_command;
pub mod nanowave_player_event;
pub mod nanowave_player_service;

use crate::{NanowavePlayerCommand, NanowavePlayerEvent, ServiceConfig};
use chrono::{DateTime, Local};
use std::thread;
use std::time::{Duration, SystemTime};
use tokio::runtime::{Handle};
use tokio::sync::broadcast::{Receiver, Sender};



pub fn start_services(
    config: ServiceConfig,
    tx: Sender<NanowavePlayerEvent>,
    rx: Receiver<NanowavePlayerCommand>,
    runtime_handle: Handle,
) {
    thread::spawn(move || {

        println!("start_services");
        runtime_handle.block_on(background_services(config, tx, rx));
    });
/*
    thread::spawn(move || {
        println!("spawn");
        let config = config.clone();
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();

        runtime.block_on(background_services(config, tx, rx));
    });

 */
}

pub async fn background_services(
    config: ServiceConfig,
    tx: Sender<NanowavePlayerEvent>,
    _rx: Receiver<NanowavePlayerCommand>,
) {
    println!("background_services");
    let timer_task = tokio::spawn(async move {
        println!("timer_task");

        loop {
            println!("timer update");

            let now = SystemTime::now();
            let _r = tx
                .send(NanowavePlayerEvent::Position(format_time(now)));
            // .await;
            // Timer::after(Duration::from_secs(1)).await;
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    });




    let _ = tokio::join!(
        timer_task
    );



    let _audio_device = empty_string_fallback(config.audio_device, "").clone();
    let _sample_file = empty_string_fallback(config.sample_file, "").clone();


    /*
        let x = handle.spawn(async move {

            NanowavePlayerService::new(audio_device, sample_file)
                .run(rx, tx)
                .await;


    });
*/


    /*
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
                            .send(NanowavePlayerEvent::Position(format_time(now)));
                            // .await;
                        Timer::after(Duration::from_secs(1)).await;
                    }
                }
            })
            .detach();

            // Keep executor alive forever
            pending::<()>().await;
        });
    });

     */
}

fn empty_string_fallback(value: String, fallback_value: &str) -> String {
    if value.is_empty() {
        fallback_value.to_string()
    } else {
        value
    }
}

fn format_time(t: SystemTime) -> String {
    let datetime: DateTime<Local> = t.into();
    datetime.format("%H:%M:%S").to_string()
}
