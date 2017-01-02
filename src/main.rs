extern crate hound;
extern crate sample;

use sample::{signal, Signal, ToFrameSliceMut};
use std::i16;


mod wav {
    pub const NUM_CHANNELS: usize = 2;
    pub const PATH: &'static str = "imperial_march.wav";
    pub type Frame = [i16; NUM_CHANNELS];
}

const FRAMES_PER_BUFFER: u32 = 64;
const SAMPLE_RATE: f64 = 44_100.0;

fn main() {
    
    let specification = hound::WavSpec {
    channels: 1,
    sample_rate: 22050,
    bits_per_sample: 16,
    sample_format: hound::SampleFormat::Int,
    };

    let mut reader = hound::WavReader::open("imperial_march.wav").unwrap();
    let mut writer = hound::WavWriter::create("new_wav_file.wav", specification).unwrap();

    let read_sample = reader.samples::<i16>().fold(0.0, |read_sample, s| {
        let each_sample = s.unwrap() as f64;
        writer.write_sample((each_sample) as i16).unwrap();
        each_sample
    });

}
