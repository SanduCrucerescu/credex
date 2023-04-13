use std::{cell::RefCell, ops::Deref};

use crate::{api::api_fetch_client, bank::Bank};
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationErrors};
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::{navigator, prelude::use_navigator};
use yewdux::prelude::*;

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
struct LoginClientSchema {
    #[validate(
        length(min = 1, message = "Email required"),
        email(message = "Must be an email")
    )]
    email: String,
    #[validate(
        length(min = 1, message = "Password is requred"),
        length(min = 6, message = "Passoword must be at least 6 characters long")
    )]
    pw: String,
}

fn get_input(
    name: &'static str,
    cloned_form: UseStateHandle<LoginClientSchema>,
) -> Callback<String> {
    Callback::from(move |value| {
        let mut data = cloned_form.deref().clone();
        match name {
            "email" => data.email = value,
            "pw" => data.pw = value,
            _ => (),
        }
        cloned_form.set(data);
    })
}

#[function_component]
pub fn LoginPage() -> Html {
    let (bank, dispatch) = use_store::<Bank>();
    let form = use_state(|| LoginClientSchema::default());
    let validation_errors = use_state(|| Rc::new(RefCell::new(ValidationErrors::new())));
    let navigator = use_navigator().unwrap();

    html! {
        <>
        <form>
            <label> {"Email:"} </label> <br />
            <input type="text" /> <br/>
            <label> {"Password:"} </label> <br/>
            <input type="text" /> <br/> <br/>
            <button type="submit"> {"Submit"} </button>
        </form>
        </>
    }
}
