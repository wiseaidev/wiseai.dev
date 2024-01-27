use next_rs::prelude::*;

const BG_BLACK_CLASS: &str = "bg-black shadow-dark rounded w-full xs:w-1/2";
const PROJECT_IMAGE_CLASS: &str =
    "project-image w-auto h-64 object-cover rounded-t w-full xs:w-1/2";
const PROJECT_ITEM_CLASS: &str =
    "project-item relative rounded shadow-dark border-2 border-blue-500 shadow-md overflow-hidden w-full text-center";
const DETAILS_CLASS: &str = "absolute bottom-0 left-0 w-full h-full bg-black opacity-0 hover:opacity-90 transition-opacity duration-300";
const TERM_CLASS: &str =
    "term text-blue-900 bg-white rounded-bl-lg rounded-br-lg text-sm px-4 py-2";
const TITLE_CLASS: &str = "title text-white text-xl pt-10";
const MORE_BUTTON_CLASS: &str = "more-button text-white text-2xl";
const FAB_CLASS: &str = "fab fa-github text-primary";

#[derive(Properties, Clone, PartialEq)]
pub struct ProjectData {
    #[prop_or_default]
    pub id: u32,
    #[prop_or_default]
    pub title: &'static str,
    #[prop_or_default]
    pub category: &'static str,
    #[prop_or_default]
    pub image: &'static str,
    #[prop_or_default]
    pub link: &'static str,
}

#[function_component(Project)]
pub fn project(props: &ProjectData) -> Html {
    let toggler = use_state(|| false);

    let handle_lightbox = {
        let link = props.link;
        Callback::from(move |_| {
            if link.is_empty() {
                toggler.set(!false);
            }
        })
    };
    html! {
        <div class={BG_BLACK_CLASS}>
            <a
                href={props.link}
                class={PROJECT_IMAGE_CLASS}
                as="project"
                onclick={handle_lightbox}
                target="_blank"
                rel="noreferrer"
            >
                <div class={PROJECT_ITEM_CLASS}>
                    <div class="relative">
                        <img
                            class={PROJECT_IMAGE_CLASS}
                            src={&*props.image}
                            alt="Project-title"
                            loading="lazy"
                            style="height: 220px;"
                        />
                        <div class={DETAILS_CLASS}>
                            <span class={TERM_CLASS}>{ &props.category }</span>
                            <h4 class={TITLE_CLASS}>{ &props.title }</h4>
                            <span class={MORE_BUTTON_CLASS}><i class={FAB_CLASS} /></span>
                        </div>
                    </div>
                </div>
            </a>
        </div>
    }
}
