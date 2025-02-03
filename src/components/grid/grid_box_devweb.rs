use yew::prelude::*;

pub struct GridBoxDevWeb;

impl Component for GridBoxDevWeb {
  type Message = ();
  type Properties = ();

  fn create(_: &Context<Self>) -> Self {
    GridBoxDevWeb
  }

  fn view(&self, _: &Context<Self>) -> Html {
    html! {
      <div class="lg:col-span-2 md:col-span-3 md:row-span-1 row-span-1 relative overflow-hidden rounded-3xl border border-white/[0.1] group/bento hover:shadow-xl transition duration-200 shadow-input dark:shadow-none justify-between flex flex-col space-y-4 text-white" style="background:rgb(2,0,36);background-color:linear-gradient(90deg, rgba(2,0,36,1) 0%, rgba(59,59,68,1) 50%, rgba(93,108,111,1) 100%)">
        <div class="false h-full'">
          <div class="w-full h-full absolute">
            <img src="/public/grid.svg" alt="/grid.svg" class="object-cover object-center" />
          </div>
          <div class="absolute right-0 -bottom-5 false">
            <img src="/public/b4.svg" alt="/b4.svg" class="object-cover object-center w-full h-full" />
          </div>
          <div class="justify-start group-hover/bento:translate-x-2 transition duration-200 relative md:h-full min-h-40 flex flex-col p-5 lg:p-10">
            <div class="font-sans font-extralight max-sm:max-w-[8rem] md:max-w-[8rem] md:text-xs lg:max-w-40  lg:text-base text-sm text-[#C1C2D3] z-10"></div>
            <div class="font-sans text-lg lg:text-3xl max-w-96 font-bold z-10">{"Entusiasta de tecnologia com paixão pelo desenvolvimento"}</div>
          </div>
        </div>
      </div>
    }
  }

}