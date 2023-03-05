#![allow(unused)]
#![allow(clippy::needless_return)]

mod vidsplicer;
use vidsplicer::ffmpeg_utils;
use vidsplicer::frames_iterator::VideoFramesIterator;

mod asr;
mod disk;
mod gec;
mod ocr;
mod tests;
mod trie;

mod indexer;
use indexer::lookup;

use std::{
    fmt::format,
    sync::{Arc, Mutex},
};
use threadpool::ThreadPool;

fn main() {
    // iterate_frames("data/patterns.mp4");
    demo();
}
fn demo() {
    const FILENAME: &str = "patterns";
    const DUMP_PATH: &str = "dump/patterns";
    const FRAMES_PATH: &str = "dump/patterns/frames";
    const VIDEO_PATH: &str = "data/patterns.mp4";
    disk::create_dump(FILENAME);

    println!("Extracting keyframes...");
    ffmpeg_utils::extract_keyframes(VIDEO_PATH, FRAMES_PATH).expect("Failed to extract keyframes");

    println!("Applying OCR...");
    let fps: u32 = ffmpeg_utils::get_video_metadata(VIDEO_PATH)
        .expect("Failed to get video metadata")
        .fps;
    ocr_dir(FRAMES_PATH, fps);

    // println!("Applying ASR...");
    // asr::asr(VIDEO_PATH, DUMP_PATH);
    println!("Done");
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
        let timestamps = lookup::search(&query);
        println!("{query}: {:?}", timestamps);

        query.clear();
    }
    // println!("Saving index file...");
    // disk::save_as_json(DUMP_PATH).unwrap();
}
fn get_timestamp(filename: &str, fps: u32) -> u64 {
    let num_part = filename
        .trim_start_matches("frame-")
        .trim_end_matches(".jpg");
    let frame_number: u64 = num_part.parse::<u64>().expect("Not a valid u64");
    return frame_number / fps as u64;
}
fn ocr_dir(dir_path: &str, fps: u32) {
    const NUM_THREADS: usize = 7;
    let thread_pool = ThreadPool::new(NUM_THREADS);
    let apis_pool = make_apis_pool(NUM_THREADS);

    ////////////////////////////////////////////////
    let images = std::fs::read_dir(dir_path).unwrap();
    for entry in images {
        let entry = entry.unwrap();
        // if !entry.path().is_file() || entry.path().extension() != Some("jpg".as_ref()) {
        //     continue;
        // }
        let file_name = entry.file_name();
        let timestamp = get_timestamp(file_name.to_str().unwrap(), fps);

        // get references
        let apis = apis_pool.clone();

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
            lookup::update(&text, timestamp);
            // disk::save_as_txt(&text, timestamp, dir_path).expect("Failed to save text file");
        });
    }
    thread_pool.join();
}
fn make_apis_pool(num_threads: usize) -> Arc<Mutex<Vec<Arc<Mutex<leptess::LepTess>>>>> {
    let mut apis_pool: Vec<Arc<Mutex<leptess::LepTess>>> = Vec::new();
    for _ in 0..num_threads {
        apis_pool.push(Arc::new(Mutex::new(ocr::get_api())));
    }
    return Arc::new(Mutex::new(apis_pool));
}

fn iterate_frames(video_path: &str) {
    let fi = VideoFramesIterator::new(video_path).unwrap();
    let (width, height) = ffmpeg_utils::get_video_dims(video_path);
    println!("{width}x{height}");
    for (i, frame) in fi.enumerate() {}
}

//slave: [638, 617, 610, 603, 631, 652, 645, 624]
//master: [638, 617, 610, 603, 631, 645, 624]
