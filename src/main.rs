pub mod services;
pub mod cli;
pub mod tracing;
pub mod service_config;

use ::tracing::{debug};
use clap::Parser;

use crate::services::nanowave_player_command::NanowavePlayerCommand;
use crate::services::nanowave_player_event::NanowavePlayerEvent;
use crate::services::start_services;
use crate::cli::Cli;
// use tracing::{debug, error, info, trace, warn, Level};
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::{fmt, EnvFilter};
use crate::service_config::ServiceConfig;
use crate::tracing::init_tracing;

slint::include_modules!();

const DEFAULT_SCALE: f32 = 1.0;

fn main() {
    let cli = Cli::parse();

    // this does not seem to have any effect on UI elements?!
    let scale = if let Some(ui_scale) = cli.ui_scale && ui_scale > 0.0 {
        ui_scale
    } else {
        DEFAULT_SCALE
    };
    if scale != 1.0
    {
        // Must be set before any Slint window is created.
        // SAFETY: single-threaded at this point in startup.
        unsafe {
            std::env::set_var("SLINT_SCALE_FACTOR", scale.to_string());
        }
    }

    debug!("env_filter={}\naudio_device={}\nsample_file={}", cli.env_filter, cli.audio_device, cli.sample_file);
    init_tracing(cli.env_filter);

    let app = App::new().unwrap();

    let (ui_to_service_tx, ui_to_service_rx) = async_channel::unbounded::<NanowavePlayerCommand>();
    let (service_to_ui_tx, service_to_ui_rx) = async_channel::unbounded::<NanowavePlayerEvent>();

    // Start background services
    start_services(ServiceConfig::new(cli.audio_device.clone(), cli.sample_file.clone()), ui_to_service_rx, service_to_ui_tx);

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
