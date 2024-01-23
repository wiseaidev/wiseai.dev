use crate::components::hero::Hero;
use crate::components::header::Header;
use crate::components::hero::ProfileCardProps;
use crate::components::experience::Experience;
use crate::components::projects::Projects;
use crate::components::certifications::Certifications;
use crate::components::trending::Trending;
use crate::components::footer::Footer;
use crate::components::skills::Skills;
use yew_scroll::ScrollToTop;
use yew::prelude::*;

#[function_component(LandingPage)]
pub fn landing_page() -> Html {
    let profile_props = ProfileCardProps {
        name: "Mahmoud Harmouch".to_string().into(),
        verified: true,
        bio: "🦀 Seasoned Rust Developer 🦀".to_string().into(),
        location: "Tripoly, Lebanon".to_string().into(),
        image_src: "images/pic.png".to_string().into(),
    };

    html! {
      <>
        <Header />
        <Hero ..profile_props />
        <Trending />
        <Skills />
        <Experience />
        <Projects />
        <Certifications />
        <Footer />
        <ScrollToTop />
      </>
      }
}