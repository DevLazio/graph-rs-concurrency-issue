use futures::stream::{self, StreamExt};

use graph_http::GraphResponse;
use graph_rs_sdk::client::Graph;

use serde::Deserialize;
use serde::Serialize;

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

static ACCESS_TOKEN: &str = "eyJ0eXAiOiJKV1QiLCJub25jZSI6IkUxaUZHVzdDVGtEX2c5M2t4V0dzSXJjcDZjZVhLVklFTm11am0zeVAxdEUiLCJhbGciOiJSUzI1NiIsIng1dCI6Ik1yNS1BVWliZkJpaTdOZDFqQmViYXhib1hXMCIsImtpZCI6Ik1yNS1BVWliZkJpaTdOZDFqQmViYXhib1hXMCJ9.eyJhdWQiOiJodHRwczovL2dyYXBoLm1pY3Jvc29mdC5jb20iLCJpc3MiOiJodHRwczovL3N0cy53aW5kb3dzLm5ldC8zOWQ4MTVkYi0xZmY2LTQyMmQtYTcxOC05YTA1YWE1MDlkMzgvIiwiaWF0IjoxNjQxOTk3MTA0LCJuYmYiOjE2NDE5OTcxMDQsImV4cCI6MTY0MjAwMTAwNCwiYWlvIjoiRTJaZ1lOak44OVhvTHNOdmI0YjVUK2JuZEYwNUFBQT0iLCJhcHBfZGlzcGxheW5hbWUiOiJKYXNvbiIsImFwcGlkIjoiZmNjMjVlZTQtMDFhMC00OWRhLTkzZmItNmJlYzRmM2Y0NTBiIiwiYXBwaWRhY3IiOiIxIiwiaWRwIjoiaHR0cHM6Ly9zdHMud2luZG93cy5uZXQvMzlkODE1ZGItMWZmNi00MjJkLWE3MTgtOWEwNWFhNTA5ZDM4LyIsImlkdHlwIjoiYXBwIiwib2lkIjoiMzQ0ZDI5MTEtNzdhMi00OWMyLWFiNGItMzM5NGJhNjg2Y2U1IiwicmgiOiIwLkFUQUEyeFhZT2ZZZkxVS25HSm9GcWxDZE9PUmV3dnlnQWRwSmtfdHI3RThfUlFzd0FBQS4iLCJyb2xlcyI6WyJQcmludGVyLlJlYWQuQWxsIiwiUHJpbnRlci5SZWFkV3JpdGUuQWxsIiwiTWFpbGJveFNldHRpbmdzLlJlYWQiLCJEaXJlY3RvcnkuUmVhZC5BbGwiLCJVc2VyLlJlYWQuQWxsIiwiT3JnYW5pemF0aW9uLlJlYWQuQWxsIiwiQXVkaXRMb2cuUmVhZC5BbGwiLCJTZXJ2aWNlSGVhbHRoLlJlYWQuQWxsIiwiUmVwb3J0cy5SZWFkLkFsbCJdLCJzdWIiOiIzNDRkMjkxMS03N2EyLTQ5YzItYWI0Yi0zMzk0YmE2ODZjZTUiLCJ0ZW5hbnRfcmVnaW9uX3Njb3BlIjoiRVUiLCJ0aWQiOiIzOWQ4MTVkYi0xZmY2LTQyMmQtYTcxOC05YTA1YWE1MDlkMzgiLCJ1dGkiOiJRSWZnU29QdnJVU3RRUkRJNktSWEFRIiwidmVyIjoiMS4wIiwid2lkcyI6WyIwOTk3YTFkMC0wZDFkLTRhY2ItYjQwOC1kNWNhNzMxMjFlOTAiXSwieG1zX3RjZHQiOjE1MDUyMDE2NDR9.FdHBEAzb2hJeiYZXPtx2Sn0YLgvv0PeXclITvvOnRRgiy5VOa51kWC9KRTzzJ0SSgl5k-WoHKyRtyk5FwubUNWjdEdCa3ltf70VCZPuNCgq46c_DIvWwpl-UMnoOQfEKAFHiMl2NAR4N-2Kr6yXaDjNuqOKkkIDx3KgncC50q6-UQUUojpj8ym0hhRLRN6YMpB-rtWzDVO1W57mfLXexoZ--uEPIE-an5Bjve6kHGgcMkFHHmuUwuzJp5-Qkzwe-8yutnfZaNGRt6c5fTsoaHrA4mo-tfNteVdcsmoduS5YHUY8frPnfncGKxD19SGOAe0CQy_YQuwh_JIKblBKcyA";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client = Graph::new_async(ACCESS_TOKEN);

    let users_resp: GraphResponse<Users> = client
        .v1()
        .users()
        .list_user()
        .select(&["id", "userPrincipalName"])
        .top("100")
        .json()
        .await.unwrap();

    let users = users_resp.into_body().value;
    println!("Found {} users.", users.len());

    let mut stream = stream::iter(&users)
        .map(|user| async {
            //let client = Graph::new_async(ACCESS_TOKEN); // <- line added as workaround for 0.1.2
            match get_user_license_details(&client, user.id.as_ref().unwrap()).await
            {
                Ok(license_details) => {
                    println!("Got {} license details", license_details.len());
                    Ok(license_details)
                }
                Err(e) => Err(e),
            }
        }).buffered(10);

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
    client: &Graph<graph_http::AsyncHttpClient>,
    user_id: &str,
) -> Result<Vec<LicenseDetail>, Box<dyn std::error::Error + Send + Sync>> {
    println!("Fetching license detail for {}", user_id);

    let license_details: GraphResponse<LicenseDetails> = client
        .v1()
        .users()
        .id(user_id)
        .list_license_details()
        .json()
        .await.unwrap();

    Ok(license_details.into_body().value)
}
