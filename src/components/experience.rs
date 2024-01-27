use next_rs::prelude::*;

const SECTION_CLASS: &str = "min-h-screen pt-24 pb-16";
const CONTAINER_CLASS: &str = "container mx-auto";
const TIMELINE_CLASS: &str =
    "timeline bg-black border-2 border-blue-500 shadow-md rounded-lg mx-5 p-8";
const LINE_CLASS: &str = "line";
const BOX_LIGHT_CLASS: &str =
    "custom-box rounded data-background p-8 text-left text-light shadow-blue";
const BOX_DARK_CLASS: &str = "custom-box rounded data-background p-8 text-left shadow-blue";
const TIME_CLASS: &str = "time text-gray-600";
const TITLE_CLASS: &str = "title text-xl font-bold";
const CONTENT_CLASS: &str = "text-white";
const SECTION_TITLE_CLASS: &str = "section-title lt-sp04 text-center text-white text-4xl font-bold";
const SPACER_CLASS: &str = "spacer";

pub struct TimelineData {
    pub id: u32,
    pub title: &'static str,
    pub years: &'static str,
    pub content: &'static str,
    pub color: &'static str,
    pub content_color: &'static str,
}

#[function_component(Experience)]
pub fn experience() -> Html {
    let education_data = vec![
        TimelineData {
            id: 1,
            title: "Coursera Certifications",
            years: "2020 - Present",
            content: "Over 100 Certifications Earned From the Coursera Platform.",
            color: "#000000",
            content_color: "light",
        },
        TimelineData {
            id: 2,
            title: "Diploma's Degree",
            years: "2015 - 2020",
            content:
                "A Diploma in Electrical and Telecommunication Engineering(~ Masters's Degree).",
            color: "#000000",
            content_color: "light",
        },
    ];

    let experience_data = vec![
        TimelineData {
            id: 1,
            title: "Self Employed",
            years: "2020 - Present",
            content: "Delivered full stack projects to multiple clients on time, and on budget.",
            color: "#000000",
            content_color: "light",
        },
        TimelineData {
            id: 2,
            title: "Final Year Project",
            years: "Mar. 2020 - Sep. 2020",
            content: "Thesis about Computer Vision Automated Control System Based On Zenoh'.",
            color: "#000000",
            content_color: "light",
        },
    ];

    html! {
        <section id="experience" class={SECTION_CLASS}>
            <div class={CONTAINER_CLASS}>
                { pagetitle("Education & Experience") }
                <div class="flex flex-wrap">
                    <div class="w-full md:w-1/2">
                        <div class={TIMELINE_CLASS}>
                            { for education_data.iter().map(|education| timeline(education, true)) }
                            <span class={LINE_CLASS} />
                        </div>
                    </div>
                    <div class="w-full md:w-1/2 md:pl-8 md:p-0 pt-5">
                        <div class={SPACER_CLASS} />
                        <div class={TIMELINE_CLASS}>
                            { for experience_data.iter().map(|experience| timeline(experience, false)) }
                            <span class={LINE_CLASS} />
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
}

pub fn timeline(education: &TimelineData, is_education: bool) -> Html {
    let box_classes = if education.content_color == "light" {
        BOX_LIGHT_CLASS
    } else {
        BOX_DARK_CLASS
    };

    html! {
        <div
            class={box_classes}
            style={format!(
                "background: {}; box-shadow: 0px 5px 20px 0px rgba({} , 0.5)",
                education.color,
                &education.color,
            )}
        >
            <div class="timeline-container">
                <div class="content justify-content-start">
                    <span class={TIME_CLASS}>
                        { if is_education {
                        html! { <span class="edu-icon">{"🎓 "}</span> }
                    } else {
                        html! { <span class="exp-icon">{"💼 "}</span> }
                    } }
                        { &education.years }
                    </span>
                    <h3 class={TITLE_CLASS}>{ &education.title }</h3>
                    <p class={CONTENT_CLASS}>{ &education.content }</p>
                </div>
            </div>
        </div>
    }
}

fn pagetitle(title: &str) -> Html {
    html! {
        <>
            <h1 class={SECTION_TITLE_CLASS}>{ title }</h1>
            <div class={SPACER_CLASS} style="height: 60px;" />
        </>
    }
}
