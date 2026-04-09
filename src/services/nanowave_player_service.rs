use std::fs::File;
use std::path::Path;
use std::str::FromStr;
use std::time::{Duration, SystemTime};
use async_channel::{Receiver, Sender};
use rodio::cpal::{BufferSize, DeviceId};
use rodio::cpal::traits::HostTrait;
use rodio::{DeviceSinkBuilder, DeviceTrait, Source};
use smol::Timer;
use tracing::{debug, warn};
use crate::services::format_time;
use crate::services::nanowave_player_command::NanowavePlayerCommand;
use crate::services::nanowave_player_event::NanowavePlayerEvent;

pub struct NanowavePlayerService {
    audio_device: String,
    sample_file: String,
}

impl NanowavePlayerService {
    pub fn new(audio_device: String, sample_file: String) -> NanowavePlayerService {
        Self {
            audio_device,
            sample_file,
        }
    }

    pub async fn run(&self, rx: Receiver<NanowavePlayerCommand>, tx: Sender<NanowavePlayerEvent>) {
        loop {
            while let Ok(cmd) = rx.recv().await {
                println!("Command received...");
                match cmd {
                    NanowavePlayerCommand::PlayTest(msg) => {
                        println!("PlayTest received: {}", msg);
                        Self::playtest(self.audio_device.clone(), self.sample_file.clone()).await;
                        let response = NanowavePlayerEvent::OutputText(format!("{}: {}", format_time(SystemTime::now()), msg).into());
                        tx.send(response).await.unwrap();
                    }
                }
            }
        }
    }
    async fn playtest(audio_device: String, audio_file: String) -> Result<(), slint::PlatformError>{


        let device_ids = vec![audio_device];

        let host = rodio::cpal::default_host();

        let device_id_strings = device_ids.to_vec();
        let mut device_option = host.default_output_device();
        let device_ids: Vec<DeviceId> = device_id_strings
            .into_iter()
            .filter_map(|s| DeviceId::from_str(&s).ok())
            .collect();

        for device_id in device_ids {
            let dev = host.device_by_id(&device_id);
            if dev.is_some() {
                debug!("attempt get device by id: {} -> success", device_id);

                device_option = dev;
                break;
            } else {
                debug!("attempt get device by id: {} -> failed", device_id);
            }
        }


        debug!("trying to connect audio device...");
        if let Some(device) = device_option {
            debug!(" => device {:?} is available", device.id());

            if let Ok(builder) = DeviceSinkBuilder::from_device(device)
                && let Ok(stream) = builder.with_buffer_size(BufferSize::Fixed(512)).open_stream(){
                debug!("builder.stream.buffer_size: {:?}", stream.config().buffer_size());

                let sink = rodio::Player::connect_new(stream.mixer());

                sink.clear();
                let waves = vec![230f32, 270f32, 330f32, 270f32, 230f32];
                for w in waves {
                    let source = rodio::source::SineWave::new(w).amplify(0.1);
                    sink.append(source);
                    sink.play();
                    // sleep(Duration::from_millis(200));
                    Timer::after(Duration::from_secs(1)).await;
                    sink.stop();
                    sink.clear();
                }

                let location = audio_file.as_str();

                let path = Path::new(location);
                let file_result = File::open(path);

                if let Ok(file) = file_result {
                    let decoder_result = rodio::Decoder::try_from(file);
                    if let Ok(decoder) = decoder_result{
                        sink.clear();
                        sink.append(decoder);
                        sink.play();
                        // sink.sleep_until_end();
                        Timer::after(Duration::from_secs(3)).await;
                    } else {
                        warn!("error on decoding");
                    }
                } else {
                    warn!("error on file result");
                }

            } else {
                warn!("failed to open audio stream");
            }
        } else {
            warn!("failed to find audio device");
        }

        Ok(())
    }


}