# Video Content Search

The main idea is to search for text in videos (spoken or visible).
Using rust and ffmpeg, tesseract, and whisper.rs

The pipeline works as follows:
1. first, use ffmpeg to extract all keyframes for an .mp4
2. Apply multithreaded ocr on the directory containing the video frames
3. For each frame, the ocr outputs a string containing all words that appeared in the image
3. The indexer is passed the predicted string and the timestamp for the frame
4. The indexer tokenizes the string into individual words and updates the internal **Data structure**
4. Apply ASR (automatic speech recognition) using whisper.cpp
5. Parse the predicted ASR text and use the indexer to update the **Data structure**

## Indexer Data Structure
for the indexer, two data structures were tested, a *Trie* and a *Hashmap*

## Whisper.cpp
for more details, check [whisper.cpp](https://github.com/ggerganov/whisper.cpp)

- Currently a work in progress