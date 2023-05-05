use std::{cell::RefCell, rc::Rc};

use validator::ValidationErrors;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub input_type: Option<String>,
    pub label: String,
    pub name: String,
    pub input_ref: NodeRef,
    pub handle_onchange: Callback<String>,
    pub handle_on_input_blur: Callback<(String, String)>,
    pub errors: Rc<RefCell<ValidationErrors>>,
}

#[function_component]
pub fn FormInput(props: &Props) -> Html {
    let input_type = props
        .input_type
        .clone()
        .unwrap_or_else(|| "text".to_string());
    let val_errors = props.errors.borrow();
    let errors = val_errors.field_errors().clone();
    let empty_errors = vec![];
    let errors = match errors.get(&props.name.as_str()) {
        Some(err) => err,
        None => &empty_errors,
    };
    let error_msg = match errors.get(0) {
        Some(msg) => msg.to_string(),
        None => "".to_string(),
    };

    let handle_onchange = props.handle_onchange.clone();
    let onchange = Callback::from(move |event: Event| {
        let target = event.target().unwrap();
        let val = target.unchecked_into::<HtmlInputElement>().value();
        handle_onchange.emit(val);
    });

    let handle_on_input_blur = props.handle_on_input_blur.clone();
    let onblur = {
        let cloned_input_name = props.name.clone();
        Callback::from(move |event: FocusEvent| {
            let input_name = cloned_input_name.clone();
            let target = event.target().unwrap();
            let val = target.unchecked_into::<HtmlInputElement>().value();
            handle_on_input_blur.emit((input_name, val));
        })
    };

    html! {
        <div>
        <input
        class="block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus: ring-2 focus: ring-inset focus:ring-teal-700 sm:text-sm sm:leading-6"
        type={input_type}
        placeholder=""
        ref={props.input_ref.clone()}
        onchange={onchange}
        onblur={onblur}/>
        <span> {error_msg} </span>
        </div>
    }
}
