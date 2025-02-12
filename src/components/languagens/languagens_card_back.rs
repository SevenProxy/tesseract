use yew::prelude::*;

pub struct LanguagensCardBack;

impl Component for LanguagensCardBack {
  type Message = ();
  type Properties = ();

  fn create(_: &Context<Self>) -> Self {
    LanguagensCardBack
  }

  fn view(&self, _: &Context<Self>) -> Html {
    html! {
      <div class="max-w-sm md:max-w-full w-full mx-auto p-8 rounded-xl bordergroup border border-white/[0.1] group-hover/pin:border-white/[0.2] flex flex-col items-center justify-center" style="background:rgb(2,0,36);background-color:linear-gradient(90deg, rgba(2,0,36,1) 0%, rgba(59,59,68,1) 50%, rgba(93,108,111,1) 100%)">
        <div class="h-[15rem] md:h-[20rem] rounded-xl z-40 bg-[rgba(40,40,40,0.70)] [mask-image:radial-gradient(50%_50%_at_50%_50%,white_0%,transparent_100%)]">
          <div class="p-8 overflow-hidden h-full relative flex items-center justify-center">
            <div class="flex flex-row flex-shrink-0 justify-center items-center gap-2">
            <div class="md:h-24 md:w-24 lg:h-10 lg:w-10 rounded-full flex items-center justify-center h-8 w-8 circle-1" style="background: rgb(2, 0, 36); transform: translateY(0px);">
              <img class="h-4 w-4 select-nome mouseevent-none" src="/public/node-js.png" alt="node.js" />
            </div>
            <div class="md:h-24 md:w-24 lg:h-10 lg:w-10 rounded-full flex items-center justify-center h-12 w-12 circle-2" style="background: rgb(2, 0, 36); transform: translateY(0px);">
              <img class="lg:h-5 lg:w-5 md:h-7 md:w-7"  src="/public/python-5.svg" alt="python" />
            </div>
            <div class="h-16 w-16 md:h-24 md:w-24 lg:h-10 lg:w-10 rounded-full flex items-center justify-center circle-3" style="background: rgb(2, 0, 36); transform: translateY(-0.41581px);">
              <img class="lg:h-8 lg:w-8 md:h-10 md:w-10" src="/public/elixir.svg" alt="elixir" />
            </div>
            <div class="md:h-24 md:w-24 lg:h-10 lg:w-10 rounded-full flex items-center justify-center h-12 w-12 circle-4" style="background: rgb(2, 0, 36); transform: translateY(0px);">
              <img class="lg:h-8 lg:w-8 md:h-10 md:w-10" src="/public/rust.svg" alt="rust" />
            </div>
            <div class="md:h-24 md:w-24 lg:h-10 lg:w-10 rounded-full flex items-center justify-center h-8 w-8 circle-5" style="background: rgb(2, 0, 
            36); transform: translateY(0px);">
              <img class="md:h-24 md:w-24 lg:h-10 lg:w-10 rounded-full flex items-center justify-center h-8 w-8 circle-1" src="/public/go.svg" alt="golang" />
            </div>
          </div>
          <div class="h-40 w-px absolute top-20 m-auto z-40 bg-gradient-to-b from-transparent via-cyan-500 to-transparent animate-move">
            <div class="w-10 h-32 top-1/2 -translate-y-1/2 absolute -left-10">
              <div class="absolute inset-0">
                <span class="inline-block bg-black dark:bg-white" style="position: absolute; top: 55.7611px; left: 25.4556px; width: 2px; height: 2px; border-radius: 50%; z-index: 1; opacity: 1; transform: scale(0.7575) translateZ(0px);"></span>
                <span class="inline-block bg-black dark:bg-white" style="position: absolute; top: 45.528px; left: 25.7869px; width: 2px; height: 2px; border-radius: 50%; z-index: 1; opacity: 1; transform: scale(1.09971) translateZ(0px);"></span>
                <span class="inline-block bg-black dark:bg-white" style="position: absolute; top: 112.273px; left: 24.641px; width: 2px; height: 2px; border-radius: 50%; z-index: 1; opacity: 1; transform: scale(1.12555) translateZ(0px);"></span>
                <span class="inline-block bg-black dark:bg-white" style="position: absolute; top: 84.9973px; left: 2.94476px; width: 2px; height: 2px; border-radius: 50%; z-index: 1; opacity: 1; transform: scale(1.04626) translateZ(0px);"></span>
                <span class="inline-block bg-black dark:bg-white" style="position: absolute; top: 64.9557px; left: 21.4321px; width: 2px; height: 2px; border-radius: 50%; z-index: 1; opacity: 1; transform: scale(0.875982) translateZ(0px);"></span>
                <span class="inline-block bg-black dark:bg-white" style="position: absolute; top: 102.72px; left: 27.3998px; width: 2px; height: 2px; border-radius: 50%; z-index: 1; opacity: 1; transform: scale(1.13989) translateZ(0px);"></span>
                <span class="inline-block bg-black dark:bg-white" style="position: absolute; top: 69.5924px; left: 29.3426px; width: 2px; height: 2px; border-radius: 50%; z-index: 1; opacity: 1; transform: scale(1.05795) translateZ(0px);"></span>
                <span class="inline-block bg-black dark:bg-white" style="position: absolute; top: 98.4754px; left: 22.9847px; width: 2px; height: 2px; border-radius: 50%; z-index: 1; opacity: 1; transform: scale(1.1867) translateZ(0px);"></span>
                <span class="inline-block bg-black dark:bg-white" style="position: absolute; top: 116.966px; left: 21.6758px; width: 2px; height: 2px; border-radius: 50%; z-index: 1; opacity: 1; transform: scale(1.05136) translateZ(0px);"></span>
                <span class="inline-block bg-black dark:bg-white" style="position: absolute; top: 77.5791px; left: 36.2307px; width: 2px; height: 2px; border-radius: 50%; z-index: 1; opacity: 1; transform: scale(0.127453) translateZ(0px);"></span>
                <span class="inline-block bg-black dark:bg-white" style="position: absolute; top: 113.101px; left: 29.0703px; width: 2px; height: 2px; border-radius: 50%; z-index: 1; opacity: 1; transform: scale(0.640048) translateZ(0px);"></span>
                <span class="inline-block bg-black dark:bg-white" style="position: absolute; top: 100.776px; left: 10.6157px; width: 2px; height: 2px; border-radius: 50%; z-index: 1; opacity: 1; transform: scale(1.10346) translateZ(0px);"></span>
              </div>
            </div>
          </div>
        </div>
      </div>
      <h3 class="lg:text-lg font-semibold text-white md:text-3xl py-2">{"Back-end"}</h3>
      <p class="lg:text-sm md:text-xl font-normal text-neutral-400 max-w-sm">{"API, Arquitetura de Software, S.O.L.I.D, Cache, Armazenamento de Dados"}</p>
    </div>
    }
  }
}