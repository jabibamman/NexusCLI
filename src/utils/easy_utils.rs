use std::fs::File;
use std::io::Read;
use curl::easy::{Easy, ReadError};
use curl::Error;

pub fn read_file(file_path: &str) -> Result<(), Error> {
    let mut easy = Easy::new();
    let mut file = File::open(&file_path).expect("Le fichier n'a pas pu être ouvert");
    let mut file_contents = Vec::new();
    file.read_to_end(&mut file_contents).expect("Échec de la lecture du fichier");


    easy.read_function(move |buf| -> Result<usize, ReadError> {
        let mut slice = &file_contents[..];
        match slice.read(buf) {
            Ok(size) => Ok(size),
            Err(_) => Err(ReadError::Abort),
        }
    })
}