use next_rs::prelude::*;
use next_rs::{Image, ImageProps, Link};

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

#[func]
pub fn Project(props: &ProjectData) -> Html {
    let image_props = ImageProps {
        src: &*props.image,
        alt: "Project-title",
        width: "400",
        height: "300",
        style: "",
        class: PROJECT_IMAGE_CLASS,
        sizes: "(max-width: 768px) 100vw, (max-width: 1200px) 50vw, 33vw",
        quality: "80",
        priority: true,
        placeholder: "blur",
        on_loading_complete: Callback::from(|_| {
            println!("Image loading is complete!");
        }),
        object_fit: "cover",
        object_position: "center",
        on_error: Callback::from(|err| {
            println!("Error loading image: {:?}", err);
        }),
        decoding: "async",
        blur_data_url: "data:image/png;base64,....",
        lazy_boundary: "200px",
        unoptimized: false,
        node_ref: NodeRef::default(),
    };

    html! {
        <div class={BG_BLACK_CLASS}>
            <Link
                to={props.link}
                class={PROJECT_IMAGE_CLASS}
                target="_blank"
                rel="noreferrer"
            >
                <div class={PROJECT_ITEM_CLASS}>
                    <div class="relative">
                        <Image ..image_props />
                        <div class={DETAILS_CLASS}>
                            <span class={TERM_CLASS}>{ &props.category }</span>
                            <h4 class={TITLE_CLASS}>{ &props.title }</h4>
                            <span class={MORE_BUTTON_CLASS}><i class={FAB_CLASS} /></span>
                        </div>
                    </div>
                </div>
            </Link>
        </div>
    }
}
