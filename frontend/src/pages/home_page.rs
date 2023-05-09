use crate::components::navigation_bar::NavigationBar;
use yew::prelude::*;
use yewdux::prelude::*;

#[function_component]
pub fn HomePage() -> Html {
    html! {
        <>
        <NavigationBar/>
        <div class="flex min-h-full justify-center">
            <h1>{"Bruh"}</h1>
        </div>
        </>
    }
}
