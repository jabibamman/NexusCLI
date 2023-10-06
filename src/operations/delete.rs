use crate::cli::parser::CliArgs;
use crate::DOMAIN;
use crate::PROXY;
use cli_clipboard;
use curl::easy::Easy;
use std::process::exit;

pub fn delete(args: CliArgs) {
    let repository: &str = args.repository.as_ref().unwrap();
    let directory = args.directory.as_ref().unwrap();
    let url = format!("{}/repository/{}/{}", DOMAIN, repository, directory,);

    println!("Voulez vous éxecuter la commande curl avec l'url : {}", url);
    println!("O/N");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    if input.trim().to_uppercase() == "O".to_uppercase() {
        let mut easy = Easy::new();
        easy.url(&url).unwrap();
        easy.custom_request("DELETE").unwrap();

        if let Some(proxy) = PROXY {
            easy.proxy(&proxy).unwrap();
        }

        match easy.perform() {
            Ok(_) => println!("Requête réussie"),
            Err(e) => println!("Erreur lors de la requête : {:?}", e.description()),
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
