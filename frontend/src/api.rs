use common::{ClientLoginRespons, ClientModel};
use reqwasm::http;

pub async fn api_fetch_client(client_id: &str) -> Result<ClientModel, String> {
    let response = match http::Request::get(
        format!("http://127.0.0.1:8080/api/client/{}", client_id).as_str(),
    )
    .send()
    .await
    {
        Ok(response) => response,
        Err(_) => return Err("Failed to make a response".to_string()),
    };

    let res_json = response.json::<ClientModel>().await;
    match res_json {
        Ok(data) => Ok(data),
        Err(err) => Err(err.to_string()),
    }
}

pub async fn api_login_client(data: &str) -> Result<ClientLoginRespons, String> {
    let response = match http::Request::post("http://127.0.0.1:8080/api/client/login")
        .header("Content-Type", "application/json")
        .body(data)
        .send()
        .await
    {
        Ok(res) => res,
        Err(_) => return Err("Failed to make the request".to_string()),
    };

    // if response.status() != 200 {
    //     let error_res = response.json::<ErrorResponse>().await;

    //     if let Ok(error_res) = error_res {
    //         return Err(error_res.message);
    //     } else {
    //         return Err("API error".to_string());
    //     }
    // }
    let res_json = response.json::<ClientLoginRespons>().await;
    match res_json {
        Ok(data) => Ok(data),
        Err(_) => Err("Failed to parse".to_string()),
    }
}
