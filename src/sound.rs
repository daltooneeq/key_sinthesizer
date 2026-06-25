use cpal::{Device, SupportedStreamConfig, traits::DeviceTrait, traits::StreamTrait};

pub fn make(device: &Device, config: &SupportedStreamConfig, frequency: f32, sample_rate: f32, values: &Vec<f32>) -> cpal::Stream{
    let values = values.clone();
    let mut sample_clock = 0 as u64;

    let samples_per_period = sample_rate / frequency;

    let error_fn = |err| {eprintln!("Error with stream: {}", err)};

    let channels = config.channels() as usize;

    let stream = device.build_output_stream(config.config(), 
    move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
        for frame in data.chunks_mut(channels) {
            let phase = (sample_clock % samples_per_period as u64) as f32 / samples_per_period;

            let index = (phase * 200.0) as usize % 200; //без %200 может быть 200, а массив кончается на 199 индексе
            let value = values[index];
            for sample in frame.iter_mut() {
                *sample = value;
            }
            sample_clock += 1;
        }
    }, error_fn, None,).unwrap();
    
    stream

}