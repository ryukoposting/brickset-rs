use brickset::v3::Response;
use brickset::v3::response::{CheckUserHashResponse, LegoComDetails, LoginResponse, GetSetsResponse};
use brickset::v3::request::{OrderBy, CheckUserHash, BricksetRequest, ENDPOINT, Login, GetSets, GetSetsParameters};
use dotenv;
use reqwest::Client;
use std::{env, fs::File, io::Write};

#[tokio::main]
async fn main() {
    if dotenv::from_filename(".env.examples").is_err() {
        println!("Did not find .env.examples! If BRICKSET_KEY and BRICKSET_USERNAME aren't set in your environment, the example will crash.");
    }

    // load stuff from the environment
    let api_key = env::var("BRICKSET_KEY")
        .expect("Missing environment variable BRICKSET_KEY");
    let username = env::var("BRICKSET_USERNAME")
        .expect("Missing environment variable BRICKSET_USERNAME");

    let mut client = reqwest::Client::default();

    // log into BrickSet
    let user_hash = log_into_brickset(&mut client, &api_key, &username).await;

    // build the getSets request URI and body
    let params = GetSetsParameters::new()
        .wanted_by_user(true)
        .order_by(OrderBy::PiecesDESC)
        .page_size(500);
    let builder = GetSets::new(&api_key, Some(&user_hash), params);
    let dest = ENDPOINT.join(builder.method_name()).expect("encoding url");
    let mut body = url::form_urlencoded::Serializer::new(String::new());
    builder.encode_query(&mut body).expect("encoding body");
    let body = body.finish();

    // create a reqwest::Request
    let request = client.post(dest)
        .header(reqwest::header::CONTENT_TYPE, "application/x-www-form-urlencoded")
        .header(reqwest::header::CONTENT_LENGTH, body.as_bytes().len())
        .body(body)
        .build()
        .expect("building request");

    // execute the request
    let response = client.execute(request)
        .await
        .expect("executing request")
        .text()
        .await
        .expect("getting response body");

    // process the response
    let sets: GetSetsResponse = match serde_json::from_str(&response).expect("from_str") {
        Response::Ok(sets) => sets,
        Response::Err(err) => {
            panic!("BrickSet error on getSets: {err}")
        },
    };

    // print results
    println!("User has {} sets in wantlist", sets.matches);

    let unknown = "Unknown".to_string();

    for set in sets.sets.iter() {
        println!("{}-{} {}", set.number, set.number_variant, set.name.as_ref().unwrap_or(&unknown));
        println!("  Theme: {} (Subtheme: {})", set.theme.as_ref().unwrap_or(&unknown), set.subtheme.as_ref().unwrap_or(&unknown));
        println!("  Pieces: {}", set.pieces.map_or_else(|| unknown.clone(), |p| p.to_string()));
        println!("  Availability: {}", set.availability.as_ref().unwrap_or(&unknown));
        println!("  Pricing on LEGO.com:");
        print_pricing("USD", &set.lego_com.united_states);
        print_pricing("CAD", &set.lego_com.canada);
        print_pricing("EUR", &set.lego_com.germany);
        print_pricing("GBP", &set.lego_com.united_kingdom);
    }
}

fn print_pricing(tag: &str, details: &LegoComDetails) {
    if let Some(price) = details.retail_price {
        print!("  - {price:.2} {tag}");
        if let Some(last_available) = details.date_last_available {
            print!(" Last available: {}", last_available.format("%Y-%m-%d"));
        }
        println!("");
    }
}

async fn log_into_brickset(client: &mut Client, api_key: &str, username: &str) -> String {
    println!("Logging in...");

    // try to log in using cached token
    if dotenv::from_filename(".env.examples.generated").is_ok() {
        if let Some(user_hash) = env::var("BRICKSET_USER_HASH").ok() {
            // build the checkUserHash request URI and body
            let builder = CheckUserHash::new(api_key, &user_hash);
            let dest = ENDPOINT.join(builder.method_name()).expect("encoding url");
            let mut body = url::form_urlencoded::Serializer::new(String::new());
            builder.encode_query(&mut body).expect("encoding body");
            let body = body.finish();

            // create a reqwest::Request
            let request = client.post(dest)
                .header(reqwest::header::CONTENT_TYPE, "application/x-www-form-urlencoded")
                .header(reqwest::header::CONTENT_LENGTH, body.as_bytes().len())
                .body(body)
                .build()
                .expect("building request");

            // execute the request
            let response = client.execute(request)
                .await
                .expect("executing request")
                .text()
                .await
                .expect("getting response body");

            // process the response
            match serde_json::from_str(&response).expect("from_str") {
                Response::Ok(CheckUserHashResponse {}) => {
                    println!("Logging in using cached token");
                    return user_hash;
                }
                Response::Err(err) => {
                    println!("Could not log in with cached token: {err}")
                },
            }
        }
    }

    // couldn't log in using cached token, so ask for a password
    println!("Username: {username}");
    let password = rpassword::prompt_password("Password: ").unwrap();

    // build the login request URI and body
    let builder = Login::new(api_key, username, &password);
    let dest = ENDPOINT.join(builder.method_name()).expect("encoding url");
    let mut body = url::form_urlencoded::Serializer::new(String::new());
    builder.encode_query(&mut body).expect("encoding body");
    let body = body.finish();

    // create a reqwest::Request
    let request = client.post(dest)
        .header(reqwest::header::CONTENT_TYPE, "application/x-www-form-urlencoded")
        .header(reqwest::header::CONTENT_LENGTH, body.as_bytes().len())
        .body(body)
        .build()
        .expect("building request");

    // execute the request
    let response = client.execute(request)
        .await
        .expect("executing request")
        .text()
        .await
        .expect("getting response body");

    // process the response
    match serde_json::from_str(&response).expect("from_str") {
        Response::Ok(LoginResponse { hash }) => {
            println!("Successfully logged in");

            // successfully logged in, save the new token
            let fp = File::options()
                .create(true)
                .write(true)
                .truncate(true)
                .open(".env.examples.generated");

            if let Ok(mut fp) = fp {
                fp.write_fmt(format_args!("BRICKSET_USER_HASH={:?}", hash))
                    .expect("write_fmt");
            }

            return hash;
        }
        Response::Err(err) => {
            println!("Could not log in: {err}");
            panic!("Could not log in: {err}")
        }
    }
}

