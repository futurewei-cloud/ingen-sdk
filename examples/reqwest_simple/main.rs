use std::net::IpAddr;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), reqwest::Error> {
    // Some simple CLI args requirements...
    let url = match std::env::args().nth(1) {
        Some(url) => url,
        None => {
            println!("No CLI URL provided, using default.");
            "http://127.0.0.1:1337".into()
        }
    };

    eprintln!("Fetching {:?}...", url);

    // reqwest::get() is a convenience function.
    //
    // In most cases, you should create/build a reqwest::Client and reuse
    // it for all requests.
    let local_addr = IpAddr::from([127, 0, 0, 1]);
    let client = reqwest::Client::builder()
        .local_address(local_addr)
        .build()
        .unwrap();

    let res = client.get(url).send().await?;

    eprintln!("Response: {:?} {}", res.version(), res.status());
    eprintln!("Headers: {:#?}\n", res.headers());

    let body = res.text().await?;

    println!("{}", body);

    Ok(())
}
