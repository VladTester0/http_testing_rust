// std - like package
// env - specific group
// var - function in env
// std::env like module
// std::env::var uses the Result type (it can succeed or fail to get a value)
fn main() {
    let api_token = std::env::var("API_TOKEN")
        .expect("EXPECTED THERE TO BE AN API TOKEN");
    dbg!(&api_token);
    // std::env::args returns the struct type std::env::Args
    let mut arg_iterator = std::env::args();
    arg_iterator.next();
    let args: String = arg_iterator.collect();
    dbg!(&args);

    let client = reqwest::blocking::Client::new();

    let response = client
        .get("https://api.waqi.info/search/")
        .query(&[("token", api_token), ("keyword", args)])
        .send()
        .expect("a successful request")
        .json::<serde_json::Value>()
        .expect("expected the body to be json");
    dbg!(response);
}

// enum Result<THING_WE_WANT, ERROR_THAT_COULD_HAPPEN> {
//     Ok(THING_WE_WANT),
//     Err(ERROR_THAT_COULD_HAPPEN)
// }



