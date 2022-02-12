pub mod user;
use std::collections::HashMap;

use anyhow::Result;
use embedded_svc::io::Bytes;
use esp_idf_svc::http::client::EspHttpClient;
use embedded_svc::http::{client::*};

use crate::profile::Profile;

use self::user::User;

pub fn init(client: &mut EspHttpClient) -> Result<Profile> {
    let url = String::from("https://api.github.com/users/buhe/followers");

    // info!("About to fetch content from {}", url);

    

    let response = client.get(&url)?.submit()?;
    let body: Result<Vec<u8>, _> = Bytes::<_, 64>::new(response.reader()).collect();

    let body = body?;
    let str = String::from_utf8_lossy(&body).into_owned();
    // println!(
    //     "Body \n{:?}",
    //     &str
    // );

    let users: Vec<User> = serde_json::from_str(&str).unwrap();
    // println!("Hello, world!bugu22: {:?}", users.len());
    Ok(Profile{followers: users.len(),
        followings: 0,
        starts: 0,
        display: "buhe".to_string(),
        avatar: "".to_string(), 
        vender: "github".to_string(),
        optional: HashMap::new()})
}