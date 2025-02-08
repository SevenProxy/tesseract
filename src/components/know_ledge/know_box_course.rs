use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PropsKnowBoxCourse {
  #[prop_or_default]
  pub children: Children,
}

pub struct KnowBoxCourse;

impl Component for KnowBoxCourse {
  type Message = ();
  type Properties = PropsKnowBoxCourse;

  fn create(_: &Context<Self>) -> Self {
    KnowBoxCourse
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    html! {
      <div class="h-[50vh] md:h-[30rem] rounded-md flex flex-col antialiased  items-center justify-center relative overflow-hidden mt-4">
        <div class="scroller relative z-20 w-screen overflow-hidden [mask-image:linear-gradient(to_right,transparent,white_20%,white_80%,transparent)]" style="--animation-direction: reverse; --animation-duration: 5s;">
          <ul class="flex min-w-full shrink-0 gap-16 py-4 w-max flex-nowrap animate-scroll hover:[animation-play-state:paused]">
            { for ctx.props().children.iter() }
          </ul>
        </div>
      </div>
    }
  }

}