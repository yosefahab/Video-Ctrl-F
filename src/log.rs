use crate::ExitCode;
pub fn log(error: ExitCode) {
    match error {
        ExitCode::Success => (),
        ExitCode::InvalidArgs => eprintln!("Usage: vcf [video_path]"),
        ExitCode::InvalidPath => {
            eprintln!("Invalid File Path, Make sure the path you provided is correct")
        }
        ExitCode::KeyframesError(info) => {
            eprintln!("Failed to extract keyframes, reason: \n{}", info)
        }
        ExitCode::WavConversionError(info) => {
            eprintln!("Failed to convert to WAV, reason: \n{}", info)
        }
        ExitCode::FFProbeError(info) => eprintln!("FFprobe error, reason: \n{}", info),
    }
}
