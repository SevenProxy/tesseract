use yew::prelude::*;

pub struct KnowBoxTech;

impl Component for KnowBoxTech {
  type Message = ();
  type Properties = ();

  fn create(_: &Context<Self>) -> Self {
    KnowBoxTech
  }

  fn view(&self, _: &Context<Self>) -> Html {
    html! {
      <div class="flex flex-wrap items-center justify-center gap-4 md:gap-16 max-lg:mt-10">
        <div class="flex md:max-w-60 sm:h-full md:max-h-full max-w-32 gap-2">
          <img src="/public/ts.svg" alt="TypeScript" class="md:w-10 w-5" />
          <img src="/public/TypeScript-Logo.wine.svg" alt="TypeScript" width="150" class="md:w-24 w-20" />
        </div>
        <div class="flex md:max-w-60 sm:h-full md:max-h-full max-w-32 gap-2">
          <img src="/public/next.svg" alt="Next.js" class="md:w-10 w-5" />
          <img src="/public/nextjs-svgrepo-com.svg" alt="Next.js" width="150" class="md:w-24 w-20" />
        </div>
        <div class="flex md:max-w-60 sm:h-full md:max-h-full max-w-32 gap-2">
          <img src="/public/tail.svg" alt="stream" class="md:w-10 w-5" />
          <img src="/public/tailwindcss-logotype.svg" alt="stream" width="100" class="md:w-24 w-20" />
        </div>
      </div>
    }
  }

}