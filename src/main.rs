pub mod services;

use clap::Parser;
use crate::services::nanowave_player_command::NanowavePlayerCommand;
use crate::services::nanowave_player_event::NanowavePlayerEvent;
use crate::services::start_services;

use tracing::{debug, error, info, trace, warn, Level};
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::{fmt, EnvFilter};
use tracing_subscriber::fmt::writer::MakeWriterExt;

slint::include_modules!();



#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Cli {
    /// Sets the environment filter
    #[arg(long, default_value = "warn,nanowave_ui=trace")]
    env_filter: String,

    /// Sets the environment filter
    #[arg(long, default_value = "")]
    audio_device: String,

    #[arg(long, default_value = "")]
    sample_file: String,
}

#[derive(Clone, Debug)]
struct ServiceConfig {
    audio_device: String,
    sample_file: String,
}

impl ServiceConfig {
    pub fn new(audio_device: String, sample_file: String) -> Self {
        Self { audio_device, sample_file }
    }
}

fn init_tracing(env_filter: String) {
    let file_appender = RollingFileAppender::new(Rotation::DAILY, "logs", "nanowave");
    let filter = EnvFilter::new(
        env_filter
    );

    let subscriber = fmt()
        .with_writer(file_appender)
        .with_env_filter(filter)
        .with_ansi(false)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set global subscriber");

    /*
    trace!("trace test");
    debug!("debug test");
    info!("info test");
    warn!("warn test");
    error!("error test");

     */
}

fn main() {
    let cli = Cli::parse();
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

// fn main() -> Result<(), slint::PlatformError> {
//     let (tx, rx) = mpsc::channel::<PlayerCommand>();
//
//     let app = App::new()?;
//
//     app.run()
// }
