use yew::prelude::*;

pub struct GridBoxHour;

impl Component for GridBoxHour {
  type Message = ();
  type Properties = ();

  fn create(_: &Context<Self>) -> Self {
    GridBoxHour
  }

  fn view(&self, _: &Context<Self>) -> Html {
    html! {
      <div class="lg:col-span-2 md:col-span-3 md:row-span-2 row-span-1 relative overflow-hidden rounded-3xl border border-white/[0.1] group/bento hover:shadow-xl transition duration-200 shadow-input dark:shadow-none justify-between flex flex-col space-y-4 text-white" style="background:rgb(2,0,36);background-color:linear-gradient(90deg, rgba(2,0,36,1) 0%, rgba(59,59,68,1) 50%, rgba(93,108,111,1) 100%)">
        <div class="false h-full'">
          <div class="w-full h-full absolute"></div>
          <div class="absolute right-0 -bottom-5 false"></div>
          <div class="justify-start group-hover/bento:translate-x-2 transition duration-200 relative md:h-full min-h-40 flex flex-col p-5 lg:p-10">
            <div class="font-sans font-extralight max-sm:max-w-32 md:max-w-32 md:text-xs lg:max-w-40  lg:text-base text-sm text-[#C1C2D3] z-10"></div>
            <div style="max-width: 24rem;" class="font-sans text-lg lg:text-3xl max-w-96 font-bold z-10">{"Comunicações flexíveis em diferentes fusos horários"}</div>
            <div class="flex items-center justify-center absolute -left-5 top-36 md:top-40 lg:mt-7 w-full h-full">
              <div class="max-w-7xl mx-auto w-full relative overflow-hidden h-96 px-4">
                <div class="absolute w-full bottom-0 inset-x-0 h-40 bg-gradient-to-b  pointer-events-none select-none from-transparent dark:to-black to-white z-40"></div>
                <div class="absolute w-full h-72 md:h-full z-10">
                  <div style="position: relative; width: 100%; height: 100%; overflow: hidden; pointer-events: auto; touch-action: none;">
                    <div style="width: 100%; height: 100%;">
                      <canvas style="display: block; width: 384.797px; height: 384px; touch-action: auto;" data-engine="three.js r166" width="384" height="384"></canvas>
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    }
  }
}