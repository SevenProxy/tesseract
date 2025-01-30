use yew::prelude::*;

pub struct TextGenerateEffect;

impl Component for TextGenerateEffect {
  type Message = ();
  type Properties = ();

  fn create(_: Context<Self>) -> Self {
    TextGenerateEffect
  }

  fn view(&self, _: Context<Self>) -> Html {
    html! {
    <div class="font-bold">
      <div class="my-5">
        <div class="text-white sleading-snug tracking-wide">
          <span class="bg-gradient-to-r from-blue-600 to-purple bg-clip-text text-transparent"></span>
        </div>
      </div>
    </div>
    }
  }
}