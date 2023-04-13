use yew::prelude::*;
use yewdux::prelude::*;

use crate::api::api_fetch_client;
use crate::bank::{set_client, set_loading, Bank};

#[function_component]
pub fn UserInfo() -> Html {
    let (bank, dispatch) = use_store::<Bank>();

    use_effect_with_deps(
        move |_| {
            let dispatch = dispatch.clone();
            wasm_bindgen_futures::spawn_local(async move {
                set_loading(true, dispatch.clone());
                let response = api_fetch_client("C001").await;
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
    let clients = bank.clients.clone();
    html! {
        <div>
            <h1>{ "User Info" }</h1>
            <p>{ format!("Name: {}", clients[0].name) }</p>
            <p>{ format!("Email: {}", clients[0].email) }</p>
        </div>
    }
}
