use yew::prelude::*;

const BG_BLACK_CLASS: &str = "bg-black shadow-dark rounded w-full xs:w-1/2";
const PROJECT_IMAGE_CLASS: &str =
    "project-image w-auto h-64 object-cover rounded-t w-full xs:w-1/2";
const PROJECT_ITEM_CLASS: &str =
    "project-item relative rounded shadow-dark border-2 border-blue-500 shadow-md overflow-hidden w-full";
const DETAILS_CLASS: &str = "absolute bottom-0 left-0 w-full h-full p-4 bg-black opacity-0 hover:opacity-90 transition-opacity duration-300";
const TERM_CLASS: &str = "term text-capitalize text-white";
const TITLE_CLASS: &str = "title text-white";
const MORE_BUTTON_CLASS: &str = "more-button";
const FAB_CLASS: &str = "fab fa-github text-primary";

#[derive(Properties, Clone, PartialEq)]
pub struct ProjectData {
    #[prop_or_default]
    pub id: u32,
    #[prop_or_default]
    pub title: AttrValue,
    #[prop_or_default]
    pub category: AttrValue,
    #[prop_or_default]
    pub image: AttrValue,
    #[prop_or_default]
    pub link: AttrValue,
}

#[function_component(Project)]
pub fn project(props: &ProjectData) -> Html {
    let toggler = use_state(|| false);

    let handle_lightbox = {
        let link = props.link.clone();
        Callback::from(move |_| {
            if link.is_empty() {
                toggler.set(!false);
            }
        })
    };
    html! {
        <div class={BG_BLACK_CLASS}>
            <a
                href={props.link.clone()}
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
                            src={&props.image}
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
