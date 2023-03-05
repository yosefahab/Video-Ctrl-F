pub mod ffmpeg_utils {
    use regex::Regex;
    use std::io::Result;
    use std::path::Path;
    use std::process::{Command, Output};
    pub struct Metadata {
        pub width: u32,
        pub height: u32,
        duration: f32,
        pub fps: u32,
        num_frames: u32,
    }
    pub fn extract_keyframes(video_path: &str, output_path: &str) -> Result<Output> {
        let output_path = Path::new(output_path).join("frame-%d.jpg");
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
                output_path.to_str().unwrap(),
            ])
            .output()?;

        println!("status: {}", output.status);
        // println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
        // println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
        return Ok(output);
    }
    pub fn conv2wav(video_path: &str, output_path: &str) -> Result<Output> {
        let output_path = Path::new(output_path).join("audio.wav");
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
                output_path.to_str().unwrap(),
            ])
            .output()?;

        println!("status: {}", output.status);
        // println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
        // println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
        return Ok(output);
    }
    pub fn get_video_dims(video_path: &str) -> (u32, u32) {
        let metadata: Metadata =
            self::get_video_metadata(video_path).expect("Failed to get video metadata");
        let width: u32 = metadata.width;
        let height: u32 = metadata.height;
        return (width, height);
    }
    pub fn get_video_metadata(video_path: &str) -> Result<Metadata> {
        let output = Command::new("ffprobe")
            .args(["-v", "quiet", "-show_format", "-show_streams", video_path])
            .output()?;

        let output_str = String::from_utf8(output.stdout).expect("Failed to decode ffprobe output");

        let re = Regex::new(r"width=(\d+)").unwrap();
        let width_match = re.captures(&output_str).unwrap();
        let re = Regex::new(r"height=(\d+)").unwrap();
        let height_match = re.captures(&output_str).unwrap();
        let re = Regex::new(r"duration=([\d\.]+)").unwrap();
        let duration_match = re.captures(&output_str).unwrap();
        let re = Regex::new(r"r_frame_rate=(\d+)/(\d+)").unwrap();
        let fps_match = re.captures(&output_str).unwrap();
        let re = Regex::new(r"nb_frames=(\d+)").unwrap();
        let num_frames_match = re.captures(&output_str).unwrap();

        let width: u32 = width_match.get(1).unwrap().as_str().parse::<u32>().unwrap();
        let height: u32 = height_match
            .get(1)
            .unwrap()
            .as_str()
            .parse::<u32>()
            .unwrap();
        let duration: f32 = duration_match
            .get(1)
            .unwrap()
            .as_str()
            .parse::<f32>()
            .unwrap();
        let fps: u32 = fps_match.get(1).unwrap().as_str().parse::<u32>().unwrap()
            / fps_match.get(2).unwrap().as_str().parse::<u32>().unwrap();

        let num_frames: u32 = num_frames_match
            .get(1)
            .unwrap()
            .as_str()
            .parse::<u32>()
            .unwrap();

        let metadata = Metadata {
            width,
            height,
            duration,
            fps,
            num_frames,
        };
        return Ok(metadata);
    }
}

pub mod frames_iterator {
    use std::io::Read;
    use std::process::{Command, Stdio};

    pub struct VideoFramesIterator {
        ffmpeg_command: Option<std::process::Child>,
        buffer: [u8; 1024],
    }
    impl VideoFramesIterator {
        pub fn new(video_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
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
