use crate::components::projects::pagetitle;
use next_rs::prelude::*;
use next_rs::{Image, ImageProps, Link, YewI18n};
use web_sys::HtmlInputElement;

const TRENDING_CONTAINER: &str = "flex items-center justify-center min-h-screen";
const SECTION_CONTAINER: &str = "trending-container max-w-screen-lg mx-auto p-4";
const GRID_CONTAINER: &str = "grid grid-cols-1 gap-16 sm:grid-cols-2 lg:grid-cols-3";
const POST_CARD_CONTAINER: &str = "bg-black rounded-lg overflow-hidden shadow-md transition duration-500 transform hover:scale-105";
const TAG_LINK: &str = "text-xs text-blue-500 hover:underline";
const AUTHOR_NAME: &str = "text-sm text-gray-400";
const IMAGE_CONTAINER: &str = "w-full h-48 object-cover";
const DATE_TEXT: &str = "text-xs text-gray-200 mb-2";
const TITLE_TEXT: &str = "text-lg font-semibold text-gray-200 mb-2";
const AVATAR_IMAGE: &str = "w-8 h-8 rounded-full mr-2 object-cover";
const BUTTON_CLASS: &str =
    "rounded-full py-2 px-6 bg-blue-500 text-white text-lg transition-colors hover:bg-blue-600";
const FLEX_CENTER_CLASS: &str = "flex justify-center mt-4";
const SELECT_CLASS: &str =
    "border border-blue-500 rounded p-2 m-2 mb-14 bg-gray-800 text-white cursor-pointer w-45 h-10";
const OPTION_CLASS: &str = "text-lg";

#[func]
pub fn Trending() -> Html {
    let mut i18n = use_context::<YewI18n>().expect("No I18n context provided");

    let selected_language_ref = use_node_ref();
    let selected_language_handle = use_state(|| "en".to_string());
    let selected_language = (*selected_language_handle).clone();

    let _ = i18n.set_translation_language(&selected_language);

    let posts = vec![
        Post {
            id: 1,
            title: Box::leak(i18n.t("Rust: The Next Big Thing in Data Science").into_boxed_str()),
            url: "https://towardsdatascience.com/rust-the-next-big-thing-in-data-science-319a03305883",
            date: Box::leak(i18n.t("24 Apr, 2023").into_boxed_str()),
            thumb: "https://miro.medium.com/v2/resize:fit:720/format:webp/1*2jSP2n1KukVJYKVg2u4RuA.png",
            tags: vec![
                Tag {
                    name: Box::leak(i18n.t("Data Science").into_boxed_str()),
                    url: "https://wiseai.dev/blog/tags/data-science",
                },
            ],
            author: Author {
                name: "Mahmoud Harmouch",
                avatar_url: "images/pic.png",
            },
        },
        Post {
            id: 2,
            title: Box::leak(i18n.t("The Ultimate Ndarray Handbook: Mastering the Art of Scientific Computing with Rust").into_boxed_str()),
            url: "https://towardsdatascience.com/the-ultimate-ndarray-handbook-mastering-the-art-of-scientific-computing-with-rust-ef5ab767212a",
            date: Box::leak(i18n.t("02 May, 2023").into_boxed_str()),
            thumb: "https://miro.medium.com/v2/resize:fit:720/format:webp/1*bgmO2hUgZXpCHPC1XaBy3w.png",
            tags: vec![
                Tag {
                    name: Box::leak(i18n.t("Data Science").into_boxed_str()),
                    url: "https://wiseai.dev/blog/tags/data-science",
                },
            ],
            author: Author {
                name: "Mahmoud Harmouch",
                avatar_url: "images/pic.png",
            },
        },
        Post {
            id: 3,
            title: Box::leak(i18n.t("Rust Polars: Unlocking High-Performance Data Analysis — Part 1").into_boxed_str()),
            url: "https://towardsdatascience.com/rust-polars-unlocking-high-performance-data-analysis-part-1-ce42af370ece",
            date: Box::leak(i18n.t("11 May, 2023").into_boxed_str()),
            thumb: "https://miro.medium.com/v2/resize:fit:720/0*Le8YYCDuEhc4A7tN",
            tags: vec![
                Tag {
                    name: Box::leak(i18n.t("Data Science").into_boxed_str()),
                    url: "https://wiseai.dev/blog/tags/data-science",
                },
            ],
            author: Author {
                name: "Mahmoud Harmouch",
                avatar_url: "images/pic.png",
            },
        },
    ];

    let on_select_change = {
        let selected_language_ref = selected_language_ref.clone();
        let selected_language_handle = selected_language_handle.clone();
        Callback::from(move |_| {
            if let Some(input) = selected_language_ref.cast::<HtmlInputElement>() {
                let value = input.value();
                selected_language_handle.set(value);
            }
        })
    };

    html! {
        <div class={TRENDING_CONTAINER}>
            <section class={SECTION_CONTAINER} id="blog">
                { pagetitle(&i18n.t("Trending Posts")) }
                <select
                    ref={selected_language_ref}
                    onchange={on_select_change}
                    class={SELECT_CLASS}
                >
                    <option value="en" selected=true hidden=true>{ "Select Language" }</option>
                    { for i18n.config.supported_languages.iter().map(|&lang| render_language_option(lang)) }
                </select>
                <div class={GRID_CONTAINER}>
                    { for posts.iter().map(|post| html! { <PostCard ..post.clone() /> }) }
                </div>
                <div class={FLEX_CENTER_CLASS}>
                    <Link
                        class={BUTTON_CLASS}
                        to="/blog"
                        target="_blank"
                        rel="noreferrer"
                    >{ "Go To Blog" }</Link>
                </div>
            </section>
        </div>
    }
}

#[func]
pub fn PostCard(post: &Post) -> Html {
    let image_props1 = ImageProps {
        src: &*post.thumb,
        alt: post.author.name,
        width: "400",
        height: "300",
        style: "",
        class: IMAGE_CONTAINER,
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

    let image_props2 = ImageProps {
        src: &*post.author.avatar_url,
        alt: post.author.name,
        width: "400",
        height: "300",
        style: "",
        class: AVATAR_IMAGE,
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
        <div class={POST_CARD_CONTAINER}>
            <div class="flex-shrink-0">
                <Link to={post.url} class={TAG_LINK} rel="tag" target="_blank" rel="noreferrer">
                    <Image ..image_props1 />
                </Link>
            </div>
            <div class="p-4">
                <div class={DATE_TEXT}>{ &post.date }</div>
                <h2 class={TITLE_TEXT}>
                    <Link to={post.url} target="_blank" rel="noreferrer">
                        { post.title }
                    </Link>
                </h2>
                <div class="flex space-x-2">
                    { for post.tags.iter().map(|tag| html! {
                        <Link to={tag.url} class={TAG_LINK} target="_blank" rel="noreferrer">{ &tag.name }</Link>
                    }) }
                </div>
                <div class="flex items-center mt-3">
                    <Image ..image_props2 />
                    <span class={AUTHOR_NAME}>{ &post.author.name }</span>
                </div>
            </div>
        </div>
    }
}

fn render_language_option(lang: &'static str) -> Html {
    let flag_emoji = match lang {
        "en" => "🇺🇸",
        "fr" => "🇫🇷",
        "de" => "🇩🇪",
        "es" => "🇪🇸",
        _ => "🌐",
    };

    html! {
        <option class={OPTION_CLASS} value={lang}>{ format!("{} {}", flag_emoji, lang) }</option>
    }
}

#[derive(Clone, Properties, PartialEq)]
pub struct Post {
    pub id: usize,
    pub title: &'static str,
    pub thumb: &'static str,
    pub url: &'static str,
    pub date: &'static str,
    pub tags: Vec<Tag>,
    pub author: Author,
}

#[derive(Clone, Properties, PartialEq)]
pub struct Tag {
    pub name: &'static str,
    pub url: &'static str,
}

#[derive(Clone, Properties, PartialEq)]
pub struct Author {
    pub name: &'static str,
    pub avatar_url: &'static str,
}
