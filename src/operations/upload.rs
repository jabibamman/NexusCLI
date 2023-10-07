use std::collections::HashMap;
use std::process::exit;
use crate::cli::parser::CliArgs;
use cli_clipboard;
use crate::DOMAIN;
use crate::PROXY;
use crate::utils::network_utils::execute_curl_request;

pub fn upload(args: CliArgs) {

    let repository:&str = args.repository.as_ref().unwrap();
    let directory = args.directory.as_ref().unwrap();
    let url = format!("{}/repository/{}/{}",
                      DOMAIN,
                      repository,
                      directory,
    );

    let mut curl_args = HashMap::new();
    let source_file = args.source.as_ref().unwrap();
    curl_args.insert("upload-file", source_file);

    println!("Voulez-vous éxecuter la commande POST curl avec l'url : {}", url);
    println!("O/N");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    if input.trim().to_uppercase() == "O".to_uppercase() {
        match execute_curl_request(&url, "POST", curl_args, PROXY) {
            Ok(_) => println!("Requête réussie"),
            Err(e) => println!("Erreur lors de la requête : {}", e),
        }
    } else {
        println!("Commande abordée, mais la commande a été copiée dans le presse-papier");
        let proxy = if let Some(proxy) = PROXY {
            format!(" --noproxy {}", proxy)
        } else {
            "".to_string()
        };

        let command = format!("curl{} -k --upload-file {} {}", proxy, source_file, url);
        cli_clipboard::set_contents(command.to_owned()).unwrap();
    }

    exit(0);
}