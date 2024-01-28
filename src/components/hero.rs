use next_rs::prelude::*;
use next_rs::Link;

#[derive(Clone, PartialEq, Properties)]
pub struct ProfileCardProps {
    #[prop_or_default]
    pub name: AttrValue,
    #[prop_or_default]
    pub verified: bool,
    #[prop_or_default]
    pub bio: AttrValue,
    #[prop_or_default]
    pub location: AttrValue,
    #[prop_or_default]
    pub image_src: AttrValue,
}

const WRAPPER_CLASS: &str = "w-full min-h-screen flex items-center justify-center pt-16";
const HERO_CARD_CLASS: &str =
    "w-full max-w-2xl bg-black border-2 border-blue-500 shadow-md rounded-lg mx-5";
const HERO_CARD_IMG_CLASS: &str =
    "w-40 h-40 mx-auto rounded-full border-blue-500 relative shadow-2xl absolute -top-20 border-2";
const HERO_CARD_IMG_IMG_CLASS: &str =
    "block w-full h-full object-cover object-center rounded-full z-10";
const HERO_CARD_CNT_CLASS: &str = "text-center px-5 pb-10 transition-all duration-300";
const HERO_CARD_NAME_CLASS: &str = "font-bold text-3xl text-white mb-4";
const HERO_CARD_CHECK_CIRCLE_CLASS: &str = "fa-check-circle text-blue-400 ml-2";
const HERO_CARD_TXT_CLASS: &str = "text-lg font-medium text-gray-500 mb-4";
const HERO_CARD_LOC_CLASS: &str = "flex items-center justify-center text-2xl font-semibold";
const HERO_CARD_LOC_ICON_CLASS: &str = "mr-3";
const HERO_CARD_SOCIAL_CLASS: &str = "mt-8 flex justify-center items-center flex-wrap";
const HERO_CARD_SOCIAL_ITEM_CLASS: &str = "inline-flex w-14 h-14 m-4 rounded-full items-center justify-center text-black bg-blue-500 shadow-lg transition-all duration-300 hover:bg-blue-200 hover:text-";
const HERO_CARD_SOCIAL_ITEM_IMG: &str = "hover:text-green";

const ICON_CHECK_CIRCLE: &str = "fa-check-circle";
const ICON_TWITTER: &str = "fa-brands fa-x-twitter";
const ICON_LOCATION: &str = "fa-map-marker-alt";
const ICON_GITHUB: &str = "fa-github";
const _ICON_LINK: &str = "fa-link";
const ICON_MEDIUM: &str = "fa-brands fa-medium";
const ICON_DEV: &str = "fa-brands fa-dev";
const ICON_LINKEDIN: &str = "fa-brands fa-linkedin";

#[func]
pub fn Hero(props: &ProfileCardProps) -> Html {
    html! {
        <section id="hero"><div class={WRAPPER_CLASS}>{ profile_content(props) }</div></section>
    }
}

fn profile_content(props: &ProfileCardProps) -> Html {
    html! {
        <div class={HERO_CARD_CLASS}>
            <div class={HERO_CARD_IMG_CLASS}>
                <img
                    alt="profile card"
                    src={props.image_src.clone()}
                    class={HERO_CARD_IMG_IMG_CLASS}
                />
            </div>
            <div class={HERO_CARD_CNT_CLASS}>
                { profile_name(&props.name, props.verified) }
                { profile_text(&props.bio) }
                { profile_location(&props.location) }
                { profile_social_links() }
            </div>
        </div>
    }
}

fn profile_name(name: &str, verified: bool) -> Html {
    html! {
        <div class={HERO_CARD_NAME_CLASS}>
            { name }
            { if verified {
                html! { <i class={format!("fa {} {}", ICON_CHECK_CIRCLE, HERO_CARD_CHECK_CIRCLE_CLASS)} title="Verified Rust Enjoyer"></i> }
            } else {
                html! {}
            } }
        </div>
    }
}

fn profile_text(bio: &str) -> Html {
    html! { <div class={HERO_CARD_TXT_CLASS}>{ bio }</div> }
}

fn profile_location(location: &str) -> Html {
    html! {
        <div class={HERO_CARD_LOC_CLASS}>
            <span class={HERO_CARD_LOC_ICON_CLASS}>
                <i class={format!("fa {} text-3xl", ICON_LOCATION)} />
            </span>
            <span>{ location }</span>
        </div>
    }
}

fn profile_social_links() -> Html {
    html! {
        <div class={HERO_CARD_SOCIAL_CLASS}>
            { social_link("https://www.twitter.com/wiseaidev", ICON_TWITTER) }
            { social_link("https://www.github.com/wiseaidev", ICON_GITHUB) }
            { social_link("https://wiseai.medium.com", ICON_MEDIUM) }
            { social_link("https://dev.to/wiseai", ICON_DEV) }
            { social_link("https://www.linkedin.com/in/mahmoud-harmouch", ICON_LINKEDIN) }
        </div>
    }
}

fn social_link(url: &'static str, icon: &str) -> Html {
    html! {
        <Link
            class={HERO_CARD_SOCIAL_ITEM_CLASS}
            to={url}
            target="_blank"
            rel="noreferrer"
        ><i class={format!("fa {} fa-2x {}", icon, HERO_CARD_SOCIAL_ITEM_IMG)} /></Link>
    }
}
