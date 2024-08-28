use yew::prelude::*;
use yew_router::prelude::*;
use serde::{Deserialize, Serialize};
use crate::router::Route;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Product {
    id: u64,
    name: String,
    price: i32
}

#[function_component(Products)]
pub fn products() -> Html {

    let data: UseStateHandle<Vec<Product>> = use_state(|| vec![]);
    let data_clone = data.clone();

    {
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_data = reqwest::get("http://localhost:3000/api/products")
                .await
                .expect("cannot get data from url")
                .json::<Vec<Product>>()
                .await
                .expect("cannot convert to json");
                data_clone.set(fetched_data);
            });
            || ()
        });
    }

    let products = data.iter().map(|product| html! {
        <ul>
            <li key={product.id}>{format!("Name: {}, Price {}", product.name, product.price)}</li>
        </ul>
    }).collect::<Html>();
    
    html!(
        <div class="container">
            <button class="btn-primary">
                <Link<Route> to={Route::AddProduct}>{"Add new Product"}</Link<Route>>
            </button>
            <h2>{"List of Products: "} {data.len()}</h2>
            <p>{products}</p>
        </div>
    )
}
