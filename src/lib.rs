use yew::prelude::*;
mod components;
mod utils;
mod layout;

use components::hero::{HeroRoot, HeroContext, HeroBox};
use components::grid::{GridRoot, GridBoxAbout, GridBoxHour, GridBoxLang, GridBoxDevWeb, GridBoxCourse, GridBoxEmail};
use components::projects::{ProjectsCard, ProjectsContainer, ProjectsRoot};
use components::know_ledge::{KnowRoot, KnowContainer, KnowBoxCourse, KnowBoxTech, KnowCard};
use components::certificates::{CertificatesRoot, CertificatesGrid, CertificatesCard};
use components::languagens::{LanguagensRoot, LanguagensContext, LanguagensCardFront, LanguagensCardBack, LanguagensCardTools};
use layout::HomePage;
use utils::{FETCH_COURSES, FETCH_PROJECTS, FETCH_CERTIFICATES};

#[function_component(App)]
fn app() -> Html {

    html! {
        <HomePage>
            <HeroRoot>
                <HeroContext/>
                <HeroBox />
            </HeroRoot>
            <GridRoot>
                <GridBoxAbout />
                <GridBoxHour />
                <GridBoxLang />
                <GridBoxDevWeb />
                <GridBoxCourse />
                <GridBoxEmail />
            </GridRoot>
            <ProjectsRoot>
                <ProjectsContainer>
                    { for FETCH_PROJECTS.iter().map(|p| html! {
                        <ProjectsCard title={p.title.clone()} description={p.description.clone()} banner={p.banner.clone()} url={p.url.clone()} icon_tech={p.icon_tech.clone()} />
                    }) }
                </ProjectsContainer>
            </ProjectsRoot>
            <KnowRoot>
                <KnowContainer>
                    <KnowBoxCourse>
                        { for FETCH_COURSES.iter().map(|c| html! {
                            <KnowCard title={c.title.clone()} name={c.name.clone()} description={c.description.clone()} icon={c.icon.clone()} />
                        }) }
                    </KnowBoxCourse>
                    <KnowBoxTech />
                </KnowContainer>
            </KnowRoot>
            <CertificatesRoot>
                <CertificatesGrid>
                    { for FETCH_CERTIFICATES.iter().map(|c| html! {
                        <CertificatesCard title={c.title.clone()} description={c.description.clone()} icon={c.icon.clone()} url={c.url.clone()} />
                    })}
                </CertificatesGrid>
            </CertificatesRoot>
            <LanguagensRoot>
                <LanguagensContext>
                    <LanguagensCardFront />
                    <LanguagensCardBack />
                    <LanguagensCardTools />
                </LanguagensContext>
            </LanguagensRoot>
        </HomePage>
    }
}

#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn run_app() {
    yew::Renderer::<App>::new().render();
}
