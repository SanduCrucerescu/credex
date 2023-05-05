use yew::prelude::*;
use yewdux::prelude::*;

#[function_component]
pub fn HomePage() -> Html {
    html! {
        <>
        <nav class={classes!("bg-grey-800")}>
        <div class={"mx-auto max-w-7xl px-2 sm:px-6 lg-8"}>
        <h1>{"Home page"}</h1>
        </div>

        </nav>
        </>
    }
}
