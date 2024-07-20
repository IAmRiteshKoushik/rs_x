use reqwest::blocking::Client;
use reqwest::Error;

fn main() -> Result<(), Error> {
    let client = Client::new();

    let username = "testuser".to_string();
    // Option<String> means that it can either be a string or a None type
    let passwd: Option<String> = None;

    let response = client
        .get("http://httpbin.org/get")
        .basic_auth(username, passwd)
        .send();

    println!("{:#?}", response);

    // Result is either an empty tuple of an Error, here; there is no code to
    // send back an error so only an empty tuple is being sent back
    Ok(())
}
