use sdl2::{
    audio::{AudioCallback, AudioDevice, AudioSpecDesired},
    AudioSubsystem,
};

pub struct AudioManager {
    _audio_subsystem: AudioSubsystem,
    device: AudioDevice<SquareWave>,
    duration: u32, // Remaining duration to play audio
}

impl AudioManager {
    pub fn new(audio_subsystem: AudioSubsystem) -> AudioManager {
        let desired_spec = AudioSpecDesired {
            freq: Some(44100),
            channels: Some(1), // mono
            samples: None,     // default sample size
        };

        let device = audio_subsystem
            .open_playback(None, &desired_spec, |spec| {
                // initialize the audio callback
                SquareWave {
                    phase_inc: 440.0 / spec.freq as f32,
                    phase: 0.0,
                    volume: 0.25,
                }
            })
            .expect("Could not create audio player");

        AudioManager {
            _audio_subsystem: audio_subsystem,
            device,
            duration: 0,
        }
    }

    pub fn play(&mut self, duration: u32) {
        self.device.resume();
        self.duration = duration;
    }

    pub fn tick(&mut self) {
        if self.duration > 0 {
            self.duration -= 1;
        }

        if self.duration == 0 {
            self.device.pause();
        }
    }
}

pub struct SquareWave {
    phase_inc: f32,
    phase: f32,
    volume: f32,
}

impl AudioCallback for SquareWave {
    type Channel = f32;

    fn callback(&mut self, out: &mut [Self::Channel]) {
        for x in out.iter_mut() {
            *x = if self.phase <= 0.5 {
                self.volume
            } else {
                -self.volume
            };
            self.phase = (self.phase + self.phase_inc) % 1.0;
        }
    }
}
