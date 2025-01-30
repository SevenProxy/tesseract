use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PropsHeroRoot {
  #[prop_or_default]
  pub children: Children,
}

pub struct HeroRoot;

impl Component for HeroRoot {
  type Message = ();
  type Properties = PropsHeroRoot;

  fn create(_: &Context<Self>) -> Self {
    HeroRoot
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    html! {
      <div class="pb-20 pt-36" id="home">
        <div>
          { for ctx.props().children.iter() }
        </div>
      </div>
    }
  }

}