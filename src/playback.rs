use std::fs::File;
use std::path::PathBuf;
use std::io::BufReader;
use rodio::Decoder;

pub fn get_song(filename: PathBuf) -> Decoder<BufReader<File>> {
    println!("Path: {:?}", filename);
    let file = File::open(filename).unwrap();
    return rodio::Decoder::new(BufReader::new(file)).unwrap();
}
