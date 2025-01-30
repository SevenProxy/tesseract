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
            {"Projeto Dinâmico, Criado com a Magia do Next.js, Aceternity e Shadcn
            UI"}
          </h2>

          <TextGenerateEffect />

          <p class="text-center tracking-widest md:tracking-wider mb-4 text-lg md:text-lg lg:text-lg text-white">
            {"Futuro Desenvolvedor Full Stack"}
          </p>

          <a
            href="https://drive.google.com/file/d/1EwdAdUczqlKzA_lNizax58Dpkby_QyvG/view?usp=sharing"
            target="_blank"
          >
            <MagicButton />
          </a>
        </div>
      </div>
    }
  }

}