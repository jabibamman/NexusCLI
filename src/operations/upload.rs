use crate::cli::parser::CliArgs;
use curl::easy::{Easy, ReadError};
use std::fs::File;
use std::io::Read;
use crate::DOMAIN;
use crate::PROXY;
use std::process::exit;

pub fn upload(args: CliArgs) {
    let repository: &str = args.repository.as_ref().unwrap();
    let directory = args.directory.as_ref().unwrap();
    let url = format!("{}/repository/{}/{}", DOMAIN, repository, directory);

    println!("Voulez-vous éxecuter la commande UPLOAD curl avec l'url : {}", url);
    println!("O/N");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    if input.trim().to_uppercase() == "O" {
        let source_file = args.source.expect("Le chemin du fichier source est nécessaire");

        let mut easy = Easy::new();
        easy.url(&url).expect("Erreur lors de la définition de l'URL");
        easy.upload(true).expect("Erreur lors de la mise en mode upload");
        if let Some(proxy) = PROXY {
            easy.proxy(proxy).expect("Erreur lors de la définition du proxy");
            easy.noproxy("insee.fr").expect("Erreur lors de la définition de noproxy");
        }

        let mut file = File::open(&source_file).expect("Le fichier n'a pas pu être ouvert");
        let mut file_contents = Vec::new();
        file.read_to_end(&mut file_contents).expect("Échec de la lecture du fichier");

        easy.read_function(move |buf| -> Result<usize, ReadError> {
            let mut slice = &file_contents[..];
            match slice.read(buf) {
                Ok(size) => Ok(size),
                Err(_) => Err(ReadError::Abort),
            }
        }).expect("Erreur lors de la définition de la fonction de lecture");
        match easy.perform() {
            Ok(_) => {
                if easy.response_code().expect("Erreur lors de la récupération du code de réponse") >= 300 {
                    panic!("Erreur lors de la requête : {}", easy.response_code().unwrap());
                }
                println!("Requête réussie");
            }

            Err(e) => println!("Erreur lors de la requête HTTP CODE ({}) ", e),
        }
    } else {
        println!("Commande abordée, mais la commande a été copiée dans le presse-papier");
        let proxy = if let Some(proxy) = PROXY {
            format!(" --noproxy {}", proxy)
        } else {
            "".to_string()
        };

        let command = format!("curl{} -k --upload-file {} {}", proxy, &args.source.unwrap(), url);
        cli_clipboard::set_contents(command).expect("Échec de la copie dans le presse-papier");
    }

    exit(0);
}
