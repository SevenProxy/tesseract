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
    <div class="text-center text-[40px] md:text-5x6 lg:text-6xl"> 
      <div class="font-bold">
        <div class="my-5">
          <div class="text-white sleading-snug tracking-wide">
            <span class="bg-gradient-to-r from-blue-600 to-purple bg-clip-text text-transparent">
              {"Transformando a Experiência do Usuário com Conceitos Modernos"}
            </span>
          </div>
        </div>
      </div>
    </div>
    }
  }
}