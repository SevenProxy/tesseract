use yew::prelude::*;
mod components;
mod utils;

use components::navbar::{NavbarRoot, NavbarBox, NavbarContext};
use components::hero::{HeroRoot, HeroContext, HeroBox};
use components::grid::{GridRoot, GridBoxAbout, GridBoxHour, GridBoxLang, GridBoxDevWeb, GridBoxCourse, GridBoxEmail};
use components::projects::{ProjectsCard, ProjectsContainer, ProjectsRoot};
use components::know_ledge::{KnowRoot, KnowContainer, KnowBoxCourse, KnowBoxTech, KnowCard};
use components::languagens::{LanguagensRoot, LanguagensContext, LanguagensCardFront, LanguagensCardBack, LanguagensCardTools};
use components::footer::{FooterRoot, FooterContext, FooterInformation};
use utils::{FETCH_COURSES, FETCH_PROJECTS};

#[function_component(App)]
fn app() -> Html {

    html! {
        <main class="relative bg-black-100 flex justify-center items-center 
    flex-col overflow-hidden mx-auto sm:px-10 px-5">
            <div class="max-w-7xl w-full">
                <div class="w-full flex items-center justify-center">
                    <NavbarRoot>
                        <NavbarBox>
                            <NavbarContext />
                        </NavbarBox>
                    </NavbarRoot>
                </div>
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
                <LanguagensRoot>
                    <LanguagensContext>
                        <LanguagensCardFront />
                        <LanguagensCardBack />
                        <LanguagensCardTools />
                    </LanguagensContext>
                </LanguagensRoot>
                <FooterRoot>
                    <FooterContext />
                    <FooterInformation />
                </FooterRoot>
            </div>
        </main>
    }
}

#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn run_app() {
    yew::Renderer::<App>::new().render();
}
