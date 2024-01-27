use crate::components::projects::pagetitle;
use next_rs::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct CertificationData {
    #[prop_or_default]
    pub id: usize,
    #[prop_or_default]
    pub title: &'static str,
    #[prop_or_default]
    pub category: &'static str,
    #[prop_or_default]
    pub image: &'static str,
    #[prop_or_default]
    pub link: &'static str,
}

const CONTAINER_CLASS: &str = "min-h-screen pt-24 pb-16 container mx-auto";
const GRID_CLASS: &str =
    "text-center grid grid-cols-1 sm:grid-cols-3 md:grid-cols-3 gap-x-12 gap-y-12";
const GRID_ITEM_CLASS: &str = "grid-item group";
const FLEX_CENTER_CLASS: &str = "flex justify-center mt-4";
const BUTTON_CLASS: &str =
    "rounded-full py-2 px-6 bg-blue-500 text-white text-lg transition-colors hover:bg-blue-600";
const GRID_ITEM_WRAPPER_CLASS: &str = "bg-black shadow-dark rounded group w-2/3 ml-16";
const CERTIFICATION_IMAGE_CLASS: &str = "certification-image";
const CERTIFICATION_ITEM_CLASS: &str =
    "certification-item rounded shadow-dark border border-primary relative overflow-hidden";
const DETAILS_CLASS: &str = "details absolute top-0 left-0 opacity-0 group-hover:opacity-90 bg-black transition-all duration-500 w-full h-full";
const TERM_CLASS: &str =
    "term text-blue-900 bg-white rounded-bl-lg rounded-br-lg text-sm px-4 py-2";
const TITLE_CLASS: &str = "title text-white text-xl pt-10";
const MORE_BUTTON_CLASS: &str = "more-button text-white text-2xl";
const MASK_CLASS: &str = "mask";

#[function_component(Certifications)]
pub fn certifications() -> Html {
    let visible_items: &[CertificationData] = &[
        CertificationData {
            id: 1,
            title: "Google IT Automation Certificate",
            category: "Python",
            image: "https://images.credly.com/size/340x340/images/efbdc0d6-b46e-4e3c-8cf8-2314d8a5b971/GCC_badge_python_1000x1000.png",
            link: "https://www.credly.com/badges/a45a4aab-82ce-4621-86e5-3e5fe2f6fb58",
        },
        CertificationData {
            id: 2,
            title: "Open Source Software Development, Linux and Git Specialization",
            category: "Git",
            image: "https://images.credly.com/size/340x340/images/a8e890b4-d484-4e04-b521-fba516a8c3cd/coursera-specialization-badge.png",
            link: "https://www.credly.com/badges/cf74adc4-f37f-4dc5-8741-db8e9a66067b",
        },
        CertificationData {
            id: 3,
            title: "Google IT Support Professional Certificate",
            category: "IT",
            link: "https://www.credly.com/badges/eace30f1-b663-4248-8a61-14ee4cc63f50",
            image: "https://images.credly.com/size/340x340/images/ae2f5bae-b110-4ea1-8e26-77cf5f76c81e/GCC_badge_IT_Support_1000x1000.png",
        },
        CertificationData {
            id: 4,
            title: "Google IT Automation with Python",
            category: "Python",
            link: "https://coursera.org/share/d234914ee7fb0dfab0bfb5d48219493f",
            image: "https://s3.amazonaws.com/coursera_assets/meta_images/generated/CERTIFICATE_LANDING_PAGE/CERTIFICATE_LANDING_PAGE~JK6EAGFRVTPS/CERTIFICATE_LANDING_PAGE~JK6EAGFRVTPS.jpeg",
        },
        CertificationData {
            id: 5,
            title: "Introduction to Git and GitHub",
            category: "Git",
            link: "https://coursera.org/share/06f9a0e105ab0e4b6c3285ea7323980a",
            image: "https://s3.amazonaws.com/coursera_assets/meta_images/generated/CERTIFICATE_LANDING_PAGE/CERTIFICATE_LANDING_PAGE~8GCLBDG62Q9J/CERTIFICATE_LANDING_PAGE~8GCLBDG62Q9J.jpeg",
        },
        CertificationData {
            id: 6,
            title: "Crash Course on Python",
            category: "Python",
            link: "https://coursera.org/share/229d14d379e81aaeea39e78ddf21ba8a",
            image: "https://s3.amazonaws.com/coursera_assets/meta_images/generated/CERTIFICATE_LANDING_PAGE/CERTIFICATE_LANDING_PAGE~WTUWSGL2ZAWA/CERTIFICATE_LANDING_PAGE~WTUWSGL2ZAWA.jpeg",
        },
        CertificationData {
            id: 7,
            title: "Troubleshooting and Debugging Techniques",
            category: "Python",
            link: "https://coursera.org/share/4a230d92b0d1239a2e0fadcd05339508",
            image: "https://s3.amazonaws.com/coursera_assets/meta_images/generated/CERTIFICATE_LANDING_PAGE/CERTIFICATE_LANDING_PAGE~MTQ6HHL3FARV/CERTIFICATE_LANDING_PAGE~MTQ6HHL3FARV.jpeg",
        },
        CertificationData {
            id: 8,
            title: "Automating Real-World Tasks with Python",
            category: "Python",
            link: "https://coursera.org/share/b515301d8c054c962137e26a9cec20ae",
            image: "https://s3.amazonaws.com/coursera_assets/meta_images/generated/CERTIFICATE_LANDING_PAGE/CERTIFICATE_LANDING_PAGE~5B7BJ3WQXGAK/CERTIFICATE_LANDING_PAGE~5B7BJ3WQXGAK.jpeg",
        },
        CertificationData {
            id: 9,
            title: "Configuration Management and the Cloud",
            category: "Python",
            link: "https://coursera.org/share/6355e3b760f7dd9ba1b9b9561fa9b774",
            image: "https://s3.amazonaws.com/coursera_assets/meta_images/generated/CERTIFICATE_LANDING_PAGE/CERTIFICATE_LANDING_PAGE~CWJ4HEV447MX/CERTIFICATE_LANDING_PAGE~CWJ4HEV447MX.jpeg",
        },
        CertificationData {
            id: 10,
            title: "Using Python to Interact with the Operating System",
            category: "Python",
            link: "https://coursera.org/share/6355e3b760f7dd9ba1b9b9561fa9b774",
            image: "https://s3.amazonaws.com/coursera_assets/meta_images/generated/CERTIFICATE_LANDING_PAGE/CERTIFICATE_LANDING_PAGE~CWJ4HEV447MX/CERTIFICATE_LANDING_PAGE~CWJ4HEV447MX.jpeg",
        },
        CertificationData {
            id: 11,
            title: "Open Source Software Development, Linux and Git",
            category: "Linux/Git",
            link: "https://coursera.org/share/d1a371d47a4e1fa1c836c068bef12370",
            image: "https://s3.amazonaws.com/coursera_assets/meta_images/generated/CERTIFICATE_LANDING_PAGE/CERTIFICATE_LANDING_PAGE~92JPZRK3JAXC/CERTIFICATE_LANDING_PAGE~92JPZRK3JAXC.jpeg",
        },
        CertificationData {
            id: 12,
            title: "Open Source Software Development Methods",
            category: "Linux/Git",
            link: "https://coursera.org/share/f1b7901e067ac18d96bf3f2b9830b685",
            image: "https://s3.amazonaws.com/coursera_assets/meta_images/generated/CERTIFICATE_LANDING_PAGE/CERTIFICATE_LANDING_PAGE~5VXM96KHX8J8/CERTIFICATE_LANDING_PAGE~5VXM96KHX8J8.jpeg",
        },
        CertificationData {
            id: 13,
            title: "Linux for Developers",
            category: "Linux/Git",
            link: "https://coursera.org/share/f1b7901e067ac18d96bf3f2b9830b685",
            image: "https://s3.amazonaws.com/coursera_assets/meta_images/generated/CERTIFICATE_LANDING_PAGE/CERTIFICATE_LANDING_PAGE~5VXM96KHX8J8/CERTIFICATE_LANDING_PAGE~5VXM96KHX8J8.jpeg",
        },
        CertificationData {
            id: 14,
            title: "Linux for Developers",
            category: "Linux/Git",
            link: "https://coursera.org/share/8cce8e8886f6f370f4ffac4238e93bef",
            image: "https://s3.amazonaws.com/coursera_assets/meta_images/generated/CERTIFICATE_LANDING_PAGE/CERTIFICATE_LANDING_PAGE~4FHGKPH6WNHP/CERTIFICATE_LANDING_PAGE~4FHGKPH6WNHP.jpeg",
        },
        CertificationData {
            id: 15,
            title: "Linux Tools for Developers",
            category: "Linux/Git",
            link: "https://coursera.org/share/60654ea285bd6a95559ae3b5b48b485d",
            image: "https://s3.amazonaws.com/coursera_assets/meta_images/generated/CERTIFICATE_LANDING_PAGE/CERTIFICATE_LANDING_PAGE~WSYWDZDMPLNB/CERTIFICATE_LANDING_PAGE~WSYWDZDMPLNB.jpeg",
        },
        CertificationData {
            id: 16,
            title: "Using Git for Distributed Development",
            category: "Linux/Git",
            link: "https://coursera.org/share/25f32445bf7573e8948feb9400062483",
            image: "https://s3.amazonaws.com/coursera_assets/meta_images/generated/CERTIFICATE_LANDING_PAGE/CERTIFICATE_LANDING_PAGE~J3VXT7CB7XSX/CERTIFICATE_LANDING_PAGE~J3VXT7CB7XSX.jpeg",
        },
    ];
    // TODO: fix these buttons
    let no_more_post_handle = use_state(|| false);
    let no_more_post = *no_more_post_handle;

    let no_more_hide_handle = use_state(|| false);
    let no_more_hide = *no_more_hide_handle;

    let data_visible_count_handle = use_state(|| 3);
    let _data_visible_count = *data_visible_count_handle;

    let handle_hidemore = {
        let no_more_hide_handle = no_more_hide_handle.clone();
        Callback::from(move |_| {
            no_more_hide_handle.set(!no_more_hide);
        })
    };

    let handle_loadmore = {
        let no_more_post_handle = no_more_post_handle.clone();
        Callback::from(move |_| {
            no_more_post_handle.set(!no_more_post);
        })
    };

    html! {
        <section id="certifications" class={CONTAINER_CLASS}>
            <div class={CONTAINER_CLASS}>
                { pagetitle("Certifications") }
                <div class={GRID_CLASS}>
                    { for visible_items.iter().map(|item| {
                        html! {
                            <div class={GRID_ITEM_CLASS} key={item.id}>
                                <Certification ..item.clone() />
                            </div>
                        }
                    }) }
                </div>
                <div class={FLEX_CENTER_CLASS}>
                    <button class={format!("{} mr-4", BUTTON_CLASS)} onclick={handle_hidemore}>
                        { if no_more_hide { "Can't Hide" } else { "Hide" } }
                    </button>
                    <button class={BUTTON_CLASS} onclick={handle_loadmore}>
                        { if no_more_post { "No More Items" } else { "Show More" } }
                    </button>
                </div>
            </div>
        </section>
    }
}

#[function_component(Certification)]
pub fn certification(certification: &CertificationData) -> Html {
    let toggler = use_state(|| false);

    let handle_lightbox = {
        let _no_more_post_handle = certification.link;
        Callback::from(move |_| {
            toggler.set(!*toggler);
        })
    };

    html! {
        <div class={GRID_ITEM_WRAPPER_CLASS}>
            <a
                href={certification.link}
                class={CERTIFICATION_IMAGE_CLASS}
                onclick={handle_lightbox}
                target="_blank"
                rel="noreferrer"
            >
                <div class={CERTIFICATION_ITEM_CLASS}>
                    <div class={DETAILS_CLASS}>
                        <span class={TERM_CLASS}>{ &certification.category }</span>
                        <h4 class={TITLE_CLASS}>{ &certification.title }</h4>
                        <span class={MORE_BUTTON_CLASS}><i class="fa-solid fa-link text-primary" /></span>
                    </div>
                    <div class={MASK_CLASS} />
                    <div class="thumb">
                        <img src={&*certification.image} alt="Certification-title" loading="lazy" />
                        <div class={MASK_CLASS} />
                    </div>
                </div>
            </a>
        </div>
    }
}
