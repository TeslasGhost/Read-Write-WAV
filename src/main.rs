extern crate hound;
extern crate sample;

use sample::{signal, Signal, ToFrameSliceMut};
use std::i16;


mod wav {
    pub const NUM_CHANNELS: usize = 2;
    pub const PATH: &'static str = "imperial_march.wav";
    pub const NEW_FILE: &'static str = "new_wav_file.wav";
    pub type Frame = [i16; NUM_CHANNELS];
}

const FRAMES_PER_BUFFER: u32 = 64;
const SAMPLE_RATE: f64 = 44_100.0;
const ORIGINAL_WAV_FILE: &'static str = "imperial_march.wav";
const NEW_WAV_FILE: &'static str = "new_wav_file.wav";
const SLICED_WAV_FILE: &'static str = "sliced_wav.wav";

#[allow(dead_code)]
fn main() {
   // uncomment the below line to read wav file and write to new wav file
  //  read_and_write();
    slice_the_wav_array();
  // read_and_write();


}

#[allow(dead_code)]
fn read_and_write() {
    let specification = hound::WavSpec {
    channels: 1,
    sample_rate: 22050,
    bits_per_sample: 16,
    sample_format: hound::SampleFormat::Int,
    };

    let mut reader = hound::WavReader::open(ORIGINAL_WAV_FILE).unwrap();
    let mut writer = hound::WavWriter::create(NEW_WAV_FILE, specification).unwrap();

    let read_sample = reader.samples::<i16>().fold(0.0, |read_sample, s| {
        let each_sample = s.unwrap() as f64;
        writer.write_sample((each_sample) as i16).unwrap();
        each_sample
    });

}



// Attempt to slice the array
#[allow(dead_code)]
fn slice_the_wav_array() {
    
    let slice_specification = hound::WavSpec {
    channels: 1,
    sample_rate: 22050,
    bits_per_sample: 16,
    sample_format: hound::SampleFormat::Int,
    };

    let frames: Vec<wav::Frame> = frames(wav::NEW_FILE);
    
    // Slice up the vector to capture 2 seconds worth of data.... each tick = 1 sec = 22.050 KHz
    let testslice = &frames[220500..264600];    // from :10 > :12 seconds    (@ 22.050 kHz sample)
    println!("testslice: {:?}", testslice);
    
    let mut slicewriter = hound::WavWriter::create(SLICED_WAV_FILE, slice_specification).unwrap();

}



// Given the file name, produces a Vec of `Frame`s which may be played back.
#[allow(dead_code)]
fn frames(file_name: &'static str) -> Vec<wav::Frame> {
    let mut reader = hound::WavReader::open(NEW_WAV_FILE).unwrap();
    let spec = reader.spec();
    let samples = reader.samples().map(|s| s.unwrap());
    
    signal::from_interleaved_samples::<_, wav::Frame>(samples)
        .from_hz_to_hz(spec.sample_rate as f64, SAMPLE_RATE as f64)
        .collect()

}


