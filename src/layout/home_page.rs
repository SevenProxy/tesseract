use yew::prelude::*;
use crate::components::navbar::{NavbarRoot, NavbarBox, NavbarContext};
use crate::components::footer::{FooterRoot, FooterContext, FooterInformation};


#[derive(Properties, PartialEq)]
pub struct PropsHomePage {
  #[prop_or_default]
  pub children: Children,
}

pub struct HomePage;

impl Component for HomePage {
  type Message = ();
  type Properties = PropsHomePage;

  fn create(_: &Context<Self>) -> Self {
    HomePage
  }


  fn view(&self, ctx: &Context<Self>) -> Html {
    html! {
      <main class="relative bg-black-100 flex justify-center items-center 
    flex-col overflow-hidden mx-auto sm:px-10 px-5">
        <div class="max-w-7xl w-full">
          <div class="w-full flex items-center justify-center">
            <NavbarRoot>
              <NavbarBox>
                <NavbarContext />
              </NavbarBox>
            </NavbarRoot>
          </div>
          { for ctx.props().children.iter() }
          <FooterRoot>
            <FooterContext />
            <FooterInformation />
          </FooterRoot>
        </div>
      </main>
    }
  }

}