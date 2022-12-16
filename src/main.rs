use fundsp::hacker::*;

fn main() {
    const _HOUR: f64 = 60.00 * 10.00 * 6.00;
    let dim_freq: [f64; 36] = [
        16.35, 19.45, 23.12, 27.50, 32.70, 38.89, 46.25, 55.00, 65.41, 77.78, 92.50, 110.00,
        130.81, 155.56, 185.00, 220.00, 261.63, 311.13, 369.99, 440.00, 523.25, 622.25, 739.99,
        880.00, 1046.50, 1244.51, 1479.98, 1760.00, 2093.00, 2489.02, 2959.96, 3520.00, 4186.01,
        4978.03, 5919.91, 7040.00,
    ];
    let _pitch_values: [&str; 36] = [
        "c0", "eb0", "gb0", "a0", "c1", "eb1", "gb1", "a1", "c2", "eb2", "gb2", "a2", "c3", "eb3",
        "gb3", "a3", "c4", "eb4", "gb4", "a4", "c5", "eb5", "gb5", "a5", "c6", "eb6", "gb6", "a6",
        "c7", "eb7", "gb7", "a7", "c8", "eb8", "gb8", "a8",

    ];
    const FIVE_SECONDS: f64 = 5.00;
    let _wave2 = Wave64::render(44100.0, FIVE_SECONDS, &mut (pink()));
    for freq in dim_freq.iter() {
        let wave1 = Wave64::render(
            44100.0,
            FIVE_SECONDS,
            &mut (sine_hz(freq.to_f64()) - pink()),
        );
        let freq_str = format!("{}.wav", freq.to_string());
        wave1
            .save_wav32(freq_str)
            .expect("Could not save wav file.")
    }
}
