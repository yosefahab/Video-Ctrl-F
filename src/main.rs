#![allow(clippy::needless_return)]

mod asr;
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
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};
use threadpool::ThreadPool;
use vidsplicer::{ffmpeg_utils, frames_iterator::VideoFramesIterator};

enum PipelineResult {
    Error(ExitCode),
    Success(Indexer),
}
pub enum ExitCode {
    Success,
    InvalidArgs,
    InvalidPath,
    KeyframesError(String),
    WavConversionError(String),
    FFProbeError(String),
}
fn main() {
    // iterate_frames("data/patterns.mp4");

    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        log(ExitCode::InvalidArgs);
    }
    let path = PathBuf::from(&args[1]);
    match demo(&path) {
        PipelineResult::Error(code) => {
            log(code);
        }
        PipelineResult::Success(indexer) => init_ui(indexer),
    }
}
fn init_ui(mut index: Indexer) {
    print!("Enter word to search");
    println!("or enter 'q' to exit");
    let mut query = String::new();
    loop {
        std::io::stdin()
            .read_line(&mut query)
            .expect("Failed to query");
        query = query.trim().to_lowercase();
        if query == "q" {
            break;
        }
        let timestamps = index.search(&query);
        query.clear();

        if timestamps.is_empty() {
            println!("Not found");
        }
        println!("{query}: {:?}", timestamps);
    }
}
fn demo(video_path: &Path) -> PipelineResult {
    if !video_path.exists() {
        return PipelineResult::Error(ExitCode::InvalidPath);
    }
    let video_name = video_path.file_stem().unwrap();
    let dump_path = PathBuf::from("dump").join(video_name);
    let frames_path = PathBuf::from("dump").join("frames");
    ////////////////////// Initialize Dump Directory //////////////////////////
    disk::create_dump(&dump_path);

    ////////////////////// Video Splicing //////////////////////////
    match ffmpeg_utils::extract_keyframes(&video_path, &frames_path) {
        ffmpeg_utils::FFmpegResult::Failure(error) => {
            return PipelineResult::Error(ExitCode::KeyframesError(error));
        }
        ffmpeg_utils::FFmpegResult::Success(_) => {
            println!("Successfully extracted keyframes...");
        }
    }
    match ffmpeg_utils::conv2wav(&video_path, &dump_path) {
        ffmpeg_utils::FFmpegResult::Failure(error) => {
            return PipelineResult::Error(ExitCode::WavConversionError(error));
        }
        ffmpeg_utils::FFmpegResult::Success(_) => {
            println!("Successfully converted to WAV...");
        }
    }
    ////////////////////////////////////////////////
    let indexer = Indexer::new();
    ////////////////////////////////////////////////
    // println!("Applying OCR...");
    // let fps: u32 = ffmpeg_utils::get_video_metadata(video_path)
    //     .expect("Failed to get video metadata")
    //     .fps;
    // let indexer = ocr_dir(frames_path, fps, indexer);
    ////////////////////////////////////////////////
    println!("Applying ASR...");
    let index = asr_audio(&dump_path.join("audio.wav"), indexer);
    // println!("Saving index file...");
    // disk::save_as_json(dump_path).unwrap();

    println!("Done");
    return PipelineResult::Success(index);
}
fn asr_audio(audio_path: &Path, mut indexer: Indexer) -> Indexer {
    let ctx = asr::asr(audio_path);
    for i in 0..ctx.full_n_segments() {
        let segment = ctx.full_get_segment_text(i).expect("failed to get segment");
        let start_timestamp = ctx.full_get_segment_t0(i);
        let _ = ctx.full_get_segment_t1(i);
        indexer.update(&segment, start_timestamp as u64);
    }
    return indexer;
}
fn ocr_dir(frames_path: &str, fps: u32, indexer: Indexer) -> PipelineResult {
    const NUM_THREADS: usize = 7;
    let thread_pool = ThreadPool::new(NUM_THREADS);
    let apis_pool = make_apis_pool(NUM_THREADS);
    let index_arc = Arc::new(Mutex::new(indexer));
    ////////////////////////////////////////////////
    let images = std::fs::read_dir(frames_path).unwrap();
    for entry in images {
        let entry = entry.unwrap();
        let file_name = entry.file_name();
        let timestamp = get_timestamp(file_name.to_str().unwrap(), fps);
        // get references
        let apis = apis_pool.clone();
        let index = index_arc.clone();
        thread_pool.execute(move || {
            // pull an api from the apis pool and let go of the lock
            let mut apis_pool = apis.lock().unwrap();
            let api = apis_pool.pop().unwrap();
            drop(apis_pool);
            ////////////////////////////////////
            let text =
                ocr::threaded_ocr_from_disk(entry.path().to_str().unwrap(), api.lock().unwrap());
            // put an api back in the pool and let go of the lock
            let mut apis_pool = apis.lock().unwrap();
            apis_pool.push(api);
            drop(apis_pool);
            ////////////////////////////////////
            let text = gec::correct(&text);
            let mut index_lock = index.lock().unwrap();
            index_lock.update(&text, timestamp);
        });
    }
    thread_pool.join();
    let mut res = index_arc.lock().unwrap();
    return PipelineResult::Success(std::mem::replace(&mut *res, Indexer::new()));

    fn make_apis_pool(num_threads: usize) -> Arc<Mutex<Vec<Arc<Mutex<leptess::LepTess>>>>> {
        let mut apis_pool: Vec<Arc<Mutex<leptess::LepTess>>> = Vec::new();
        for _ in 0..num_threads {
            apis_pool.push(Arc::new(Mutex::new(ocr::get_api())));
        }
        return Arc::new(Mutex::new(apis_pool));
    }
    fn get_timestamp(filename: &str, fps: u32) -> u64 {
        let num_part = filename.trim_end_matches(".jpg");
        let frame_number: u64 = num_part.parse::<u64>().expect("Not a valid u64");
        return frame_number / fps as u64;
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
            ffmpeg_utils::FFprobeResult::Failure(error) => return ExitCode::FFProbeError(error),
            _ => return ExitCode::Success,
        },
    }
}
