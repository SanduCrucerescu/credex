use crate::router;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::{navigator, prelude::use_navigator};
#[function_component]
pub fn NavigationBar() -> Html {
    let navigator = use_navigator().unwrap();
    let on_click = {
        let cloned_navigator = navigator.clone();
        move |_| cloned_navigator.push(&router::Route::LoginPage)
    };

    html! {
        <nav class="bg-gray-100">
            <div class="mx-auto max-w-7xl px-2 sm:px-6 lg:px-8">
                <div class="relative flex h-16 items-center justify-between">
                    <div class="sm:mx-auto sm:max-w-sm items-left">
                        <img class="mx-auto h-11 w-auto" src="img/icon.png" alt="logo"/>
                    </div>
                    <div class="flex flex-1 items-center justify-center sm:items-stretch sm:justify-start">
                        <div class="flex space-x-4">
                            <button class="text-teal-700 hover:bg-teal-500 hover:text-white rounded-md px-3 py-2 text-sm font-medium">{"Dashboard"}</button>
                            <button class="text-teal-700 hover:bg-teal-500 hover:text-white rounded-md px-3 py-2 text-sm font-medium">{"Our services"}</button>
                            <button class="text-teal-700 hover:bg-teal-500 hover:text-white rounded-md px-3 py-2 text-sm font-medium">{"About us"}</button>
                            <button class="text-teal-700 hover:bg-teal-500 hover:text-white rounded-md px-3 py-2 text-sm font-medium">{"Contact us"}</button>
                        </div>
                    </div>
                    <div class="flex flex-4 items-end justify-center sm:items-stretch sm:justify-start">
                        <button onclick={on_click} class="text-teal-700 hover:bg-teal-500 hover:text-white rounded-md px-3 py-2 text-sm font-medium">{"Sign In"}</button>
                    </div>
                </div>
            </div>

        </nav>
    }
}
