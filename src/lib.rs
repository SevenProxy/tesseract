use yew::prelude::*;
mod components;

use components::navbar::{NavbarRoot, NavbarBox, NavbarContext};
use components::hero::{HeroRoot, HeroContext, HeroBox};
use components::grid::GridRoot;

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
                <GridRoot />
            </div>
        </main>
    }
}

#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn run_app() {
    yew::Renderer::<App>::new().render();
}
