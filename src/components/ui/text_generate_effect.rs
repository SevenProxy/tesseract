use yew::prelude::*;

pub struct Text_generate_effect;

impl Component for Text_generate_effect {
  type Message = ();
  type Properties = ();

  fn create(_: Context<Self>) -> Self {
    Text_generate_effect
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