#![allow(clippy::needless_return)]

mod asr;
mod config;
mod disk;
mod gec;
mod indexer;
mod log;
mod ocr;
mod tests;
mod trie;
mod vidsplicer;

use indexer::Indexer;
use log::log;

use std::{
    io::{self, Write},
    path::{Path, PathBuf},
    process::exit,
    sync::{Arc, Mutex},
};
use threadpool::ThreadPool;
use vidsplicer::{
    ffmpeg_utils,
    ffmpeg_utils::{FFmpegResult, FFprobeResult},
    frames_iterator::VideoFramesIterator,
};

enum PipelineResult {
    Error(ExitCode),
    Success(Indexer),
}

pub enum ExitCode {
    Success,
    InvalidArgs,
    InvalidPath,
    SaveError(String),
    KeyframesError(String),
    WavConversionError(String),
    FFProbeError(String),
}
fn main() {
    // iterate_frames("data/patterns.mp4");
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        log(ExitCode::InvalidArgs);
        exit(1);
    }
    let path = PathBuf::from(&args[1]);
    match demo(&path) {
        PipelineResult::Error(code) => {
            eprintln!("Pipeline error, check the logs");
            log(code);
        }
        PipelineResult::Success(indexer) => init_ui(indexer),
    }
}
fn format_timestamp(seconds: u64) -> String {
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    let seconds = seconds % 60;
    format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
}

fn init_ui(mut index: Indexer) {
    print!("Enter word to search");
    println!("or enter 'q' to exit");
    let mut query = String::new();
    loop {
        print!("Search >>> ");
        let _ = io::stdout().flush();
        io::stdin().read_line(&mut query).expect("Failed to query");
        query = query.trim().to_lowercase();
        if query == "q" {
            break;
        }
        let timestamps = index.search(&query);
        if timestamps.is_empty() {
            println!("Not found");
            continue;
        }
        print!("[");
        timestamps
            .iter()
            .for_each(|t| print!("{}, ", format_timestamp(*t)));
        println!("]\n");
        query.clear();
    }
}
fn demo(video_path: &Path) -> PipelineResult {
    if !video_path.exists() {
        return PipelineResult::Error(ExitCode::InvalidPath);
    }

    //--------------Initialize Dump Directory--------------//
    let video_name = video_path.file_stem().unwrap();
    let dump_path = PathBuf::from("dump").join(video_name);
    let frames_path = dump_path.join("frames");

    disk::create_dump(&dump_path);

    //--------------Video Splicing--------------//
    match ffmpeg_utils::extract_keyframes(video_path, &frames_path) {
        FFmpegResult::Failure(error) => {
            return PipelineResult::Error(ExitCode::KeyframesError(error));
        }
        FFmpegResult::Success(_) => {
            println!("Successfully extracted keyframes...");
        }
    }
    match ffmpeg_utils::conv2wav(video_path, &dump_path) {
        FFmpegResult::Failure(error) => {
            return PipelineResult::Error(ExitCode::WavConversionError(error));
        }
        FFmpegResult::Success(_) => {
            println!("Successfully converted to WAV...");
        }
    }

    //------------------------------------------//
    let indexer = Indexer::new();

    //--------------OCR--------------//
    let fps = match ffmpeg_utils::get_video_metadata(video_path) {
        FFprobeResult::Failure(error) => {
            return PipelineResult::Error(ExitCode::FFProbeError(error))
        }
        FFprobeResult::Success(metadata) => metadata.fps,
    };

    let indexer = match ocr_dir(&frames_path, fps, indexer) {
        PipelineResult::Error(code) => return PipelineResult::Error(code),
        PipelineResult::Success(indexer) => {
            println!("Successfully applied OCR...");
            indexer
        }
    };
    //--------------ASR--------------//
    let indexer = match asr_audio(&dump_path.join("audio.wav"), indexer) {
        PipelineResult::Error(code) => return PipelineResult::Error(code),
        PipelineResult::Success(indexer) => {
            println!("Successfully applied ASR...");
            indexer
        }
    };

    //--------------Serialize and Save --------------//
    match disk::save_as_json(indexer.serialize(), &dump_path.join("index.json")) {
        Ok(_) => {
            println!("Successfully saved index...");
            println!("Done");
            return PipelineResult::Success(indexer);
        }
        Err(error) => {
            return PipelineResult::Error(ExitCode::SaveError(error.to_string()));
        }
    }
}
fn asr_audio(audio_path: &Path, mut indexer: Indexer) -> PipelineResult {
    let ctx = asr::asr(audio_path);
    for i in 0..ctx.full_n_segments() {
        let segment = ctx.full_get_segment_text(i).expect("failed to get segment");
        let start_timestamp = ctx.full_get_segment_t0(i);
        let _ = ctx.full_get_segment_t1(i);
        indexer.update(&segment, start_timestamp as u64);
    }
    return PipelineResult::Success(indexer);
}
fn ocr_dir(frames_path: &Path, fps: u64, indexer: Indexer) -> PipelineResult {
    const NUM_THREADS: usize = config::constants::NUM_THREADS as usize;
    let thread_pool = ThreadPool::new(NUM_THREADS);
    let apis_pool_arc = Arc::new(Mutex::new(make_apis_pool(NUM_THREADS)));
    let indexer_arc = Arc::new(Mutex::new(indexer));
    ///////////////////////////////
    let images = std::fs::read_dir(frames_path).unwrap();
    for entry in images {
        let entry = entry.unwrap();
        let file_name = entry.file_name();
        let timestamp = get_timestamp(file_name.to_str().unwrap(), fps);
        // get references
        let apis = apis_pool_arc.clone();
        let index = indexer_arc.clone();
        thread_pool.execute(move || {
            // pull an api from the apis pool and let go of the lock
            let mut apis_pool = apis.lock().unwrap();
            let mut api = apis_pool.pop().unwrap();
            drop(apis_pool);
            ///////////////////////////////
            let text = ocr::threaded_ocr(entry.path().to_str().unwrap(), &mut api);
            // put an api back in the pool and let go of the lock
            let mut apis_pool = apis.lock().unwrap();
            apis_pool.push(api);
            drop(apis_pool);
            ///////////////////////////////
            // let text = gec::correct(&text);
            let mut index_lock = index.lock().unwrap();
            index_lock.update(&text, timestamp);
        });
    }
    thread_pool.join();
    let mut res = indexer_arc.lock().unwrap();
    let res = std::mem::replace(&mut *res, Indexer::new());
    return PipelineResult::Success(res);

    fn make_apis_pool(num_threads: usize) -> Vec<leptess::LepTess> {
        let mut apis_pool: Vec<leptess::LepTess> = Vec::new();
        for _ in 0..num_threads {
            apis_pool.push(ocr::get_api());
        }
        return apis_pool;
    }
    /// extracts timestamp from frame number (image name)
    fn get_timestamp(filename: &str, fps: u64) -> u64 {
        let num_part = filename.trim_end_matches(".jpg");
        let frame_number: u64 = num_part.parse::<u64>().expect("Not a valid u64");
        return frame_number / fps;
    }
}
#[allow(unused)]
fn iterate_frames(video_path: &Path) -> ExitCode {
    let fi = VideoFramesIterator::new(video_path).unwrap();
    match ffmpeg_utils::get_video_dims(video_path) {
        Ok((width, height)) => {
            println!("Dimensions: {}x{}", width, height);
            for (i, frame) in fi.enumerate() {}
            return ExitCode::Success;
        }
        Err(error) => match error {
            FFprobeResult::Failure(error) => return ExitCode::FFProbeError(error),
            _ => return ExitCode::Success,
        },
    }
}
