use futures::stream::{self, StreamExt};

use graph_rs_sdk::http::GraphResponse;
use graph_rs_sdk::client::Graph;

use serde::Deserialize;
use serde::Serialize;
use reqwest::Client;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub(crate) id: Option<String>,
    #[serde(rename = "userPrincipalName")]
    user_principal_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Users {
    pub(crate) value: Vec<User>,
    #[serde(rename = "@odata.nextLink")]
    pub(crate) next_link: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LicenseDetail {
    id: Option<String>,
    #[serde(rename = "skuId")]
    sku_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LicenseDetails {
    pub(crate) value: Vec<LicenseDetail>,
}

static ACCESS_TOKEN: &str = "";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client = Graph::new_async(ACCESS_TOKEN);

    let users_resp: GraphResponse<Users> = client
        .v1()
        .users()
        .list_user()
        .select(&["id", "userPrincipalName"])
        .top("999")
        .json()
        .await.unwrap();

    let users = users_resp.into_body().value;
    println!("Found {} users.", users.len());

    let reqwest_client = reqwest::Client::new();
    let mut stream = stream::iter(&users)
        .map(|user| async {
            match get_user_license_details_by_reqwest(&reqwest_client, ACCESS_TOKEN, user.id.as_ref().unwrap()).await
            {
                Ok(license_details) => {
                    println!("Got {} license details", license_details.len());
                    Ok(license_details)
                }
                Err(e) => Err(e),
            }
        }).buffer_unordered(100);

    let mut license_details = vec![];
    while let Some(result) = stream.next().await {
        match result {
            Ok(mut license_detail) => {
                license_details.append(&mut license_detail);
            }
            Err(e) => println!("Error when getting license_details : {}", e),
        }
    }

    let mut stream = stream::iter(&users)
        .map(|user| async {
            match get_user_license_details(&client, user.id.as_ref().unwrap()).await
            {
                Ok(license_details) => {
                    println!("Got {} license details", license_details.len());
                    Ok(license_details)
                }
                Err(e) => Err(e),
            }
        }).buffer_unordered(100);

    let mut license_details = vec![];
    while let Some(result) = stream.next().await {
        match result {
            Ok(mut license_detail) => {
                license_details.append(&mut license_detail);
            }
            Err(e) => println!("Error when getting license_details : {}", e),
        }
    }
    Ok(())
}

pub async fn get_user_license_details(
    client: &Graph<graph_rs_sdk::http::AsyncHttpClient>,
    user_id: &str,
) -> Result<Vec<LicenseDetail>, Box<dyn std::error::Error + Send + Sync>> {
    println!("Fetching license detail for {}", user_id);

    let _a = client
        .v1();
    //.users()
    //.id(user_id);
    //  let b = a
    //.list_license_details();

    //  let license_details: GraphResponse<LicenseDetails> = b.json().await?;

    Ok(vec![])
}

pub async fn get_user_license_details_by_reqwest(
    client: &Client,
    access_token: &str,
    user_id: &str,
) -> Result<Vec<LicenseDetail>, Box<dyn std::error::Error + Send + Sync>> {
    let license_details: LicenseDetails = client
        .get(format!("https://graph.microsoft.com/v1.0/users/{user_id}/licenseDetails"))
        .timeout(core::time::Duration::from_secs(20))
        .bearer_auth(access_token)
        .send()
        .await?
        .json()
        .await?;

    Ok(license_details.value)
}