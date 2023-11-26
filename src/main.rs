use std::collections::HashMap;

use serde_derive::Deserialize;
use serde_json::Value;
use reqwest::{Client, ClientBuilder, Response};
use tokio::runtime::Runtime;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Review {
    id: u8,
    comment: String,
    rating: u32,
    user_id: String,
    item_id: u8,
}

async fn get_from_url(client: &Client, url: &str) -> Result<Response, reqwest::Error> {
    let request = client
        .get(url)
        .build()
        .expect("There was an error building the request.");
    client.execute(request)
        .await

}

fn get_unique_users<'a>(reviews: &'a Vec<Review>) -> Vec<&'a str> {
    let mut ids: HashMap<&'a str, _> = HashMap::new();
    reviews.iter().for_each(|review| {
        ids.entry(&review.user_id).or_insert(0);
    });

    ids.iter().map(|user_id| *user_id.0).collect()
}

async fn get_user_info(client: &Client, url: &str, param: &str) -> Result<String, reqwest::Error> {
    let url = format!("{url}{param}");
    let request = client.get(&url).build()?;
    let response = client.execute(request).await?;
    let text = response.text().await?;
    Ok(text)    
}


fn main() {
    let review_url = "https://naturalflowersas.azurewebsites.net/review";
    let profile_url = "https://naturalflowersas.azurewebsites.net/profile/";
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        let client = ClientBuilder::new();
        let client = client.build().expect("There was an error constructing the client.");
        let json = get_from_url(&client, review_url)
            .await
            .expect("There was an error processing the request.")
            .text()
            .await
            .expect("There was an error converting the response to text.");

        let reviews: Vec<Review> = serde_json::from_str(&json).expect("Error deserializing data");

        let ids = get_unique_users(&reviews);

        for (i, userid) in ids.iter().enumerate() {
            let user_info = get_user_info(&client, profile_url, userid).await;
            if let Ok(info_str) = user_info {
                println!("User info: {}", info_str);
                continue;
            }
            println!("Error handling user info at vec location: {i}");
        }

    })
}
