use curl::easy::Easy;
use dialoguer::theme::ColorfulTheme;
use dialoguer::FuzzySelect;
use std::io::{stdin, stdout, Write};
use std::process::Command;

const BANNER: &str = r#"
    ╔╗   ╔╗
    ║║  ╔╝╚╗
╔══╗║╚═╗╚╗╔╝     ╔═╗╔══╗
║╔═╝║╔╗║ ║║ ╔═══╗║╔╝║══╣
║╚═╗║║║║ ║╚╗╚═══╝║║ ╠══║
╚══╝╚╝╚╝ ╚═╝     ╚╝ ╚══╝
"#;

fn main() {
    let languages: Vec<String> = vec![
        String::from("arduino"),
        String::from("ArnoldC"),
        String::from("assembly/"),
        String::from("awk"),
        String::from("bash"),
        String::from("basic"),
        String::from("bf"),
        String::from("c"),
        String::from("chapel"),
        String::from("clean"),
        String::from("clojure"),
        String::from("coffee"),
        String::from("cpp"),
        String::from("csharp"),
        String::from("css"),
        String::from("d"),
        String::from("dart"),
        String::from("delphi"),
        String::from("dylan"),
        String::from("eiffel"),
        String::from("elisp"),
        String::from("elixir"),
        String::from("elm"),
        String::from("erlang"),
        String::from("factor"),
        String::from("forth"),
        String::from("fortran"),
        String::from("fsharp"),
        String::from("gdb"),
        String::from("go"),
        String::from("golang"),
        String::from("groovy"),
        String::from("haskell"),
        String::from("html"),
        String::from("java"),
        String::from("javascript"),
        String::from("js"),
        String::from("julia"),
        String::from("kotlin"),
        String::from("latex"),
        String::from("lisp"),
        String::from("lua"),
        String::from("matlab"),
        String::from("nim"),
        String::from("nodejs"),
        String::from("objective-c"),
        String::from("ocaml"),
        String::from("octave"),
        String::from("perl"),
        String::from("perl6"),
        String::from("php"),
        String::from("pike"),
        String::from("python"),
        String::from("python3"),
        String::from("r"),
        String::from("racket"),
        String::from("ruby"),
        String::from("rust"),
        String::from("scala"),
        String::from("scheme"),
        String::from("solidity"),
        String::from("swift"),
        String::from("tcl"),
        String::from("tcsh"),
        String::from("tmux"),
        String::from("typescript"),
        String::from("v"),
        String::from("vb"),
        String::from("vbnet"),
        String::from("vlang"),
        String::from("zsh"),
    ];

    let utils: Vec<String> = vec![
        String::from("find"),
        String::from("man"),
        String::from("tldr"),
        String::from("sed"),
        String::from("awk"),
        String::from("tr"),
        String::from("cp"),
        String::from("ls"),
        String::from("grep"),
        String::from("xargs"),
        String::from("rg"),
        String::from("ps"),
        String::from("mv"),
        String::from("kill"),
        String::from("lsof"),
        String::from("less"),
        String::from("head"),
        String::from("tail"),
        String::from("tar"),
        String::from("cp"),
        String::from("rm"),
        String::from("rename"),
        String::from("jq"),
        String::from("cat"),
        String::from("ssh"),
        String::from("cargo"),
        String::from("git"),
        String::from("git-worktree"),
        String::from("git-status"),
        String::from("git-commit"),
        String::from("git-rebase"),
        String::from("docker"),
        String::from("docker-compose"),
        String::from("stow"),
        String::from("chmod"),
        String::from("chown"),
        String::from("make"),
    ];

    let mut query = String::new();
    let mut easy = Easy::new();

    println!("{}", BANNER);
    let mut combined_vec: Vec<String> = languages.clone();
    combined_vec.extend(utils.into_iter());

    let selector: &str = "fzf";
    println!("Please select a language: ");
    let choice: usize;
    if selector.is_installed() {
        let fzf_choice = rust_fzf::fzf_select(combined_vec.clone());
        if let Some(index) = combined_vec.iter().position(|s| *s == fzf_choice) {
            choice = index;
        } else {
            return;
        }
    } else {
        choice = FuzzySelect::with_theme(&ColorfulTheme::default())
            .with_prompt("Select a language/utility")
            .default(0)
            .items(&combined_vec[..])
            .interact()
            .unwrap();
    }

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
        query = query.replace(' ', "+").trim().parse().unwrap();
        format!("https://cht.sh/{}/{}", choice, query)
    } else {
        format!("https://cht.sh/{}~{}", choice, query)
    };

    easy.url(&url).unwrap();
    easy.write_function(|data| {
        stdout().write_all(data).unwrap();
        Ok(data.len())
    })
    .unwrap();
    easy.useragent("curl").unwrap();
    easy.perform().unwrap();
}

trait Installed {
    fn is_installed(self) -> bool;
}

impl Installed for &str {
    // Checks if a given command is installed on the system or not.
    fn is_installed(self) -> bool {
        let output = Command::new("which")
            .arg(&self)
            .output()
            .expect("Failed to execute command");
        output.status.success()
    }
}
