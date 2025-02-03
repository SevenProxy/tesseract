use yew::prelude::*;

pub struct GridBoxCourse;

impl Component for GridBoxCourse {
  type Message = ();
  type Properties = ();

  fn create(_: &Context<Self>) -> Self {
    GridBoxCourse
  }

  fn view(&self, _: &Context<Self>) -> Html {
    html! {
      <div class="md:col-span-3 md:row-span-2 row-span-1 relative overflow-hidden rounded-3xl border border-white/[0.1] group/bento hover:shadow-xl transition duration-200 shadow-input dark:shadow-none justify-between flex flex-col space-y-4 text-white" style="background:rgb(2,0,36);background-color:linear-gradient(90deg, rgba(2,0,36,1) 0%, rgba(59,59,68,1) 50%, rgba(93,108,111,1) 100%)">
        <div class="false h-full'">
          <div class="w-full h-full absolute">
            <img src="/public/b5.svg" alt="/b5.svg" class="absolute right-0 bottom-0 md:w-96 w-60 object-cover object-center" />
          </div>
          <div class="absolute right-0 -bottom-5 w-full opacity-80">
            <img src="/public/grid.svg" alt="/grid.svg" class="object-cover object-center w-full h-full" />
          </div>
          <div class="justify-center md:justify-start lg:justify-center group-hover/bento:translate-x-2 transition duration-200 relative md:h-full min-h-40 flex flex-col p-5 lg:p-10">
            <div class="font-sans font-extralight max-sm:max-w-[8rem] md:max-w-[8rem] md:text-xs lg:max-w-40  lg:text-base text-sm text-[#C1C2D3] z-10">{"Unicessumar"}</div>
            <div class="font-sans text-lg lg:text-3xl max-w-96 font-bold z-10">{"Atualmente cursando CyberSecurity e DevOps."}</div>
          </div>
        </div>
      </div>
    }
  }

}