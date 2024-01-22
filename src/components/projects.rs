use crate::components::project::{Project, ProjectData};
use yew::prelude::*;

const SECTION_CLASS: &str = "min-h-screen p-6";
const CONTAINER_CLASS: &str = "container mx-auto";
const PROJECT_WRAPPER_CLASS: &str =
    "grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-x-52 md:gap-x-30 sm:gap-x-12 gap-y-12";
const SECTION_TITLE_CLASS: &str = "section-title lt-sp04 text-center text-white text-4xl font-bold";
const SPACER_CLASS: &str = "spacer";

#[function_component(Projects)]
pub fn projects() -> Html {
    let get_all_items = projects_data();
    html! {
        <section id="projects" class={SECTION_CLASS}>
            <div class={CONTAINER_CLASS}>
                { pagetitle("Featured Projects") }
                <div class={PROJECT_WRAPPER_CLASS}>
                    { for get_all_items.iter().map(|item| html! { <Project ..item.clone() /> }) }
                </div>
            </div>
        </section>
    }
}

pub fn pagetitle(title: &str) -> Html {
    html! {
        <>
            <h1 class={SECTION_TITLE_CLASS}>{ title }</h1>
            <div class={SPACER_CLASS} style="height: 60px;" />
        </>
    }
}

fn projects_data() -> Vec<ProjectData> {
    vec![
        ProjectData {
            id: 1,
            title: "Brave Chat: A fully featured chat application.",
            category: "chat app",
            image: "https://raw.githubusercontent.com/brave-chat/brave-chat/main/docs/static/images/banner.PNG",
            link: "https://github.com/brave-chat",
        },
        ProjectData {
            id: 2,
            title: "Brave Date: A fully featured dating platform.",
            category: "dating app",
            image: "https://raw.githubusercontent.com/brave-date/brave-date/main/docs/static/images/Banner.png",
            link: "https://github.com/brave-date",
        },
        ProjectData {
            id: 3,
            title: "Fine Dashboard: A fully featured admin panel and dashboard.",
            category: "admin panel",
            link: "https://github.com/wiseaidev/fine-dashboard",
            image: "https://raw.githubusercontent.com/wiseaidev/fine-dashboard/main/assets/banner.png",
        },
        ProjectData {
            id: 4,
            title: "Flexify: An add-on to customize any website with a click of a button.",
            category: "add-on",
            link: "https://github.com/inclusive-web-hub/flexify",
            image: "https://addons.mozilla.org/user-media/previews/full/279/279758.png?modified=1678549737",
        },
        ProjectData {
            id: 5,
            title: "Hill Climb ThirdWeb: A web3 clone of the Hill Climb game.",
            category: "web3",
            link: "https://github.com/hill-climb",
            image: "https://dev-to-uploads.s3.amazonaws.com/uploads/articles/3797ehvmagk9ysrljuh2.PNG",
        },
        ProjectData {
            id: 6,
            title: "Truth Guard: A Fully Async-based Fake news detection API.",
            category: "API",
            link: "https://github.com/wiseaidev/truth-guard",
            image: "https://raw.githubusercontent.com/wiseaidev/truth-guard/main/assets/banner.png",
        },
    ]
}
