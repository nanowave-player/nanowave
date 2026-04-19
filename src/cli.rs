use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Cli {
    pub ui_scale: Option<f32>,
    /// Sets the environment filter
    #[arg(long, default_value = "warn,nanowave=trace")]
    pub env_filter: String,

    /// Sets the environment filter
    #[arg(long, default_value = "")]
    pub audio_device: String,

    #[arg(long, default_value = "media/sample-3s.wav")]
    pub sample_file: String,
}