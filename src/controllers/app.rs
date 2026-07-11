use slint::ComponentHandle;
use crate::{size_window, App, BROADCAST_CHANNEL_BUFFER_SIZE};
use crate::cli::Cli;
use crate::service_config::ServiceConfig;
use crate::services::nanowave_player_command::NanowavePlayerCommand;
use crate::services::nanowave_player_event::NanowavePlayerEvent;
use crate::services::start_services;

pub fn setup(app: &App, cli: &Cli) {

}