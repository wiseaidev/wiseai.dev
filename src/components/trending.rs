use yew::prelude::*;
use crate::components::projects::pagetitle;

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

#[function_component(Trending)]
pub fn trending_component() -> Html {
    let posts = vec![
        Post {
            id: 1,
            title: "Rust: The Next Big Thing in Data Science",
            url: "https://towardsdatascience.com/rust-the-next-big-thing-in-data-science-319a03305883",
            date: "24 Apr, 2023",
            thumb: "https://miro.medium.com/v2/resize:fit:720/format:webp/1*2jSP2n1KukVJYKVg2u4RuA.png",
            tags: vec![
                Tag {
                    name: "Data Science",
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
            title: "The Ultimate Ndarray Handbook: Mastering the Art of Scientific Computing with Rust",
            url: "https://towardsdatascience.com/the-ultimate-ndarray-handbook-mastering-the-art-of-scientific-computing-with-rust-ef5ab767212a",
            date: "02 May, 2023",
            thumb: "https://miro.medium.com/v2/resize:fit:720/format:webp/1*bgmO2hUgZXpCHPC1XaBy3w.png",
            tags: vec![
                Tag {
                    name: "Data Science",
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
            title: "Rust Polars: Unlocking High-Performance Data Analysis — Part 1",
            url: "https://towardsdatascience.com/rust-polars-unlocking-high-performance-data-analysis-part-1-ce42af370ece",
            date: "11 May, 2023",
            thumb: "https://miro.medium.com/v2/resize:fit:720/0*Le8YYCDuEhc4A7tN",
            tags: vec![
                Tag {
                    name: "Data Science",
                    url: "https://wiseai.dev/blog/tags/data-science",
                },
            ],
            author: Author {
                name: "Mahmoud Harmouch",
                avatar_url: "images/pic.png",
            },
        },
    ];

    html! {
        <div class={TRENDING_CONTAINER}>
            <section class={SECTION_CONTAINER} id="blog">
                { pagetitle("Trending Posts") }
                <div class={GRID_CONTAINER}>
                    { for posts.iter().map(|post| html! { <PostCard ..post.clone() /> }) }
                </div>
                <div class={FLEX_CENTER_CLASS}>
                    <a class={BUTTON_CLASS} href="/blog" title="Go To Blog" target="_blank" rel="noreferrer">
                        { "Go To Blog" }
                    </a>
                </div>
            </section>
        </div>
    }
}

#[function_component(PostCard)]
pub fn post_card(post: &Post) -> Html {
    html! {
        <div class={POST_CARD_CONTAINER}>
            <div class="flex-shrink-0">
                <a href={post.url} class={TAG_LINK} rel="tag" target="_blank" rel="noreferrer">
                    <img class={IMAGE_CONTAINER} src={post.thumb} alt={post.author.name} />
                </a>
            </div>
            <div class="p-4">
                <div class={DATE_TEXT}>{ &post.date }</div>
                <h2 class={TITLE_TEXT}>
                    <a href={post.url} title={post.title} target="_blank" rel="noreferrer">
                        { &post.title }
                    </a>
                </h2>
                <div class="flex space-x-2">
                    { for post.tags.iter().map(|tag| html! {
                        <a href={tag.url} class={TAG_LINK} rel="tag" target="_blank" rel="noreferrer">{ &tag.name }</a>
                    }) }
                </div>
                <div class="flex items-center mt-3">
                    <img class={AVATAR_IMAGE} src={post.author.avatar_url} alt={post.author.name} />
                    <span class={AUTHOR_NAME}>{ &post.author.name }</span>
                </div>
            </div>
        </div>
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
