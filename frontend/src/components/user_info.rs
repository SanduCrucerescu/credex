use gloo::console::log;
use wasm_bindgen::JsValue;
use yew::prelude::*;
use yewdux::prelude::*;

use crate::api::api_fetch_client;
use crate::bank::{set_client, set_loading, Bank};

#[function_component]
pub fn UserInfo() -> Html {
    let (bank, dispatch) = use_store::<Bank>();
    // let clients = bank.clients.clone();

    use_effect_with_deps(
        move |_| {
            let dispatch = dispatch.clone();
            wasm_bindgen_futures::spawn_local(async move {
                set_loading(true, dispatch.clone());
                let response = api_fetch_client("1").await;
                let r = JsValue::from(response.clone().unwrap().name);
                log!(r);
                match response {
                    Ok(client) => {
                        set_loading(false, dispatch.clone());
                        set_client(client, dispatch);
                    }
                    Err(err) => {
                        set_loading(false, dispatch.clone());
                        println!("Error: {}", err);
                    }
                }
            })
        },
        (),
    );
    // let o = JsValue::from(&clients[0].clone());
    log!("here");
    let clients = bank.clients.clone();
    // log!(clients[0].name.clone());
    html! {
        <div>
            <h1>{ "User Info" }</h1>
            <p>{ format!("Name: {}", clients[0].name) }</p>
            <p>{ format!("Email: {}", clients[0].email) }</p>
            <p>{ format!("Balance: {}", clients[0].balance) }</p>
        </div>
    }
}
