use std::{fmt, env, process::{Command, Stdio}, thread, time::Duration};
use serde::Deserialize;
use serde_json;

extern crate reqwest;
use reqwest::header;


#[derive(Deserialize)]
struct Response {
    message: Option<String>,
    ssh_url: Option<String>
}


impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "message: {:?}, url: {:?}", self.message, self.ssh_url)
    }
}


fn command(args: &[&str]) -> Result<(), std::io::Error> {
    let output = Command::new("git")
    .args(&*args)
    .stdout(Stdio::piped())
    .spawn()?;
    thread::sleep(Duration::from_millis(10));

    Ok(())
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let name = &args[1];

    let repo = create_repo(name);
    match repo {
        Ok(v)  => println!("message: {v:?}"),
        Err(e) => println!("error: {e:?}"),
    }
}


fn add_remote(url: &str) -> String {
    command(&["init"]);
    command(&["remote", "add", "origin", &url]);
    command(&["remote", "set-url", "origin", &url]);

    format!("Added remote repo: {}", &url)
}


fn create_repo(name: &str) -> Result<String, Box<dyn std::error::Error>> {
    let first_part = "{\"name\":\"";
    let second_part = "\"}";
    let repo_name = format!("{}{}{}", first_part, name, second_part);

    let mut headers = header::HeaderMap::new();
    headers.insert("Authorization", "token <TOKEN>".parse().unwrap());
    headers.insert("Content-Type", "application/x-www-form-urlencoded".parse().unwrap());

    let res = reqwest::Client::new()
        .post("https://api.github.com/user/repos")
        .headers(headers)
        .body(repo_name)
        .send()?
        .text()?;
    let json: Response = serde_json::from_str(&res)?;

    match json.ssh_url {
        Some(url) => Ok(add_remote(&url)),
        None      => Ok(json.message.unwrap()),
    }
}
