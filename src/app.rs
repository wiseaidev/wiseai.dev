use next_rs::prelude::*;
use next_rs::router::*;

use crate::router::{switch, Route};

#[function_component(App)]
pub fn app() -> Html {
    html! { <BrowserRouter><Switch<Route> render={switch} /></BrowserRouter> }
}
