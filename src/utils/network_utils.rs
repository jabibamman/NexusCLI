use curl::easy::{Easy, ReadError};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::sync::{Arc, Mutex};

pub fn execute_curl_request(
    url: &str,
    method: &str,
    args: HashMap<&str, &String>,
    proxy: Option<&str>,
) -> Result<(), curl::Error> {
    let mut easy = Easy::new();
    easy.url(url)?;
    easy.custom_request(method)?;

    if let Some(proxy) = proxy {
        easy.proxy(proxy)?;
    }

    if let Some(filepath) = args.get("upload-file") {
        easy.upload(true)?;
        let mut file = File::open(filepath).expect("Le fichier n'a pas pu être ouvert");
        let file_contents = Arc::new(Mutex::new(Vec::new()));
        file.read_to_end(&mut *file_contents.lock().unwrap())
            .expect("Échec de la lecture du fichier");

        let shared_contents = file_contents.clone();

        easy.read_function(move |buf| -> Result<usize, ReadError> {
            let locked = shared_contents.lock().unwrap();
            let mut slice = &(*locked)[..];
            match slice.read(buf) {
                Ok(size) => Ok(size),
                Err(_) => Err(ReadError::Abort),
            }
        })?;
    }

    match easy.perform() {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}
