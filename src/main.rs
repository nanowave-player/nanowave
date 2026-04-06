pub mod services;

use std::sync::mpsc;
use crate::services::start_services;

slint::include_modules!();

pub enum PlayerCommand {
    PlayTest(String),
}

pub enum PlayerEvent {
    OutputText(String),
    Position(String),
}
fn main() {
    let app = App::new().unwrap();

    let (ui_to_service_tx, ui_to_service_rx) = async_channel::unbounded::<PlayerCommand>();
    let (service_to_ui_tx, service_to_ui_rx) = async_channel::unbounded::<PlayerEvent>();

    // Start background services
    start_services(ui_to_service_rx, service_to_ui_tx);

    // UI → async (button click)
    app.on_send_clicked({
        let tx = ui_to_service_tx.clone();
        move |msg| {
            println!("send clicked: {}", msg);
            let cmd = PlayerCommand::PlayTest(msg.into());
            let send_result = tx.try_send(cmd);
            if let Err(err) = send_result {
                println!("send failed: {}", err);

            } else {
                println!("send success");
            }
        }
    });

    // Async → UI (update text safely)
    {
        let app_weak = app.as_weak();

        std::thread::spawn(move || {
            smol::block_on(async move {
                while let Ok(player_event) = service_to_ui_rx.recv().await {
                    let app = app_weak.clone();

                    slint::invoke_from_event_loop(move || {
                        if let Some(app) = app.upgrade() {
                            match player_event {
                                PlayerEvent::OutputText(msg) => {
                                    println!("outputText");
                                    app.set_output_text(msg.into());
                                },
                                PlayerEvent::Position(position_as_str) =>  {
                                    println!("position");
                                    app.set_position(position_as_str.into());
                                }
                            }
                        }
                    }).unwrap();
                }
            });
        });
    }

    app.run().unwrap();
}

// fn main() -> Result<(), slint::PlatformError> {
//     let (tx, rx) = mpsc::channel::<PlayerCommand>();
//
//     let app = App::new()?;
//
//     app.run()
// }
