use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PropsCertificatesGrid {
  #[prop_or_default]
  pub children: Children,
}

pub struct CertificatesGrid;

impl Component for CertificatesGrid {
  type Message = ();
  type Properties = PropsCertificatesGrid;

  fn create(_: &Context<Self>) -> Self {
    CertificatesGrid
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    html! {
      <div class="mt-12 gap-10 grid grid-cols-1 lg:grid-cols-4 max-md:grid-cols-1">
        { for ctx.props().children.iter() }
      </div>
    }
  }
}