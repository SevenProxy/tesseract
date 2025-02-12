use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PropsProjectsContainer {
  #[prop_or_default]
  pub children: Children,
}

pub struct ProjectsContainer;

impl Component for ProjectsContainer {
  type Message = ();
  type Properties = PropsProjectsContainer;

  fn create(_: &Context<Self>) -> Self {
    ProjectsContainer
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    html! {
      <div class="flex flex-wrap items-center justify-center p-4  gap-x-24 gap-y-24 mt-10">
        { for ctx.props().children.iter() }
      </div>
    }
  }

}