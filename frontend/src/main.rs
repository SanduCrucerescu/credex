mod api;
mod bank;
mod components;

use bank::Bank;
use components::user_info::UserInfo;
use yew::prelude::*;
use yewdux::prelude::*;

#[function_component]
fn App() -> Html {
    let (bank, _) = use_store::<Bank>();
    let loading = &bank.loading;

    html! {
        <>
        <h1>{"Hello World!"}</h1>
        <main>
        <UserInfo />
        </main>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
