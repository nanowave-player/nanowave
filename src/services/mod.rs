use std::future;
use std::future::pending;
use async_channel::{Sender, Receiver};
use smol::lock::futures;
use crate::{PlayerCommand, PlayerEvent};

pub fn start_services(
    rx: Receiver<PlayerCommand>,
    tx: Sender<PlayerEvent>,
) {
    std::thread::spawn(move || {
        smol::block_on(async move {

            // Service 1: Echo service
            let _service1 = smol::spawn({
                let rx = rx.clone();
                let tx = tx.clone();

                async move {
                    while let Ok(msg) = rx.recv().await {
                        let response = PlayerEvent::Update("Service 1".into());
                        tx.send(response).await.unwrap();
                    }
                }
            }).detach();

            // Service 2: Logger service
            let _service2 = smol::spawn({
                let rx = rx.clone();

                async move {
                    while let Ok(cmd) = rx.recv().await {
                        match cmd {
                            PlayerCommand::PlayTest(id) => {
                                println!("[LOG] received: {}", id);
                            }
                        }
                    }
                }
            }).detach();

            // Keep executor alive forever
            pending::<()>().await;
        });
    });
}