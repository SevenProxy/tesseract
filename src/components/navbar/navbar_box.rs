use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PropsNavbarBox {
    #[prop_or_default]
    pub children: Children,
}

pub struct NavbarBox;

impl Component for NavbarBox {
    type Message = ();
    type Properties = PropsNavbarBox;

    fn create(_: &Context<Self>) -> Self {
        NavbarBox
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
          <div class="relative bg-black-100 flex justify-center items-center  flex-col overflow-hidden mx-auto sm:px-10 px-5">
            <div class="max-w-7xl w-full">
              {for ctx.props().children.iter()}
            </div>
          </div>
        }
    }
}
