// use common::Client;
use yew::prelude::*;
use yewdux::prelude::*;

//replace the struct with the struct in the common struct
#[derive(Properties, PartialEq)]
pub struct Client {
    pub client_id: String,
    pub name: String,
    pub email: String,
    pub password: String,
}
#[function_component]
pub fn UserPage(client: &Client) -> Html {
    html! {
        <div>
            // <p>{client.name}</p>
        </div>
    }
}
