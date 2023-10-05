use crate::cli::parser::CliArgs;
use curl::easy::{Easy, Easy2, Handler, List, WriteError};
use cli_clipboard;
/*
curl --noproxy nexus.insee.fr -k -X DELETE
 https://nexus.insee.fr/repository/depot-local/rp/omer/ihm/homere-gestion-DV05.zip
*/
pub fn delete(args: CliArgs) {
    let url:&str = "https://nexus.insee.fr";
    let repository:&str = "depot-local";
    let directory = "rp/omer/ihm";
    let artifact_id:&str = "homere-gestion-DV05";
    let artifact_extension:&str = "zip";
    let url = format!("{}/repository/{}/{}/{}/{}",
        url,
        repository,
        directory,
        artifact_id,
        artifact_extension
    );

    println!("Voulez vous éxecuter la commande suivante : {}", url);
    println!("O/N");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    if input.trim().to_uppercase() == "O".to_uppercase() {
        let mut easy = Easy::new();
        easy.url(&url).unwrap();
        easy.custom_request("DELETE").unwrap();
        easy.perform().unwrap();
    } else {
        let command = format!("curl --noproxy nexus.insee.fr -k -X DELETE {}", url);
        cli_clipboard::set_contents(command.to_owned()).unwrap();
        panic!("Commande abordée, mais la commande a été copiée dans le presse-papier");
   }

}