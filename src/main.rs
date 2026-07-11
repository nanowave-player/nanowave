pub mod cli;
pub mod service_config;
pub mod services;
pub mod tracing;
mod controllers;

use crate::cli::Cli;
use crate::services::nanowave_player_command::NanowavePlayerCommand;
use crate::services::nanowave_player_event::NanowavePlayerEvent;
use crate::services::start_services;
use ::tracing::debug;
use clap::Parser;
use slint::{PhysicalSize, WindowSize};
// use tracing::{debug, error, info, trace, warn, Level};
use crate::service_config::ServiceConfig;
use crate::tracing::init_tracing;
// use tracing_appender::rolling::{RollingFileAppender, Rotation};
// use tracing_subscriber::{//EnvFilter, fmt};

slint::include_modules!();

const BROADCAST_CHANNEL_BUFFER_SIZE: usize = 64;

const UI_DEFAULT_SCALE: f32 = 1.0;
const UI_DEFAULT_WIDTH: u32 = 368;
const UI_DEFAULT_HEIGHT: u32 = 552;

// for debugging on wayland set env SLINT_BACKEND=winit-software to prevent font embedding errors
fn main() {
    let cli = Cli::parse();
    scale_ui(&cli);
    debug_cli(&cli);
    init_tracing(&cli.env_filter);

    let app = App::new().unwrap();


    size_window(&cli, &app);

    let (command_tx, command_rx) =
        tokio::sync::broadcast::channel::<NanowavePlayerCommand>(BROADCAST_CHANNEL_BUFFER_SIZE);
    let (event_tx, event_rx) =
        tokio::sync::broadcast::channel::<NanowavePlayerEvent>(BROADCAST_CHANNEL_BUFFER_SIZE);

    // new_multi_thread() is required, when UI and start_services should run simultaneously.
    // current_thread() will result in one of the threads won't do any work
    // possible improvement: use enable_io().enable_time() instead of enable_all()
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    let handle = runtime.handle().clone();
    start_services(
        ServiceConfig::new(cli.audio_device.clone(), cli.sample_file.clone()),
        event_tx.clone(),
        command_rx.resubscribe(),
        handle.clone(),
    );

    let app_weak = app.as_weak();

    // let (event_tx, event_rx) = tokio::sync::broadcast::channel::<NanowavePlayerEvent>(BROADCAST_CHANNEL_BUFFER_SIZE);
    handle.spawn(async move {
        let event_rx = &mut event_rx.resubscribe();

        let app = app_weak.clone();

        loop {
            match event_rx.recv().await {
                Ok(player_event) => {
                    let app = app.clone();

                    slint::invoke_from_event_loop(move || {
                        if let Some(app) = app.upgrade() {
                            match player_event {
                                NanowavePlayerEvent::OutputText(msg) => {
                                    // app.set_output_text(msg.into());
                                }
                                NanowavePlayerEvent::Position(pos) => {
                                    // app.set_position(pos.into());
                                }
                            }
                        }
                    })
                        .unwrap();
                }

                Err(tokio::sync::broadcast::error::RecvError::Lagged(n)) => {
                    eprintln!("Lagged by {n} messages");
                }

                Err(tokio::sync::broadcast::error::RecvError::Closed) => break,
            }
        }
    });

    controllers::app::setup(&app, &cli);
    controllers::home::setup(&app);


    /*
    app.on_send_clicked({
        let tx = command_tx.clone();
        move |msg| {
            println!("send clicked: {}", msg);
            let cmd = NanowavePlayerCommand::PlayTest(msg.into());
            let send_result = tx.send(cmd);
            if let Err(err) = send_result {
                println!("send failed: {}", err);
            } else {
                println!("send success");
            }
        }
    });
    */



    app.run().unwrap();
}

fn debug_cli(cli: &Cli) {
    debug!(
        "env_filter={}\naudio_device={}\nsample_file={}",
        cli.env_filter, cli.audio_device, cli.sample_file
    );
}

fn scale_ui(cli: &Cli) {
    // this does not seem to have any effect on UI elements?!
    let scale = if let Some(ui_scale) = cli.ui_scale
        && ui_scale > 0.0
    {
        ui_scale
    } else {
        UI_DEFAULT_SCALE
    };
    if scale != 1.0 {
        // Must be set before any Slint window is created.
        // SAFETY: single-threaded at this point in startup.
        unsafe {
            std::env::set_var("SLINT_SCALE_FACTOR", scale.to_string());
        }
    }
}

fn size_window(cli: &Cli, app: &App) {
    let width = cli.ui_width.unwrap_or(UI_DEFAULT_WIDTH);
    let height = cli.ui_height.unwrap_or(UI_DEFAULT_HEIGHT);

    let window_size = WindowSize::Physical(PhysicalSize { width, height });
    app.window().set_size(window_size);
}
