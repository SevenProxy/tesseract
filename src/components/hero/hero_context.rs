use yew::prelude::*;

pub struct HeroContext;

impl Component for HeroContext {
  type Message = ();
  type Properties = ();

  fn create(_: &Context<Self>) -> Self {
    HeroContext
  }

  fn view(&self, _: &Context<Self>) -> Html {
    html! {
      <div class="h-screen w-full bg-black-100 dark:bg-grid-white/[0.06] bg-grid-white/[0.06] flex items-center justify-center absolute top-0 left-0">
        <div
          class="absolute pointer-events-none inset-0 flex items-center justify-center
         bg-black-100 [mask-image:radial-gradient(ellipse_at_center,transparent_20%,black)]"
        />
      </div>
    }
  }

}