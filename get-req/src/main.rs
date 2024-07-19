use error_chain::error_chain;
use std::io::Read;

// What is error-chain ? What does it do ?
error_chain! (
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
);

fn main() -> Result<()> {
    // Here, we have used blocking requests and not async/await. So unless one
    // call gets resolved, another cannot be made .. which means that the
    // server would not response
    let mut res = reqwest::blocking::get("http://httpbin.org/get")?;
    let mut body = String::new();
    res.read_to_string(&mut body)?;

    println!("Status: {}", res.status());
    println!("Headers: \n{:#?}", res.headers());
    println!("Body: \n{}", body);

    Ok(())
}
