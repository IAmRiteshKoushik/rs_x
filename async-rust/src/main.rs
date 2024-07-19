use error_chain::error_chain;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // Using async await
    let res = reqwest::get("http://httpbin.org/get").await?;
    println!("Status:{}", res.status());
    println!("Headers:\n{:#?}", res.status());

    let body = res.text().await?;
    println!("Body:\n{}", body);
    Ok(())
}
