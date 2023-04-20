use hound::{SampleFormat, WavReader};
use std::{path::Path, process::Command};
use whisper_rs::{FullParams, SamplingStrategy, WhisperContext};

pub fn asr(audio_path: &Path) -> WhisperContext {
    let whisper_path =
        Path::new("/Users/youssefahab/Development/GP/Video-Ctrl-F/models/cpp_whisper");

    let model_path = Path::new(whisper_path).join("models/ggml-base.en.bin");

    // whisper_cpp(&audio_path, whisper_path, &model_path);
    // println!("DONE");
    return whisper_rs(&audio_path, &model_path);
}
fn whisper_rs(audio_path: &Path, model_path: &Path) -> WhisperContext {
    let original_samples = parse_wav_file(audio_path);
    let samples = whisper_rs::convert_integer_to_float_audio(&original_samples);
    let mut ctx = WhisperContext::new(&model_path.to_string_lossy()).expect("failed to open model");
    let mut params = FullParams::new(SamplingStrategy::default());
    params.set_n_threads(7);
    params.set_language(Some("en"));
    params.set_translate(false);
    params.set_print_special(false);
    params.set_print_progress(false);
    params.set_print_realtime(false);
    ctx.full(params, &samples)
        .expect("failed to convert samples");

    return ctx;
}
fn parse_wav_file(path: &Path) -> Vec<i16> {
    let reader = WavReader::open(path).expect("failed to read file");

    if reader.spec().channels != 1 {
        panic!("expected mono audio file");
    }
    if reader.spec().sample_format != SampleFormat::Int {
        panic!("expected integer sample format");
    }
    if reader.spec().sample_rate != 16000 {
        panic!("expected 16KHz sample rate");
    }
    if reader.spec().bits_per_sample != 16 {
        panic!("expected 16 bits per sample");
    }

    return reader
        .into_samples::<i16>()
        .map(|x| x.expect("sample"))
        .collect::<Vec<_>>();
}

#[allow(unused)]
fn whisper_cpp(audio_path: &Path, whisper_path: &Path, model_path: &Path) {
    let output = Command::new(whisper_path.join("main"))
        .args([
            audio_path.to_str().unwrap(),
            "-ml",
            "1",
            "--model",
            model_path.to_str().unwrap(),
            "--threads",
            &7.to_string(),
            "-l",
            "en",
            "-oj",
        ])
        .output()
        .expect("failed to execute process");
    println!("status: {}", output.status);
}
