use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PropsProjectsRoot {
  #[prop_or_default]
  pub children: Children,
}

pub struct ProjectsRoot;

impl Component for ProjectsRoot {
  type Message = ();
  type Properties = PropsProjectsRoot;

  fn create(_: &Context<Self>) -> Self {
    ProjectsRoot
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    html! {
      <div class="pt-40 pb-40" id="projects">
        <h1 class="heading">
          <span class="text-white">{"Galeria dos"}</span>
          <span class="bg-gradient-to-r from-blue-600 to-purple bg-clip-text text-transparent">{"Projetos Recentes"}</span>
        </h1>
        { for ctx.props().children.iter() }
      </div>
    }
  }

}