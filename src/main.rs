pub mod services;
pub mod cli;
pub mod tracing;
pub mod service_config;

use ::tracing::{debug};
use clap::Parser;
use slint::{PhysicalSize, WindowSize};
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

const UI_DEFAULT_SCALE: f32 = 1.0;
const UI_DEFAULT_WIDTH:u32 = 368;
const UI_DEFAULT_HEIGHT:u32 = 552;

fn main() {
    // for debugging on wayland set env SLINT_BACKEND=winit-software to prevent font errors
    let cli = Cli::parse();
    scale_ui(&cli);
    debug_cli(&cli);
    init_tracing(&cli.env_filter);

    let app = App::new().unwrap();
    size_window(&cli, &app);

    let (command_tx, mut command_rx) = tokio::sync::mpsc::unbounded_channel::<NanowavePlayerCommand>();
    let (event_tx, mut event_rx) = tokio::sync::mpsc::unbounded_channel::<NanowavePlayerEvent>();

    // this uses only current thread to save CPU for embedded
    // could be even better to use enable_io().enable_time() instead of enable_all()
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    /*
    // for modern machines
    let runtime = tokio::runtime::Builder::new_multi_thread()
    .enable_all()
    .build()
    .unwrap();
     */

    let handle = runtime.handle().clone();

    // Start background services
    start_services(ServiceConfig::new(cli.audio_device.clone(), cli.sample_file.clone()), &mut command_rx, event_tx, handle.clone());

    // UI → async (button click)
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


    let app_weak = app.as_weak();

    handle.spawn(async move {
        while let Some(player_event) = event_rx.recv().await {
            let app = app_weak.clone();

            slint::invoke_from_event_loop(move || {
                if let Some(app) = app.upgrade() {
                    match player_event {
                        NanowavePlayerEvent::OutputText(msg) => {
                            app.set_output_text(msg.into());
                        }

                        NanowavePlayerEvent::Position(position) => {
                            app.set_position(position.into());
                        }
                    }
                }
            })
                .unwrap();
        }
    });

    /*
    tokio::spawn(async move {
        while let Ok(player_event) = service_to_ui_rx.recv().await {
            let app = app_weak.clone();

            slint::invoke_from_event_loop(move || {
                if let Some(app) = app.upgrade() {
                    match player_event {
                        NanowavePlayerEvent::OutputText(msg) => {
                            println!("outputText");
                            app.set_output_text(msg.into());
                        }

                        NanowavePlayerEvent::Position(position_as_str) => {
                            println!("position");
                            app.set_position(position_as_str.into());
                        }
                    }
                }
            })
                .unwrap();
        }
    });
    */

    app.run().unwrap();
}

fn debug_cli(cli: &Cli) {
    debug!("env_filter={}\naudio_device={}\nsample_file={}", cli.env_filter, cli.audio_device, cli.sample_file);
}

fn scale_ui(cli: &Cli) {
    // this does not seem to have any effect on UI elements?!
    let scale = if let Some(ui_scale) = cli.ui_scale && ui_scale > 0.0 {
        ui_scale
    } else {
        UI_DEFAULT_SCALE
    };
    if scale != 1.0
    {
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

    let window_size = WindowSize::Physical(PhysicalSize {
        width,
        height
    });
    app.window().set_size(window_size);
}