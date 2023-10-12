use curl::easy::{Easy};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use crate::utils::easy_utils::read_file;

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
        let mut file_contents = Vec::new();
        file.read_to_end(&mut file_contents)
            .expect("Échec de la lecture du fichier");
        read_file(filepath).expect("Erreur lors de la définition de la fonction de lecture");
    }

    easy.verbose(true)?;
    match easy.perform() {
        Ok(_) => {
            let mut response_code = 0;
            easy.response_code()
                .map(|code| response_code = code)
                .unwrap();
            if response_code >= 400 {
                println!("Erreur lors de la requête : {}", response_code);
            }
            println!("{}", response_code);

            Ok(())
        }
        Err(e) => Err(e),
    }
}
