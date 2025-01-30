use yew::prelude::*;

pub struct HeroRoot;

impl Component for HeroRoot {
  type Message = ();
  type Properties = ();

  fn create(_: &Context<Self>) -> Self {
    HeroRoot
  }

  fn view(&self, _: &Context<Self>) -> Html {
    html! {
      <div>
      </div>
    }
  }

}