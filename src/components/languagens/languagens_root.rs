use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PropsLanguagensRoot {
  #[prop_or_default]
  pub children: Children,
}

pub struct LanguagensRoot;

impl Component for LanguagensRoot {
  type Message = ();
  type Properties = PropsLanguagensRoot;

  fn create(_: &Context<Self>) -> Self {
    LanguagensRoot
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    html! {
      <div>
        <h1 class="heading" id="knowledge">
          <span class="text-white">{"Tecnologias "}</span>
          <span class="bg-gradient-to-r from-blue-600 to-purple bg-clip-text text-transparent">{"Favoritas"}</span>
        </h1>

        { for ctx.props().children.iter() }
      </div>
    }
  }

}