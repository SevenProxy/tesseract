use yew::prelude::*;

pub struct KnowRoot;

impl Component for KnowRoot {
  type Message = ();
  type Properties = ();

  fn create(_: &Context<Self>) -> Self {
    KnowRoot
  }

  fn view(&self, _: &Context<Self>) -> Html {
    html! {
      <div>
        <h1 class="heading" id="knowledge">
            <span class="text-white">{"Educação e"}</span> 
            <span class="bg-gradient-to-r from-blue-600 to-purple bg-clip-text text-transparent">{"Capacitações"}</span>
        </h1>
        <div class="flex flex-col items-center max-lg:mt-10">
            <div class="h-[50vh] md:h-[30rem] rounded-md flex flex-col antialiased  items-center justify-center relative overflow-hidden mt-4">
                <div class="scroller relative z-20 w-screen overflow-hidden [mask-image:linear-gradient(to_right,transparent,white_20%,white_80%,transparent)]" style="--animation-direction: reverse; --animation-duration: 5s;">
                    <ul class="flex min-w-full shrink-0 gap-16 py-4 w-max flex-nowrap animate-scroll hover:[animation-play-state:paused]">
                        <li class="w-[80vw] max-w-full relative rounded-2xl border flex-shrink-0 border-slate-800 p-5 md:p-16 md:w-[60vw]" style="background:rgb(2,0,36);background-color:linear-gradient(90deg, rgba(2,0,36,1) 0%, rgba(59,59,68,1) 50%, rgba(93,108,111,1) 100%)">
                            <blockquote>
                                <div aria-hidden="true" class="user-select-none -z-1 pointer-events-none absolute -left-0.5 -top-0.5 h-[calc(100%_+_4px)] w-[calc(100%_+_4px)]"></div>
                                <span class="relative z-20 text-sm md:text-lg leading-[1.6] text-white font-normal">{"O curso prepara para implementar, manter e gerenciar infraestruturas de TI garantindo usabilidade, robustez, integridade e segurança."}</span>
                                <div class="relative z-20 mt-6 flex flex-row items-center">
                                    <div class="me-3">
                                        <img src="/image.png" alt="logo" width="50" class="rounded-lg" />
                                    </div>
                                    <span class="flex flex-col gap-1">
                                        <span class="text-xl font-bold leading-[1.6] text-white">{"Análise e Desenvolvimento de Sistemas"}</span>
                                        <span class=" text-sm leading-[1.6] text-white font-normal">{"Faculdade Anhanguera - São José dos Campos"}</span>
                                    </span>
                                </div>
                            </blockquote>
                        </li>
                        <li class="w-[80vw] max-w-full relative rounded-2xl border flex-shrink-0 border-slate-800 p-5 md:p-16 md:w-[60vw]" style="background:rgb(2,0,36);background-color:linear-gradient(90deg, rgba(2,0,36,1) 0%, rgba(59,59,68,1) 50%, rgba(93,108,111,1) 100%)">
                            <blockquote>
                                <div aria-hidden="true" class="user-select-none -z-1 pointer-events-none absolute -left-0.5 -top-0.5 h-[calc(100%_+_4px)] w-[calc(100%_+_4px)]"></div>
                                <span class="relative z-20 text-sm md:text-lg leading-[1.6] text-white font-normal">{"O curso prepara para implementar, manter e gerenciar infraestruturas de TI garantindo usabilidade, robustez, integridade e segurança."}</span>
                                <div class="relative z-20 mt-6 flex flex-row items-center">
                                    <div class="me-3">
                                        <img src="/image.png" alt="logo" width="50" class="rounded-lg" />
                                    </div>
                                    <span class="flex flex-col gap-1">
                                        <span class="text-xl font-bold leading-[1.6] text-white">{"Análise e Desenvolvimento de Sistemas"}</span>
                                        <span class=" text-sm leading-[1.6] text-white font-normal">{"Faculdade Anhanguera - São José dos Campos"}</span>
                                    </span>
                                </div>
                            </blockquote>
                        </li>
                        <li class="w-[80vw] max-w-full relative rounded-2xl border flex-shrink-0 border-slate-800 p-5 md:p-16 md:w-[60vw]" style="background:rgb(2,0,36);background-color:linear-gradient(90deg, rgba(2,0,36,1) 0%, rgba(59,59,68,1) 50%, rgba(93,108,111,1) 100%)">
                            <blockquote>
                                <div aria-hidden="true" class="user-select-none -z-1 pointer-events-none absolute -left-0.5 -top-0.5 h-[calc(100%_+_4px)] w-[calc(100%_+_4px)]"></div>
                                <span class="relative z-20 text-sm md:text-lg leading-[1.6] text-white font-normal">{"O curso prepara para implementar, manter e gerenciar infraestruturas de TI garantindo usabilidade, robustez, integridade e segurança."}</span>
                                <div class="relative z-20 mt-6 flex flex-row items-center">
                                    <div class="me-3">
                                        <img src="/image.png" alt="logo" width="50" class="rounded-lg" />
                                    </div>
                                    <span class="flex flex-col gap-1">
                                        <span class="text-xl font-bold leading-[1.6] text-white">{"Análise e Desenvolvimento de Sistemas"}</span>
                                        <span class=" text-sm leading-[1.6] text-white font-normal">{"Faculdade Anhanguera - São José dos Campos"}</span>
                                    </span>
                                </div>
                            </blockquote>
                        </li>
                    </ul>
                </div>
            </div>


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
        </div>
      </div>
    }
  }
}
