use std::todo;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::landing::LandingPage;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    LandingPage,
    #[at("/blog/:id")]
    Blog { id: usize },
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::LandingPage => html! { <LandingPage/> },
        Route::Blog { .. } => todo!(),
    }
}
