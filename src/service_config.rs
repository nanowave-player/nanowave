
#[derive(Clone, Debug)]
pub struct ServiceConfig {
    pub audio_device: String,
    pub sample_file: String,
}

impl ServiceConfig {
    pub fn new(audio_device: String, sample_file: String) -> Self {
        Self { audio_device, sample_file }
    }
}