use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PropsCertificatesRoot {
  #[prop_or_default]
  pub children: Children,
}

pub struct CertificatesRoot;

impl Component for CertificatesRoot {
  type Message = ();
  type Properties = PropsCertificatesRoot;

  fn create(_: &Context<Self>) -> Self {
    CertificatesRoot
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    html! {
      <div class="pt-40 pb-40 w-full">
        <h1 class="heading" id="knowledge">
          <span class="text-white">{"Meus"}</span>
          <span class="bg-gradient-to-r from-blue-600 to-purple bg-clip-text text-transparent">{" Certificados"}</span>
        </h1>
        { for ctx.props().children.iter() }
      </div>
    }
  }
}