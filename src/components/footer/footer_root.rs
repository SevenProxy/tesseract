use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PropsFooterRoot {
  #[prop_or_default]
  pub children: Children,
}

pub struct FooterRoot;

impl Component for FooterRoot {
  type Message = ();
  type Properties = PropsFooterRoot;

  fn create(_: &Context<Self>) -> Self {
    FooterRoot
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    html! {
      <footer class="w-full pt-40  pb-10" id="contact">
        <div class="w-full absolute left-0 -bottom-72 min-h-96">
          <img src="/public/footer-grid.svg" alt="footer-img" class="w-full h-full opacity-50" />
        </div>
        { for ctx.props().children.iter() }
      </footer>
    }
  }
}