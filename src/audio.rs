use hound::{WavWriter, WavSpec, SampleFormat};
use std::f32::consts::PI;

const SAMPLE_RATE: u32 = 44100; 
const FREQUENCY: f32 = 600.0; // Morse code tone (Hz)
const AMPLITUDE: f32 = 0.5; 

const UNIT: f32 = 0.1; // Base unit (100ms)
const DIT: f32 = UNIT;         
const DAH: f32 = UNIT * 3.0;    
const SPACE: f32 = UNIT;        
const LETTER_SPACE: f32 = UNIT * 3.0; 
const WORD_SPACE: f32 = UNIT * 7.0;   

fn generate_tone(duration: f32) -> Vec<i16> {
    let samples = (duration * SAMPLE_RATE as f32) as usize;
    (0..samples)
        .map(|i| {
            let t = i as f32 / SAMPLE_RATE as f32;
            let sample = AMPLITUDE * (2.0 * PI * FREQUENCY * t).sin();
            (sample * i16::MAX as f32) as i16
        })
        .collect()
}

fn generate_silence(duration: f32) -> Vec<i16> {
    let samples = (duration * SAMPLE_RATE as f32) as usize;
    vec![0; samples]
}

pub fn morse_audio(text: &str) {
    let spec = WavSpec {
        channels: 1,
        sample_rate: SAMPLE_RATE,
        bits_per_sample: 16,
        sample_format: SampleFormat::Int,
    };

    let mut writer = WavWriter::create("morse_code.wav", spec).unwrap();

    for morse in text.split(" ") {
        for symbol in morse.chars() {
            let sound = match symbol {
                '.' => generate_tone(DIT),
                '-' => generate_tone(DAH),
                _ => generate_silence(WORD_SPACE),
            };
            
            for sample in sound {
                writer.write_sample(sample).unwrap();
            }
            for sample in generate_silence(SPACE) {
                writer.write_sample(sample).unwrap();
            }
        }

        for sample in generate_silence(LETTER_SPACE) {
            writer.write_sample(sample).unwrap();
        }
    }
}
