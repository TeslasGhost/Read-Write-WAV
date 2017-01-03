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
 //  slice_the_wav_array();
 //  calculate_total_frequency_cycles_timeslice(22050, 10);

 calculate_frequency_start(10, 22050);
 calculate_frequency_end(20, 22050);

}

#[allow(dead_code)]
fn read_and_write() {
    let specification = hound::WavSpec {
                                            channels: 1,
                                            sample_rate: 22050,
                                            bits_per_sample: 16,
                                            sample_format: hound::SampleFormat::Int, };

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
                                                sample_format: hound::SampleFormat::Int, };

    let frames: Vec<wav::Frame> = frames(wav::NEW_FILE);
    
    // Slice up the vector to capture 2 seconds worth of data.... each tick = 1 sec = 22.050 KHz
    let testslice = &frames[220500..264600];    // from :10 > :12 seconds    (@ 22.050 kHz sample)

    // Raw pointer to the first element of the array
    &testslice[0] as *const i16;

    let mut slicewriter = hound::WavWriter::create(SLICED_WAV_FILE, slice_specification).unwrap();

    // 44100 being 2 seconds worth...
    for t in testslice.iter() {
        let slice_sample = t;
        // write only the one channel to satisfy the write_sample() trait with reference to the spec
        slicewriter.write_sample(slice_sample[0]).unwrap();
    }

  //  println!("testslice: {:?}", testslice);
}

// Given the file name, produces a Vector of 'Frames' which may be played back.
#[allow(dead_code)]
fn frames(file_name: &'static str) -> Vec<wav::Frame> {
    let mut reader = hound::WavReader::open(NEW_WAV_FILE).unwrap();
    let spec = reader.spec();
    let samples = reader.samples().map(|s| s.unwrap());
    
    signal::from_interleaved_samples::<_, wav::Frame>(samples)
        .from_hz_to_hz(spec.sample_rate as f64, SAMPLE_RATE as f64)
        .collect()
}

// Function to do all...
#[allow(dead_code)]
fn read_slice_write(filename: &'static str, start: i16, finish: i16, sample_rate: i32) {

    let origin_wav_file = filename;
    let time_slice_in_seconds = calculate_total_time_slice(start, finish);
    let frequency_cycles = calculate_total_frequency_cycles_timeslice(sample_rate, time_slice_in_seconds as i32);
    let casted_sample_rate = sample_rate as u32;

    let specification = hound::WavSpec {
                                            channels: 1,
                                            sample_rate: casted_sample_rate,
                                            bits_per_sample: 16,
                                            sample_format: hound::SampleFormat::Int, };

    let mut reader = hound::WavReader::open(origin_wav_file).unwrap();
    let mut writer = hound::WavWriter::create(NEW_WAV_FILE, specification).unwrap();

    let read_sample = reader.samples::<i16>().fold(0.0, |read_sample, s| {
        let each_sample = s.unwrap() as f64;
        writer.write_sample((each_sample) as i16).unwrap();
        each_sample
    });

    let start_frame = calculate_frequency_start(start, sample_rate);
    let end_frame = calculate_frequency_end(start, sample_rate);


    // Fill Vector array with the original WAV values
    let frames: Vec<wav::Frame> = frames(wav::NEW_FILE);

    // Slice up the vector to capture 2 seconds worth of data.... each tick = 1 sec = 22.050 KHz
    // let testslice = &frames[220500..264600];    // from :10 > :12 seconds    (@ 22.050 kHz sample)
    let the_slice = &frames[start_frame..end_frame];

}

//  Pre: Requires start and end time in seconds
// Post: Returns the difference in time slice
#[allow(dead_code)]
fn calculate_total_time_slice (begin: i16, end: i16) -> i16 {
    let total_time_slice = end - begin;
    total_time_slice
}

//  Pre: Requires the sample rate in KHz and total time slice
// Post: Returns the total frequency cycles 
#[allow(dead_code)]
fn calculate_total_frequency_cycles_timeslice(sample_rate: i32, total_time_in_seconds: i32) -> i32 {
    let total_frequency_cycles = total_time_in_seconds * sample_rate;
    total_frequency_cycles
}

//  Pre: Requires a begin time in seconds and sample rate
// Post: Returns the value of the first element in WAV array for slice
#[allow(dead_code)]
fn calculate_frequency_start(begin: i16, sample_rate: i32) -> i32 {
    let start_frequency = ((begin as i32) * (sample_rate));
    start_frequency
}

//  Pre: Requires an end time in seconds and sample rate
// Post: Returns the value of the final element in WAV array for slice
#[allow(dead_code)]
fn calculate_frequency_end(end: i16, sample_rate: i32) -> i32 {
    let end_frequency = ((end as i32) * (sample_rate));
    end_frequency
}
