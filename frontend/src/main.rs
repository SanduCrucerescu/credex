mod api;
mod bank;
mod components;
mod pages;
mod router;

use crate::router::*;
use bank::Bank;
use yew::prelude::*;
use yew_router::{BrowserRouter, Switch};
use yewdux::prelude::*;

#[function_component]
fn App() -> Html {
    let (bank, _) = use_store::<Bank>();
    let loading = &bank.loading;

    html! {
        <BrowserRouter>
            <Switch<Route> render={switch}/>
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
