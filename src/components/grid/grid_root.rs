use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PropsGridRoot {
  #[prop_or_default]
  pub children: Children,
}

pub struct GridRoot;

impl Component for GridRoot {
    type Message = ();
    type Properties = PropsGridRoot;

    fn create(_: &Context<Self>) -> Self {
        GridRoot
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
          <section id="about" class="mt-8">
            <div class="grid grid-cols-1 md:grid-cols-6 lg:grid-cols-5 md:grid-row-7 gap-4 lg:gap-8 mx-auto">
              { for ctx.props().children.iter() }
            </div>
          </section>
        }
    }
}
