use std::{cell::RefCell, ops::Deref, rc::Rc};

use crate::{
    api::{api_fetch_client, api_login_client},
    bank::{set_loading, Bank},
    components::form_input::FormInput,
    router,
};
use gloo::console::log;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationErrors};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::{navigator, prelude::use_navigator};
use yewdux::prelude::*;

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
struct LoginClientSchema {
    // #[validate(
    //     length(min = 1, message = "Email required"),
    //     email(message = "Must be an email")
    // )]
    email: String,
    // #[validate(
    //     length(min = 1, message = "Password is requred"),
    //     length(min = 6, message = "Passoword must be at least 6 characters long")
    // )]
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
    let email_input_ref = NodeRef::default();
    let pw_input_ref = NodeRef::default();

    let validate_input_on_blur = {
        let cloned_form = form.clone();
        let cloned_validation_errors = validation_errors.clone();
        Callback::from(move |(name, value): (String, String)| {
            let mut data = cloned_form.deref().clone();
            match name.as_str() {
                "email" => data.email = value,
                "pw" => data.pw = value,
                _ => (),
            }
            cloned_form.set(data);

            match cloned_form.validate() {
                Ok(_) => {
                    cloned_validation_errors
                        .borrow_mut()
                        .errors_mut()
                        .remove(name.as_str());
                }
                Err(err) => {
                    cloned_validation_errors
                        .borrow_mut()
                        .errors_mut()
                        .retain(|key, _| key != &name);
                    for (field_name, error) in err.errors() {
                        if field_name == &name {
                            cloned_validation_errors
                                .borrow_mut()
                                .errors_mut()
                                .insert(field_name.clone(), error.clone());
                        }
                    }
                }
            }
        })
    };

    let handle_email_input = get_input("email", form.clone());
    let handle_pw_input = get_input("pw", form.clone());

    let on_submit = {
        let cloned_from = form.clone();
        let clone_dispatch = dispatch.clone();
        let clone_email_input_ref = email_input_ref.clone();
        let clone_pw_input_ref = pw_input_ref.clone();
        let clone_validation_errors = validation_errors.clone();

        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();
            let form = cloned_from.clone();
            let validation_errors = clone_validation_errors.clone();
            let dispatch = clone_dispatch.clone();
            let cloned_navigator = navigator.clone();
            let email_input_ref = clone_email_input_ref.clone();
            let pw_input_ref = clone_pw_input_ref.clone();

            spawn_local(async move {
                match form.validate() {
                    Ok(_) => {
                        log!("here");
                        let form_data = form.deref().clone();
                        set_loading(true, dispatch.clone());

                        let email_input = email_input_ref.cast::<HtmlInputElement>().unwrap();
                        let pw_input = pw_input_ref.cast::<HtmlInputElement>().unwrap();

                        email_input.set_value("");
                        pw_input.set_value("");

                        let form_json = serde_json::to_string(&form_data).unwrap();
                        log!(&form_json);
                        let res = api_login_client(&form_json).await;
                        match res {
                            Ok(_) => {
                                set_loading(false, dispatch);
                                cloned_navigator.push(&router::Route::HomePage);
                            }
                            Err(err) => {
                                set_loading(false, dispatch);
                                // log!(err.to_string())
                            }
                        }
                    }
                    Err(err) => validation_errors.set(Rc::new(RefCell::new(err))),
                }
            });
        })
    };

    html! {
        <>
        <form onsubmit={on_submit}>
            <label> {"Email:"} </label> <br />
            <FormInput name="email" label="email" input_type="text" handle_onchange={handle_email_input} input_ref={email_input_ref} handle_on_input_blur={validate_input_on_blur.clone()} errors={&*validation_errors} /> <br/>
            <label> {"Password:"} </label> <br/>
            <FormInput name="p" label="pw" input_type="text" handle_onchange={handle_pw_input} input_ref={pw_input_ref} handle_on_input_blur={validate_input_on_blur.clone()} errors={&*validation_errors}/> <br/> <br/>
            <button type="submit"> {"Submit"} </button>
        </form>
        </>
    }
}
