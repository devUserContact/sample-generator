use fundsp::hacker::*;
use std::fs;

fn main() {
    const _HOUR: f64 = 60.00 * 10.00 * 6.00;
    let dim_freq: [f64; 36] = [
        16.35, 19.45, 23.12, 27.50, 32.70, 38.89, 46.25, 55.00, 65.41, 77.78, 92.50, 110.00,
        130.81, 155.56, 185.00, 220.00, 261.63, 311.13, 369.99, 440.00, 523.25, 622.25, 739.99,
        880.00, 1046.50, 1244.51, 1479.98, 1760.00, 2093.00, 2489.02, 2959.96, 3520.00, 4186.01,
        4978.03, 5919.91, 7040.00,
    ];
    let pitch_values: [&str; 36] = [
        "0c", "0eb", "0gb", "0xa", "1c", "1eb", "1gb", "1xa", "2c", "2eb", "2gb", "2xa", "3c",
        "3eb", "3gb", "3xa", "4c", "4eb", "4gb", "4xa", "5c", "5eb", "5gb", "5xa", "6c", "6eb",
        "6gb", "6xa", "7c", "7eb", "7gb", "7xa", "8c", "8eb", "8gb", "8xa",
    ];
    fs::create_dir("samples").unwrap();

    const TIME: f64 = 2.00;
    let _wave2 = Wave64::render(44100.0, TIME, &mut (pink()));
    for index in 0..dim_freq.len() {
        let mut wave1 = Wave64::render(
            44100.0,
            TIME,
            &mut ((sine_hz(dim_freq[index].to_f64()) - saw_hz(dim_freq[index].to_f64() - 4.0))
                - oversample(pink())),
        );
        wave1.fade_in(0.03);
        wave1.fade_out(2.0);
        wave1.normalize();
        let freq_str = format!("samples/{}.wav", pitch_values[index]);
        wave1
            .save_wav32(freq_str)
            .expect("Could not save wav file.")
    }
}
