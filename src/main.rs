pub mod services;

use crate::services::nanowave_player_command::NanowavePlayerCommand;
use crate::services::nanowave_player_event::NanowavePlayerEvent;
use crate::services::start_services;

slint::include_modules!();




fn main() {
    let app = App::new().unwrap();

    let (ui_to_service_tx, ui_to_service_rx) = async_channel::unbounded::<NanowavePlayerCommand>();
    let (service_to_ui_tx, service_to_ui_rx) = async_channel::unbounded::<NanowavePlayerEvent>();

    // Start background services
    start_services(ui_to_service_rx, service_to_ui_tx);

    // UI → async (button click)
    app.on_send_clicked({
        let tx = ui_to_service_tx.clone();
        move |msg| {
            println!("send clicked: {}", msg);
            let cmd = NanowavePlayerCommand::PlayTest(msg.into());
            let send_result = tx.try_send(cmd);
            if let Err(err) = send_result {
                println!("send failed: {}", err);

            } else {
                println!("send success");
            }
        }
    });


        let app_weak = app.as_weak();

    std::thread::spawn(move || {
        smol::block_on(async move {
            
            
            while let Ok(player_event) = service_to_ui_rx.recv().await {
                let app = app_weak.clone();

                slint::invoke_from_event_loop(move || {
                    if let Some(app) = app.upgrade() {
                        match player_event {
                            NanowavePlayerEvent::OutputText(msg) => {
                                println!("outputText");
                                app.set_output_text(msg.into());
                            },
                            NanowavePlayerEvent::Position(position_as_str) =>  {
                                println!("position");
                                app.set_position(position_as_str.into());
                            }
                        }
                    }
                }).unwrap();
            }
        });
    });


    app.run().unwrap();
}

// fn main() -> Result<(), slint::PlatformError> {
//     let (tx, rx) = mpsc::channel::<PlayerCommand>();
//
//     let app = App::new()?;
//
//     app.run()
// }
