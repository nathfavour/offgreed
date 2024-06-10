use std::env;
use std::path::PathBuf;

fn main() {
    // Download and build eSpeak NG
    let espeak_dir = PathBuf::from(env::var("OUT_DIR").unwrap()).join("espeak-ng");
    let espeak_url = "https://github.com/espeak-ng/espeak-ng/releases/download/v2.0.23/espeak-ng-2.0.23-linux-64bit-generic.tar.gz";
    download_and_extract(espeak_url, &espeak_dir).unwrap();
    cc::Build::new()
        .files(espeak_dir.join("src").glob("*.c").unwrap())
        .compile("espeak-ng");

    // Download and build Vosk
    let vosk_dir = PathBuf::from(env::var("OUT_DIR").unwrap()).join("vosk");
    let vosk_url = "https://alphacephei.com/vosk/models/vosk-model-small-en-us-0.15.zip";
    download_and_extract(vosk_url, &vosk_dir).unwrap();
    cc::Build::new()
        .files(vosk_dir.join("src").glob("*.c").unwrap())
        .compile("vosk");
}

fn download_and_extract(url: &str, dir: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    // Download the file
    let resp = reqwest::blocking::get(url)?;
    let mut file = std::fs::File::create(dir.join("archive.zip"))?;
    // std::io::copy(&mut resp, &mut
    std::io::copy(&mut resp, &mut file)?;
}
