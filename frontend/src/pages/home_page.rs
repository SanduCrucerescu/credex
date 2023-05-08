use crate::components::navigation_bar::NavigationBar;
use yew::prelude::*;
use yewdux::prelude::*;

#[function_component]
pub fn HomePage() -> Html {
    html! {
        <>
        <NavigationBar/>
        </>
    }
}
