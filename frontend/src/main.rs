use yew::prelude::*;
use yew_router::prelude::*;

mod products;
mod form;

mod router;
use router::{Route, switch};

#[function_component(App)]
fn app() -> Html {
    html! {
      <BrowserRouter>
        <div class="container">
          <h1 class="title">{"Yew Products App"}</h1>
          <Switch<Route> render={switch} />
        </div>
      </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
