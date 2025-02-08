use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PropsKnowContainer {
  #[prop_or_default]
  pub children: Children,
}

pub struct KnowContainer;

impl Component for KnowContainer {
  type Message = ();
  type Properties = PropsKnowContainer;

  fn create(_: &Context<Self>) -> Self {
    KnowContainer
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    html! {
      <div class="flex flex-col items-center max-lg:mt-10">
        { for ctx.props().children.iter() }
      </div>
    }
  }

}