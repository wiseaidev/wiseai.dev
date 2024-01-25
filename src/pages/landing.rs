use crate::components::certifications::Certifications;
use crate::components::experience::Experience;
use crate::components::footer::Footer;
use crate::components::hero::Hero;
use crate::components::hero::ProfileCardProps;
use crate::components::projects::Projects;
use crate::components::skills::Skills;
use crate::components::trending::Trending;
use std::collections::HashMap;
use yew::prelude::*;
use yew_i18n::I18nProvider;
use yew_navbar::{Menu, Navbar};
use yew_scroll::ScrollToTop;

#[function_component(LandingPage)]
pub fn landing_page() -> Html {
    let profile_props = ProfileCardProps {
        name: "Mahmoud Harmouch".to_string().into(),
        verified: true,
        bio: "🦀 Seasoned Rust Developer 🦀".to_string().into(),
        location: "Tripoly, Lebanon".to_string().into(),
        image_src: "images/pic.png".to_string().into(),
    };

    let mut translations = HashMap::new();

    translations.insert(
        "en".to_string(),
        serde_json::json!({
            "24 Apr, 2023": "24 Apr, 2023",
            "02 May, 2023": "02 May, 2023",
            "11 May, 2023": "11 May, 2023",
            "Trending Posts": "Trending Posts",
            "Rust: The Next Big Thing in Data Science": "Rust: The Next Big Thing in Data Science",
            "Data Science": "Data Science",
            "The Ultimate Ndarray Handbook: Mastering the Art of Scientific Computing with Rust": "The Ultimate Ndarray Handbook: Mastering the Art of Scientific Computing with Rust",
            "Rust Polars: Unlocking High-Performance Data Analysis — Part 1": "Rust Polars: Unlocking High-Performance Data Analysis — Part 1",
        }),
    );

    translations.insert(
        "fr".to_string(),
        serde_json::json!({
            "24 Apr, 2023": "24 Avr, 2023",
            "02 May, 2023": "02 Mai, 2023",
            "11 May, 2023": "11 Mai, 2023",
            "Trending Posts": "Articles Tendances",
            "Rust: The Next Big Thing in Data Science": "Rust : La Prochaine Grande Avancée en Science des Données",
            "Data Science": "Science des Données",
            "The Ultimate Ndarray Handbook: Mastering the Art of Scientific Computing with Rust": "Le Manuel Ultime Ndarray : Maîtriser l'Art du Calcul Scientifique avec Rust",
            "Rust Polars: Unlocking High-Performance Data Analysis — Part 1": "Rust Polars : Libérer l'Analyse de Données Haute Performance — Partie 1",
        }),
    );

    translations.insert(
        "de".to_string(),
        serde_json::json!({
            "24 Apr, 2023": "24 Apr, 2023",
            "02 May, 2023": "02 Mai, 2023",
            "11 May, 2023": "11 Mai, 2023",
            "Trending Posts": "Trending Beiträge",
            "Rust: The Next Big Thing in Data Science": "Rust: Die nächste große Sache in der Datenwissenschaft",
            "Data Science": "Datenwissenschaft",
            "The Ultimate Ndarray Handbook: Mastering the Art of Scientific Computing with Rust": "Das ultimative Ndarray-Handbuch: Die Kunst des wissenschaftlichen Rechnens mit Rust meistern",
            "Rust Polars: Unlocking High-Performance Data Analysis — Part 1": "Rust Polars: Freischalten der High-Performance-Datenanalyse — Teil 1",
        }),
    );

    translations.insert(
        "es".to_string(),
        serde_json::json!({
            "24 Apr, 2023": "24 Abr, 2023",
            "02 May, 2023": "02 May, 2023",
            "11 May, 2023": "11 May, 2023",
            "Trending Posts": "Publicaciones Destacadas",
            "Rust: The Next Big Thing in Data Science": "Rust: La Próxima Gran Novedad en Ciencia de Datos",
            "Data Science": "Ciencia de Datos",
            "The Ultimate Ndarray Handbook: Mastering the Art of Scientific Computing with Rust": "El Manual Definitivo de Ndarray: Dominando el Arte de la Computación Científica con Rust",
            "Rust Polars: Unlocking High-Performance Data Analysis — Part 1": "Rust Polars: Desbloqueando el Análisis de Datos de Alto Rendimiento — Parte 1",
        }),
    );

    let menus: Vec<Menu> = vec![
        Menu {
            id: 1,
            link: "#blog",
            name: "Blog",
        },
        Menu {
            id: 2,
            link: "#skills",
            name: "Skills",
        },
        Menu {
            id: 3,
            link: "#experience",
            name: "Experience",
        },
        Menu {
            id: 4,
            link: "#projects",
            name: "Projects",
        },
        Menu {
            id: 5,
            link: "#certifications",
            name: "Certifications",
        },
    ];

    html! {
        <I18nProvider supported_languages={vec!["en", "fr", "de", "es"]} translations={translations} >
          <Navbar menus={menus} button_text="HireMe" button_href="mailto:business@wiseai.dev" />
          <Hero ..profile_props />
          <Trending />
          <Skills />
          <Experience />
          <Projects />
          <Certifications />
          <Footer />
          <ScrollToTop />
        </I18nProvider>
    }
}
