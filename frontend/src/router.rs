use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::login_page::LoginPage;
#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    LoginPage,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::LoginPage => html! {<LoginPage/>},
    }
}
