use dialoguer::theme::ColorfulTheme;
use dialoguer::FuzzySelect;
use std::fs;
use std::io::{stdin, stdout, Write};
use std::process::{Command, ExitStatus};

const BANNER: &str = r#"
    ╔╗   ╔╗
    ║║  ╔╝╚╗
╔══╗║╚═╗╚╗╔╝     ╔═╗╔══╗
║╔═╝║╔╗║ ║║ ╔═══╗║╔╝║══╣
║╚═╗║║║║ ║╚╗╚═══╝║║ ╠══║
╚══╝╚╝╚╝ ╚═╝     ╚╝ ╚══╝
"#;

fn main() {
    let mut query = String::new();
    let languagepath = "./src/languages.txt";
    let utilpath = "./src/utils.txt";

    println!("{}", BANNER);

    let languagedata = fs::read_to_string(languagepath).expect("Couldn't read languages");
    let utildata = fs::read_to_string(utilpath).expect("Couldn't read utils");

    let mut languages: Vec<String> = Vec::new();
    let mut utils: Vec<String> = Vec::new();

    for language in languagedata.split('\n') {
        languages.push(language.to_string());
    }
    for util in utildata.split('\n') {
        utils.push(util.to_string());
    }

    let mut combined_vec: Vec<String> = languages.clone();
    combined_vec.extend(utils.into_iter());

    println!("Please select a language: ");
    let choice = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a language/utility")
        .default(0)
        .items(&combined_vec[..])
        .interact()
        .unwrap();

    if combined_vec.get(choice).is_none() {
        // if combined_vec.get(choice) {
        println!("Please select a Language or a Utility");
        return;
    }
    print!("Please enter a query for {}: ", &combined_vec[choice]);
    stdout().flush().unwrap();
    stdin()
        .read_line(&mut query)
        .expect("Error reading query from user");

    let url = if languages.contains(&combined_vec[choice]) {
        query = query.replace(" ", "+").trim().parse().unwrap();
        format!("https://cht.sh/{}/{}", choice, query)
    } else {
        format!("https://cht.sh/{}~{}", choice, query)
    };

    // let _ = Command::new("xdg-open").arg(url).spawn();
    let mut child = Command::new("curl")
        .arg(url)
        .spawn()
        .expect("failed to start curl");
    let _status: ExitStatus = child.wait().unwrap();
}
