pub mod ffmpeg_utils {
    use std::path::{Path, PathBuf};
    use std::process::{Command, Stdio};

    #[derive(Debug)]
    pub struct Metadata {
        pub width: u64,
        pub height: u64,
        pub duration: f64,
        pub num_frames: u64,
        pub fps: u64,
    }
    #[derive(Debug)]
    pub enum FFmpegResult {
        Success(String),
        Failure(String),
    }
    #[derive(Debug)]
    pub enum FFprobeResult {
        Success(Metadata),
        Failure(String),
    }

    /// uses ffmpeg to extract keyframes from a video
    pub fn extract_keyframes(video_path: &Path, output_path: &Path) -> FFmpegResult {
        let output_path = PathBuf::from(output_path).join("%06d.jpg");
        let output_path = output_path.to_str().unwrap();
        let video_path = video_path.to_str().unwrap();
        let output = Command::new("ffmpeg")
            .args([
                "-skip_frame",
                "nokey",
                "-i",
                video_path,
                "-fps_mode",
                "passthrough",
                "-f",
                "image2",
                "-frame_pts",
                "true",
                "-qscale:v",
                "2",
                "-qmin",
                "1",
                output_path,
            ])
            .output();
        match output {
            Ok(output) => match output.status.success() {
                true => {
                    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
                    return FFmpegResult::Success(stdout);
                }
                false => {
                    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
                    return FFmpegResult::Failure(stderr);
                }
            },
            Err(e) => return FFmpegResult::Failure(e.to_string()),
        };
    }
    /// converts a video a .wav file
    pub fn conv2wav(video_path: &Path, output_path: &Path) -> FFmpegResult {
        let video_path = video_path.to_str().unwrap();
        let output_path = PathBuf::from(output_path).join("audio.wav");
        let output_path = output_path.to_str().unwrap();
        let output = Command::new("ffmpeg")
            .args([
                "-i",
                video_path,
                "-ar",
                "16000",
                "-ac",
                "1",
                "-c:a",
                "pcm_s16le",
                output_path,
            ])
            .output();
        match output {
            Ok(output) => match output.status.success() {
                true => {
                    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
                    return FFmpegResult::Success(stdout);
                }
                false => {
                    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
                    return FFmpegResult::Success(stderr);
                }
            },
            Err(e) => return FFmpegResult::Failure(e.to_string()),
        };
    }

    /// returns (width, height) for a given video
    pub fn get_video_dims(video_path: &Path) -> Result<(u64, u64), FFprobeResult> {
        match get_video_metadata(video_path) {
            FFprobeResult::Success(metadata) => {
                return Ok((metadata.width, metadata.height));
            }
            FFprobeResult::Failure(e) => {
                return Err(FFprobeResult::Failure(e));
            }
        }
    }

    pub fn get_video_metadata(video_path: &Path) -> FFprobeResult {
        // Execute the ffprobe command as a subprocess
        let video_path = video_path.to_str().unwrap();
        let output = Command::new("ffprobe")
            .args([
                "-v",
                "error",
                "-select_streams",
                "v:0",
                "-show_entries",
                "stream=width,height,duration,r_frame_rate,nb_frames",
                "-of",
                "json",
                video_path,
            ])
            .stdout(Stdio::piped())
            .output();
        match output {
            Ok(output) => {
                let metadata_json = String::from_utf8(output.stdout).unwrap();
                let metadata: serde_json::Value = serde_json::from_str(&metadata_json).unwrap();
                let video_stream = &metadata["streams"][0];
                let width = video_stream["width"].as_u64().unwrap();
                let height = video_stream["height"].as_u64().unwrap();
                let duration =
                    parse_string::<f64>(video_stream["duration"].as_str().unwrap()).unwrap();
                let num_frames =
                    parse_string::<u64>(video_stream["nb_frames"].as_str().unwrap()).unwrap();
                let fps = parse_frame_rate(video_stream["r_frame_rate"].as_str().unwrap());
                return FFprobeResult::Success(Metadata {
                    width,
                    height,
                    duration,
                    num_frames,
                    fps,
                });
            }
            Err(e) => {
                return FFprobeResult::Failure(e.to_string());
            }
        }
    }
    fn parse_string<T: std::str::FromStr>(s: &str) -> Result<T, <T as std::str::FromStr>::Err> {
        s.parse::<T>()
    }
    fn parse_frame_rate(fps: &str) -> u64 {
        let parts: Vec<&str> = fps.split('/').collect();
        let num = parse_string::<f64>(parts[0]).unwrap();
        let den = parse_string::<f64>(parts[1]).unwrap();
        return (num / den) as u64;
    }
}

// !WIP
pub mod frames_iterator {
    use std::io::Read;
    use std::path::Path;
    use std::process::{Command, Stdio};

    pub struct VideoFramesIterator {
        ffmpeg_command: Option<std::process::Child>,
        buffer: [u8; 1024],
    }
    impl VideoFramesIterator {
        pub fn new(video_path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
            let video_path = video_path.to_str().unwrap();
            let ffmpeg_command = Command::new("ffmpeg")
                .args([
                    "-i",
                    video_path,
                    "-f",
                    "image2pipe",
                    "-pix_fmt",
                    "rgb8",
                    "-vcodec",
                    "rawvideo",
                    "-",
                ])
                .stdout(Stdio::piped())
                .stderr(Stdio::null())
                .spawn()?;

            Ok(Self {
                ffmpeg_command: Some(ffmpeg_command),
                buffer: [0u8; 1024],
            })
        }
    }
    impl Iterator for VideoFramesIterator {
        type Item = Vec<u8>;

        fn next(&mut self) -> Option<Self::Item> {
            let mut frame_data = Vec::new();

            if let Some(ref mut ffmpeg) = self.ffmpeg_command {
                let size = match ffmpeg.stdout.as_mut().unwrap().read(&mut self.buffer) {
                    Ok(size) if size > 0 => size,
                    _ => {
                        // The child process has finished or there was an error
                        self.ffmpeg_command = None;
                        return None;
                    }
                };

                frame_data.extend_from_slice(&self.buffer[..size]);
            }

            return Some(frame_data);
        }
    }
}
