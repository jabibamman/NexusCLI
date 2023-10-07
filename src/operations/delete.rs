use std::process::exit;
use crate::cli::parser::CliArgs;
use cli_clipboard;
use crate::DOMAIN;
use crate::PROXY;
use crate::utils::network_utils::execute_curl_request;

pub fn delete(args: CliArgs) {
    let repository:&str = args.repository.as_ref().unwrap();
    let directory = args.directory.as_ref().unwrap();
    let url = format!("{}/repository/{}/{}",
                      DOMAIN,
                      repository,
                      directory,
    );

    println!("Voulez-vous éxecuter la commande DELETE curl avec l'url : {}", url);
    println!("O/N");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    if input.trim().to_uppercase() == "O".to_uppercase() {
        match execute_curl_request(&url, "DELETE", PROXY) {
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

        let command = format!("curl{} -k -X DELETE {}", proxy, url);
        cli_clipboard::set_contents(command.to_owned()).unwrap();
    }

    exit(0);
}