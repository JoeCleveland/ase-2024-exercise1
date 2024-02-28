use std::{fs::File, io::Write};

use vibrato::Vibrato;

mod ring_buffer;
mod vibrato;
mod lfo;

fn show_info() {
    eprintln!("MUSI-6106 Assignment Executable");
    eprintln!("(c) 2024 Stephen Garrett & Ian Clester");
}

fn main() {
   show_info();

    // Parse command line arguments
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <input wave filename> <output text filename>", args[0]);
        return
    }

    // Open the input wave file
    let mut reader = hound::WavReader::open(&args[1]).unwrap();
    let spec = reader.spec();
    let channels = spec.channels;
    let block_size = 1024;

    let writer_spec = hound::WavSpec {
        channels: channels,
        sample_rate: spec.sample_rate,
        bits_per_sample: 32,
        sample_format: hound::SampleFormat::Float
    };
    let mut writer = hound::WavWriter::create(&args[2], writer_spec).unwrap();

    let mut vibratos: Vec<Vibrato> = Vec::new();

    for c in 0..channels {
        vibratos.push(
            Vibrato::new(1.0, 1.0, spec.sample_rate as usize)
        );
    }
    let mut input_buffer: Vec<Vec<f32>> = Vec::new();
    let mut output_buffer: Vec<Vec<f32>> = Vec::new();
    for i in 0..channels {
        input_buffer.push(vec![0f32; block_size]);
        output_buffer.push(vec![0f32; block_size]);
    }

    // Read audio data and write it to the output text file (one column per channel)
    // let mut out = File::create(&args[2]).expect("Unable to create file");

    for (i, sample) in reader.samples::<i16>().enumerate() {
        let sample = sample.unwrap() as f32 / (1 << 15) as f32;
        input_buffer[i % channels as usize].push(sample);

        // Once the input buffer is full up, process the block
        if (i + 1) / channels as usize % block_size == 0 {
            for c in 0..channels as usize {
                vibratos[c].process_block(input_buffer[c].as_slice(), output_buffer[c].as_mut_slice());
                input_buffer[c].clear();
            }

            for i in 0..block_size {
                for c in 0..channels as usize {
                    writer.write_sample(output_buffer[c][i]);
                }
            }

            for c in 0..channels as usize {
                output_buffer[c].clear();
                input_buffer[c].clear();
            }
        }
    }
}
