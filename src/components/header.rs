use crate::router::Route;
use yew::prelude::*;
use yew_router::prelude::*;

const HEADER_CLASS: &str = "fixed top-0 left-0 w-full bg-black text-white font-roboto z-20";
const LOGO_CLASS: &str = "flex items-center";
const LOGO_IMG_CLASS: &str = "w-32 md:w-40";
const MENU_TOGGLE_CLASS: &str = "btn-menu ml-4 md:hidden cursor-pointer";
const LINE_CLASS: &str = "line h-1 mb-1 bg-white transition-transform transform origin-center";
const FLEX_CONTAINER_CLASS: &str = "flex justify-between items-center";
const HIDDEN_MD_CLASS: &str = "hidden md:flex nav-wrap";
const NAV_CLASS: &str = "flex flex-grow justify-end items-center space-x-4 md:space-x-8";
const MENU_ITEM_CLASS: &str = "nav-link text-white hover:text-gray-300 transition-colors";
const HIRE_ME_CLASS: &str = "hire-me";
const HIRE_ME_BUTTON_CLASS: &str =
    "rounded-full py-2 px-6 bg-blue-500 text-white text-lg transition-colors hover:bg-blue-600";

#[derive(Clone)]
pub struct Menu {
    pub id: usize,
    pub link: &'static str,
    pub name: &'static str,
}

#[function_component(Header)]
pub fn header_component() -> Html {
    let menus: Vec<Menu> = vec![
        Menu {
            id: 1,
            link: "#experience",
            name: "Experience",
        },
        Menu {
            id: 2,
            link: "#projects",
            name: "Projects",
        },
        Menu {
            id: 3,
            link: "#certifications",
            name: "Certifications",
        },
        Menu {
            id: 4,
            link: "#skills",
            name: "Skills",
        },
        Menu {
            id: 5,
            link: "#blog",
            name: "Blog",
        },
        Menu {
            id: 6,
            link: "#contact",
            name: "Contact",
        },
    ];

    html! {
        <header id="header" class={HEADER_CLASS}>
            <div class="container mx-auto px-4 py-2">{ render_header_content(&menus) }</div>
        </header>
    }
}

fn render_header_content(menus: &[Menu]) -> Html {
    html! {
        <div class={FLEX_CONTAINER_CLASS}>
            { render_logo() }
            { render_menu(menus) }
            { render_menu_toggle() }
        </div>
    }
}

fn render_logo() -> Html {
    html! {
        <div id="logo" class={LOGO_CLASS}>
            <Link<Route> to={Route::LandingPage} classes="nav-link">
                <img src="images/logo.png" alt="logo" class={LOGO_IMG_CLASS} />
            </Link<Route>>
        </div>
    }
}

fn render_menu_toggle() -> Html {
    html! {
        <div class={MENU_TOGGLE_CLASS}>
            <div class={format!("{} w-6", LINE_CLASS)} />
            <div class={format!("{} w-8", LINE_CLASS)} />
            <div class={format!("{} w-6", LINE_CLASS)} />
        </div>
    }
}

fn render_menu(menus: &[Menu]) -> Html {
    html! {
        <div class={NAV_CLASS}>
            <div class={HIDDEN_MD_CLASS}>
                <nav id="mainnav" class="mainnav" data-menu-style="horizontal">
                    <ul class="flex space-x-4 md:space-x-8">
                        { for menus.iter().map(|menu| render_menu_item(menu)) }
                    </ul>
                </nav>
            </div>
            { render_hire_me_button() }
        </div>
    }
}

fn render_menu_item(menu: &Menu) -> Html {
    html! {
        <li key={menu.id}>
            <a class={MENU_ITEM_CLASS} href={menu.link.to_string()}>{ menu.name }</a>
        </li>
    }
}

fn render_hire_me_button() -> Html {
    html! {
        <div class={HIRE_ME_CLASS}>
            <a href="mailto:business@wiseai.dev" class={HIRE_ME_BUTTON_CLASS}><b>{ "HireMe" }</b></a>
        </div>
    }
}
