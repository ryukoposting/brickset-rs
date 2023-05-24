use brickset::v3::{request::OrderBy, reqwest_api::ClientWrapper, response::LegoComDetails};
use dotenv;
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

    let client = reqwest::Client::default();
    let mut client = ClientWrapper::new(&api_key, &client);

    // log into BrickSet
    log_into_brickset(&mut client, &username).await;

    // retrieve the user's wanted sets
    let sets = client
        .get_wanted_sets(OrderBy::PiecesDESC.into(), Some(500), None, false)
        .await
        .expect("get_wanted_sets");

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

async fn log_into_brickset<'a>(client: &mut ClientWrapper<'a>, username: &str) {
    println!("Logging in...");

    // try to log in using cached token
    if dotenv::from_filename(".env.examples.generated").is_ok() {
        if let Some(user_hash) = env::var("BRICKSET_USER_HASH").ok() {
            match client.reuse_login(&user_hash).await {
                Ok(_) => println!("Logged in using cached token"),
                Err(err) => println!("Could not log in with cached token: {err}"),
            }
        }
    }

    if !client.is_logged_in() {
        // couldn't log in using cached token, so ask for a password
        println!("Username: {username}");
        let password = rpassword::prompt_password("Password: ").unwrap();
        let login = client.log_in(&username, &password).await.expect("log_in");

        println!("Successfully logged in");

        // successfully logged in, save the new token
        let fp = File::options()
            .create(true)
            .write(true)
            .truncate(true)
            .open(".env.examples.generated");

        if let Ok(mut fp) = fp {
            fp.write_fmt(format_args!("BRICKSET_USER_HASH={:?}", login.hash))
                .expect("write_fmt");
        }
    }
}
