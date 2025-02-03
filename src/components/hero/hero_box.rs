use crate::components::ui::{TextGenerateEffect, MagicButton};
use yew::prelude::*;

pub struct HeroBox;

impl Component for HeroBox {
  type Message = ();
  type Properties = ();

  fn create(_: &Context<Self>) -> Self {
    HeroBox
  }

  fn view(&self, _: &Context<Self>) -> Html {
    html! {
      <div class="flex justify-center relative my-20 z-10">
        <div class="max-w-[89vw] md:max-w-2xl lg:max-w-[60vw] flex flex-col items-center justify-center">
          <h2 class="uppercase tracking-widest text-xs text-center text-blue-200 max-w-80">
            {"Projeto Dinâmico, Criado com a Magia do yew, Rust, Tailwindcss e WebAssembly"}
          </h2>

          <TextGenerateEffect />

          <p class="text-center tracking-widest md:tracking-wider mb-4 text-lg md:text-lg lg:text-lg text-white">
            {"Futuro Engenheiro de Software"}
          </p>

          <a
            href="https://drive.google.com/file/d/1EwdAdUczqlKzA_lNizax58Dpkby_QyvG/view?usp=sharing"
            target="_blank"
          >
            <MagicButton>
              {"Download CV"}
              <svg stroke="currentColor" fill="currentColor" stroke-width="0" viewBox="0 0 576 512" height="1em" width="1em" xmlns="http://www.w3.org/2000/svg">
                <path d="M160 448c-25.6 0-51.2-22.4-64-32-64-44.8-83.2-60.8-96-70.4V480c0 17.67 14.33 32 32 32h256c17.67 0 32-14.33 32-32V345.6c-12.8 9.6-32 25.6-96 70.4-12.8 9.6-38.4 32-64 32zm128-192H32c-17.67 0-32 14.33-32 32v16c25.6 19.2 22.4 19.2 115.2 86.4 9.6 6.4 28.8 25.6 44.8 25.6s35.2-19.2 44.8-22.4c92.8-67.2 89.6-67.2 115.2-86.4V288c0-17.67-14.33-32-32-32zm256-96H224c-17.67 0-32 14.33-32 32v32h96c33.21 0 60.59 25.42 63.71 57.82l.29-.22V416h192c17.67 0 32-14.33 32-32V192c0-17.67-14.33-32-32-32zm-32 128h-64v-64h64v64zm-352-96c0-35.29 28.71-64 64-64h224V32c0-17.67-14.33-32-32-32H96C78.33 0 64 14.33 64 32v192h96v-32z"></path>
              </svg>
            </MagicButton>
          </a>
        </div>
      </div>
    }
  }

}