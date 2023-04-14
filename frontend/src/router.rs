use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::{home_page::HomePage, login_page::LoginPage};
#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    HomePage,
    #[at("/login")]
    LoginPage,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::LoginPage => html! {<LoginPage/>},
        Route::HomePage => html! {<HomePage />},
    }
}
