use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PropsLanguagensContext {
  #[prop_or_default]
  pub children: Children,
}

pub struct LanguagensContext;

impl Component for LanguagensContext {
  type Message = ();
  type Properties = PropsLanguagensContext;

  fn create(_: &Context<Self>) -> Self {
    LanguagensContext
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    html! {
      <div class="my-20">
        <div class="my-20">
          <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-1 lg:grid-cols-3 gap-x-6 gap-y-12 md:gap-16">
            { for ctx.props().children.iter() }
          </div>
        </div>
      </div>
    }
  }

}