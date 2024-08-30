use web_sys::HtmlInputElement;
use yew::prelude::*;
use serde_json::json;
use yew_router::hooks::use_navigator;
use reqwest::header::{AUTHORIZATION, HeaderValue};
use crate::router::Route;

#[function_component(Form)]
pub fn form() -> Html {

    let navigator = use_navigator().unwrap();

    let name_ref = NodeRef::default();
    let price_ref = NodeRef::default();
    let name_ref_outer = name_ref.clone();
    let price_ref_outer = price_ref.clone();

    let onclick = Callback::from( move |_| {
      let price = price_ref.cast::<HtmlInputElement>().unwrap();
      let name = name_ref.cast::<HtmlInputElement>().unwrap();
      
      wasm_bindgen_futures::spawn_local(async move {
        let product = json!({
          "name": name.value(),
          "price": price.value().parse::<i32>().unwrap()
        });
  
        let client = reqwest::Client::new();
        let _ = client.post("http://localhost:3000/api/products")
            .header(AUTHORIZATION, HeaderValue::from_str("Bearer your_secret_api_key").unwrap())
            .json(&product)
            .send()
            .await;
      });

      navigator.push(&Route::Home);
    });

    html!{
        <div class="container">
          <h2>{"Add a Product"}</h2>
          <div>
            <label for="name" class="">
              {"Name"}
              <input ref={name_ref_outer.clone()}
                id="name"
                class="formInput"
                type="text" 
              />
            </label> <br /> <br />
            <label for="price" class="">
              {"Price"}
              <input ref={price_ref_outer.clone()}
                id="price"
                class="formInput"
                type="number" 
              />
            </label>
            <br /> <br />
            <button {onclick} class="btn-primary">{"Add Product"}</button>
          </div>
          <hr />
        </div>
    }
}
