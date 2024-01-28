use next_rs::prelude::*;
use next_rs::Link;

const CONTAINER_CLASS: &str = "container mx-auto px-4";
const FOOTER_CLASS: &str = "footer py-10";
const BORDER_BOTTOM_CLASS: &str = "border-b";
const BOTTOM_CLASS: &str = "bottom text-white py-12";
const GRID_CLASS: &str = "grid grid-cols-12";
const COL_SPAN_12_CLASS: &str = "col-span-12";
const COL_SPAN_4_CLASS: &str = "col-span-4 lg:col-span-4";
const COL_SPAN_6_CLASS: &str = "col-span-12 md:col-span-6 lg:col-span-6 mt-4 md:mt-0";
const COL_SPAN_2_CLASS: &str = "col-span-12 md:col-span-2 lg:col-span-2 mt-4 md:mt-0";
const TEXT_GRAY_200_CLASS: &str = "text-gray-200 text-sm";
const WIDGET_NAV_MENU_CLASS: &str = "widget-nav-menu";
const SOCIALS_LIST_CLASS: &str = "socials-list text-center md:text-right pr-0 md:pr-12";
const CUSTOM_BG_DARK_CLASS: &str = "custom-bg-dark px-4 md:pl-4";

struct SocialLink {
    href: &'static str,
    icon_class: &'static str,
    title: &'static str,
}

fn copyright_section() -> Html {
    html! {
        <div class={format!("{} {}", COL_SPAN_12_CLASS, COL_SPAN_4_CLASS)}>
            <div class={TEXT_GRAY_200_CLASS}>{ "© Mahmoud Harmouch, 2024. All rights reserved." }</div>
        </div>
    }
}

fn nav_menu_section() -> Html {
    html! {
        <div class={format!("{} {}", COL_SPAN_12_CLASS, COL_SPAN_6_CLASS)}>
            <ul class={WIDGET_NAV_MENU_CLASS} />
        </div>
    }
}

fn socials_section() -> Html {
    let social_links: &[SocialLink] = &[
        SocialLink {
            href: "https://twitter.com/wiseaidev",
            icon_class: "fa-brands fa-x-twitter",
            title: "Twitter",
        },
        SocialLink {
            href: "https://github.com/wiseaidev",
            icon_class: "fa-brands fa-github",
            title: "Github",
        },
        SocialLink {
            href: "https://www.linkedin.com/in/mahmoud-harmouch",
            icon_class: "fa-brands fa-linkedin",
            title: "Linkedin",
        },
    ];
    html! {
        <div class={format!("{} {}", COL_SPAN_12_CLASS, COL_SPAN_2_CLASS)}>
            <div class={SOCIALS_LIST_CLASS}>
                { for social_links.iter().map(|link| {
                    html! {
                        <Link
                            to={link.href}
                            target="_blank"
                            rel="noreferrer"
                            class={CUSTOM_BG_DARK_CLASS}
                        >
                            <i class={link.icon_class} alt={link.title}></i>
                        </Link>
                    }
                }) }
            </div>
        </div>
    }
}

#[func]
pub fn Footer() -> Html {
    html! {
        <section id="footer-section" class={CONTAINER_CLASS}>
            <footer id="footer" class={FOOTER_CLASS}>
                <div id="footer-widget" class={BORDER_BOTTOM_CLASS} />
                <div id="bottom" class={BOTTOM_CLASS}>
                    <div class={GRID_CLASS}>
                        { copyright_section() }
                        { nav_menu_section() }
                        { socials_section() }
                    </div>
                </div>
            </footer>
        </section>
    }
}
