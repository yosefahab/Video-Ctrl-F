# Video Content Search

The main idea is to search for text in videos (spoken or visible).
Using rust and ffmpeg, tesseract, and whisper.rs

## Pipeline
1. Create a **dump** directory for the processed video.
2. first, use ffmpeg to extract all keyframes for an .mp4 in the **dump/frames** directory.
3. For each frame,
    1. A timestamp is calculated based on the frame number.
    2. OCR (optical character recognition) is applied
    3. The indexer is updated with the predicted words and their corresponding timestamp. 
4. Apply ASR (automatic speech recognition) using **whisper.cpp**
    1. For each (word,timestamp) in the predicted string, the indexer is updated.

## Indexer Data Structure
for the indexer, two data structures were tested, a **Trie** and a **Hashmap**

## Models
Models directory should exist with the following structure:
```
|-- Models
|   |-- cpp_whisper
|   `-- traineddata
|       |-- trainneddata_base
|       |   `-- eng.traineddata
|       |-- trainneddata_fast
|       |   `-- eng.traineddata
|       `-- trainneddata_best
|           `-- eng.traineddata
```

## Whisper
Currently [whisper-rs](https://crates.io/crates/whisper-rs) is used to bind to **whisper.cpp**
#### **A compiled version of whisper.cpp is required!**
for more details, check [whisper.cpp](https://github.com/ggerganov/whisper.cpp)

## Limitations
1. Currently only .mp4 videos are supported
2. No sufficient Grammatical Error Correction crates available yet.
3. No reliable stop words removal crate
4. Currently supports English only

## Todos
- [ ] Fix incorrect timestamps
- [ ] solve indexer bottleneck issue
- [ ] implement image preprocessing
- [ ] implement key-frame filtering

