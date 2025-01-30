use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PropsNavbarRoot {
  #[prop_or_default]
  pub children: Children,
}

pub struct  NavbarRoot;

impl Component for NavbarRoot {
  type Message = ();
  type Properties = PropsNavbarRoot;

  fn create(_: &Context<Self>) -> Self {
      NavbarRoot
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    html! {
      { for ctx.props().children.iter() }
    }
  }
}