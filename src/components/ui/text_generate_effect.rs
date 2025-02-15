use yew::prelude::*;

pub struct TextGenerateEffect;

impl Component for TextGenerateEffect {
  type Message = ();
  type Properties = ();

  fn create(_: &Context<Self>) -> Self {
    TextGenerateEffect
  }

  fn view(&self, _: &Context<Self>) -> Html {
    html! {
      <div class="font-bold text-center text-[40px] md:text-5x6 lg:text-6xl">
        <div class="my-5">
          <div class="text-white sleading-snug tracking-wide">
            <div>
              <span class="text-white opacity-0" style="opacity: 1;">{"Transformando "}</span>
              <span class="text-white opacity-0" style="opacity: 1;">{"a "}</span>
              <span class="text-white opacity-0" style="opacity: 1;">{"Experiência " }</span>
              <span class="text-white opacity-0" style="opacity: 1;">{"do "}</span>
              <span class="text-white opacity-0" style="opacity: 1;">{"Usuário "}</span>
              <span class="text-white opacity-0" style="opacity: 1;">{"com "}</span>
              <span class="text-white opacity-0" style="opacity: 1;">{"Conceitos " }</span>
              <span class="bg-gradient-to-r from-blue-600 to-purple bg-clip-text text-transparent">
                {"Modernos"}
              </span>
            </div>
          </div>
        </div>
      </div>
    }
  }
}