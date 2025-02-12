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
      <div class="pb-20 pt-36 relative" id="home">
        <div class="w-full absolute left-0 -top-20 z-10 min-h-96">
          <img src="/public/footer-grid.svg" alt="footer-img" class="w-full h-full opacity-50" />
        </div>
        <div>
          { for ctx.props().children.iter() }
        </div>
      </div>
    }
  }

}