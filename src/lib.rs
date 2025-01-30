use yew::prelude::*;
mod components;

use components::navbar::{NavbarRoot, NavbarBox, NavbarContext};
use components::hero::{HeroRoot, HeroContext, HeroBox};

#[function_component(App)]
fn app() -> Html {
    html! {
        <main class="w-full py-6 fixed top-4 left-0">
            <div class="w-full flex items-center justify-center">
                <NavbarRoot>
                    <NavbarBox>
                        <NavbarContext />
                    </NavbarBox>
                </NavbarRoot>
            </div>
            <HeroRoot>
                {"<!--<div> <Spotlight /></div>->"}
                <HeroContext/>
                <HeroBox />
            </HeroRoot>
        </main>
    }
}

#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn run_app() {
    yew::Renderer::<App>::new().render();
}
