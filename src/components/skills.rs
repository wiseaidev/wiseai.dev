use crate::components::projects::pagetitle;
use next_rs::prelude::*;

const SECTION_CLASS: &str = "min-h-screen pt-24 p-11 container mx-auto";
const GRID_CLASS: &str =
    "text-center grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-x-12 gap-y-12";

#[derive(Clone, PartialEq, Properties)]
struct SkillData {
    id: u8,
    icon: &'static str,
    title: &'static str,
    content: &'static str,
    color: &'static str,
    content_color: &'static str,
}

fn skills_data() -> Vec<SkillData> {
    vec![
        SkillData {
            id: 1,
            icon: "👨‍💻",
            title: "Frontend Frameworks",
            content: "Yew.rs • Next.rs?",
            color: "#000000",
            content_color: "light",
        },
        SkillData {
            id: 2,
            icon: "⚙️",
            title: "Backend Frameworks",
            content: "Rocket • Hyper • Warp.",
            color: "#000000",
            content_color: "light",
        },
        SkillData {
            id: 3,
            icon: "🤖",
            title: "Machine Learning",
            content: "Smartcore-rs • OpenCV-rs • tch-rs.",
            color: "#000000",
            content_color: "light",
        },
        SkillData {
            id: 4,
            icon: "👨‍🏫",
            title: "Data Science",
            content: "Polars-rs • Plotters-rs • Ndarray-rs.",
            color: "#000000",
            content_color: "light",
        },
        SkillData {
            id: 5,
            icon: "♾️",
            title: "DevOps",
            content: "jetporch • Jenkins • Circle CI.",
            color: "#000000",
            content_color: "light",
        },
        SkillData {
            id: 6,
            icon: "🗃️",
            title: "DBMS",
            content: "Mysql • Redis • MongoDB.",
            color: "#000000",
            content_color: "light",
        },
    ]
}

#[func]
fn Skill(skill: &SkillData) -> Html {
    let box_class = if skill.content_color == "light" {
        "custom-box rounded bg-black text-white p-8 text-center shadow-lg border border-2 border-blue-500"
    } else {
        "custom-box rounded bg-black text-white p-8 text-center shadow-lg"
    };

    html! {
        <div
            class={box_class}
            data-color="#6C6CE5"
            style={format!("box-shadow: 0px 5px 20px 0px rgba({}, 0.5);", skill.color)}
        >
            <h2>{ skill.icon }</h2>
            <h3 class="mb-3 mt-0">{ skill.title }</h3>
            <p class="mb-4 mt-0">{ skill.content }</p>
        </div>
    }
}

#[func]
pub fn Skills() -> Html {
    html! {
        <section id="skills" class={SECTION_CLASS}>
            <div class="container">
                { pagetitle("Skills") }
                <div class={GRID_CLASS}>
                    { skills_data().into_iter().map(|skill| html! { <Skill ..skill /> }).collect::<Vec<_>>() }
                </div>
            </div>
        </section>
    }
}
