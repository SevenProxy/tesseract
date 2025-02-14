use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PropsKnowRoot {
    #[prop_or_default]
    pub children: Children,
}

pub struct KnowRoot;

impl Component for KnowRoot {
  type Message = ();
  type Properties = PropsKnowRoot;

  fn create(_: &Context<Self>) -> Self {
    KnowRoot
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    html! {
      <div>
        <h1 class="heading" id="knowledge">
            <span class="text-white">{"Educação e"}</span> 
            <span class="bg-gradient-to-r from-blue-600 to-purple bg-clip-text text-transparent">{" Capacitações"}</span>
        </h1>
        { for ctx.props().children.iter() }
      </div>
    }
  }
}
