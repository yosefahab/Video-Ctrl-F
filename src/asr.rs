use crate::vidsplicer::ffmpeg_utils;
use std::{
    io::Read,
    path::Path,
    process::{Command, Stdio},
};

pub fn asr(video_path: &str, output_path: &str) {
    ffmpeg_utils::conv2wav(video_path, output_path).expect("Failed to convert to wave");
    let audio_path = Path::new(output_path).join("audio.wav");
    cpp_whisper_piped(audio_path.to_str().unwrap());
    cpp_whisper(audio_path.to_str().unwrap());
}

fn cpp_whisper_piped(audio_path: &str) {
    let output = Command::new("../models/cpp_whisper/main")
        .args([
            audio_path,
            "-ml",
            "1",
            "--model",
            "../models/cpp_whisper/models/ggml-base.en.bin",
            "--threads",
            "7",
            "-l",
            "en",
            "-ocsv",
        ])
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let mut buffer = [0u8; 1024];
    output.stdout.unwrap().read(&mut buffer);
    println!("////////////////////////////////////");
    println!("{:?}", buffer);
    // let size = match output.stdout.as_mut().unwrap().read(&mut buffer) {
    //     Ok(size) if size > 0 => size,
    //     _ => {
    //         // The child process has finished or there was an error
    //         return ();
    //     }
    // };
    println!("////////////////////////////////////");
}
fn cpp_whisper(audio_path: &str) {
    let output = Command::new("../models/cpp_whisper/main")
        .args([
            audio_path,
            "-ml",
            "1",
            "--model",
            "../models/cpp_whisper/models/ggml-base.en.bin",
            "--threads",
            "7",
            "-l",
            "en",
            "-ocsv",
        ])
        .output()
        .expect("failed to execute process");
    println!("status: {}", output.status);
}
