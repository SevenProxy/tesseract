use yew::prelude::*;

pub struct GridRoot;

impl Component for GridRoot {
  type Message = ();
  type Properties = ();

  fn create(_: &Context<Self>) -> Self {
      GridRoot
  }

  fn view(&self, _: &Context<Self>) -> Html {
    html! {
      <section id="about" class="mt-8">
        <div class="grid grid-cols-1 md:grid-cols-6 lg:grid-cols-5 md:grid-row-7 gap-4 lg:gap-8 mx-auto">
            <div class="lg:col-span-3 md:col-span-6 md:row-span-4 lg:min-h-[60vh] row-span-1 relative overflow-hidden rounded-3xl border border-white/[0.1] group/bento hover:shadow-xl transition duration-200 shadow-input dark:shadow-none justify-between flex flex-col space-y-4 text-white" style="background:rgb(2,0,36);background-color:linear-gradient(90deg, rgba(2,0,36,1) 0%, rgba(59,59,68,1) 50%, rgba(93,108,111,1) 100%)">
              <div class="false h-full'">
                <div class="w-full h-full absolute">
                  <img src="/public/background.jpg" alt="/background.jpg" class="w-full h-full object-cover object-center" />
                </div>
                <div class="absolute right-0 -bottom-5 false"></div>
                <div class="justify-end group-hover/bento:translate-x-2 transition duration-200 relative md:h-full min-h-40 flex flex-col p-5 lg:p-10">
                  <div class="font-sans font-extralight max-sm:max-w-32 md:max-w-32 md:text-xs lg:max-w-40  lg:text-base text-sm text-[#C1C2D3] z-10">
                  </div>
                  <div class="font-sans text-lg lg:text-3xl max-w-96 font-bold z-10">{"Colaboração com cliente, promovendo a comunicação aberta"}</div>
                </div>
              </div>
            </div>

            <div class="lg:col-span-2 md:col-span-3 md:row-span-2 row-span-1 relative overflow-hidden rounded-3xl border border-white/[0.1] group/bento hover:shadow-xl transition duration-200 shadow-input dark:shadow-none justify-between flex flex-col space-y-4 text-white" style="background:rgb(2,0,36);background-color:linear-gradient(90deg, rgba(2,0,36,1) 0%, rgba(59,59,68,1) 50%, rgba(93,108,111,1) 100%)">
              <div class="false h-full'">
                <div class="w-full h-full absolute">
                </div>
                <div class="absolute right-0 -bottom-5 false"></div>
                <div class="justify-start group-hover/bento:translate-x-2 transition duration-200 relative md:h-full min-h-40 flex flex-col p-5 lg:p-10">
                  <div class="font-sans font-extralight max-sm:max-w-32 md:max-w-32 md:text-xs lg:max-w-40  lg:text-base text-sm text-[#C1C2D3] z-10"></div>
                  <div class="font-sans text-lg lg:text-3xl max-w-96 font-bold z-10">{"Comunicações flexíveis em diferentes fusos horários"}</div>
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

            
            <div class="lg:col-span-2 md:col-span-3 md:row-span-2 row-span-1 relative overflow-hidden rounded-3xl border border-white/[0.1] group/bento hover:shadow-xl transition duration-200 shadow-input dark:shadow-none justify-between flex flex-col space-y-4 text-white" style="background:rgb(2,0,36);background-color:linear-gradient(90deg, rgba(2,0,36,1) 0%, rgba(59,59,68,1) 50%, rgba(93,108,111,1) 100%)">
              <div class="false h-full'">
                <div class="w-full h-full absolute"></div><div class="absolute right-0 -bottom-5 false"></div>
                <div class="justify-center group-hover/bento:translate-x-2 transition duration-200 relative md:h-full min-h-40 flex flex-col p-5 lg:p-10">
                  <div class="font-sans font-extralight max-sm:max-w-32 md:max-w-32 md:text-xs lg:max-w-40  lg:text-base text-sm text-[#C1C2D3] z-10">{"Estou constantemente me atualizando com as"}</div>
                  <div class="font-sans text-lg lg:text-3xl max-w-96 font-bold z-10">{"Linguagens"}</div>
                  <div class="flex gap-1 lg:gap-5 w-fit absolute -right-3 lg:-right-2 ">
                    <div class="flex flex-col gap-3 lg:gap-8">
                      <span class="lg:py-4 lg:px-3 py-2 px-3 text-xs lg:text-base opacity-50  lg:opacity-100 rounded-lg text-center bg-[#10132E]">{"JavaScript"}</span>
                      <span class="lg:py-4 lg:px-3 py-2 px-3 text-xs lg:text-base opacity-50  lg:opacity-100 rounded-lg text-center bg-[#10132E]">{"React.JS"}</span>
                      <span class="lg:py-4 lg:px-3 py-2 px-3 text-xs lg:text-base opacity-50  lg:opacity-100 rounded-lg text-center bg-[#10132E]">{"Next.js"}</span>
                      <span class="lg:py-4 lg:px-3 py-4 px-3 rounded-lg text-center bg-[#10132e]"></span>
                    </div>
                    <div class="flex flex-col gap-3 md:gap-3 lg:gap-8">
                      <span class="lg:py-4 lg:px-3 py-4 px-3 rounded-lg text-center bg-[#10132E]"></span>
                      <span class="lg:py-4 lg:px-3 py-2 px-3 text-xs lg:text-base opacity-50  lg:opacity-100 rounded-lg text-center bg-[#10132E]">{"TypeScript"}</span>
                      <span class="lg:py-4 lg:px-3 py-2 px-3 text-xs lg:text-base opacity-50  lg:opacity-100 rounded-lg text-center bg-[#10132E]">{"MongoDB"}</span>
                      <span class="lg:py-4 lg:px-3 py-2 px-3 text-xs lg:text-base opacity-50  lg:opacity-100 rounded-lg text-center bg-[#10132E]">{"PostgreSQL"}</span>
                    </div>
                  </div>
                </div>
              </div>
            </div>
            
            <div class="lg:col-span-2 md:col-span-3 md:row-span-1 row-span-1 relative overflow-hidden rounded-3xl border border-white/[0.1] group/bento hover:shadow-xl transition duration-200 shadow-input dark:shadow-none justify-between flex flex-col space-y-4 text-white" style="background:rgb(2,0,36);background-color:linear-gradient(90deg, rgba(2,0,36,1) 0%, rgba(59,59,68,1) 50%, rgba(93,108,111,1) 100%)">
              <div class="false h-full'">
                <div class="w-full h-full absolute">
                  <img src="/public/grid.svg" alt="/grid.svg" class="object-cover object-center" />
                </div>
                <div class="absolute right-0 -bottom-5 false">
                  <img src="/public/b4.svg" alt="/b4.svg" class="object-cover object-center w-full h-full" />
                </div>
                <div class="justify-start group-hover/bento:translate-x-2 transition duration-200 relative md:h-full min-h-40 flex flex-col p-5 lg:p-10">
                  <div class="font-sans font-extralight max-sm:max-w-32 md:max-w-32 md:text-xs lg:max-w-40  lg:text-base text-sm text-[#C1C2D3] z-10"></div>
                  <div class="font-sans text-lg lg:text-3xl max-w-96 font-bold z-10">{"Entusiasta de tecnologia com paixão pelo desenvolvimento"}</div>
                </div>
              </div>
            </div>

            <div class="md:col-span-3 md:row-span-2 row-span-1 relative overflow-hidden rounded-3xl border border-white/[0.1] group/bento hover:shadow-xl transition duration-200 shadow-input dark:shadow-none justify-between flex flex-col space-y-4 text-white" style="background:rgb(2,0,36);background-color:linear-gradient(90deg, rgba(2,0,36,1) 0%, rgba(59,59,68,1) 50%, rgba(93,108,111,1) 100%)">
              <div class="false h-full'"><div class="w-full h-full absolute">
                <img src="/public/b5.svg" alt="/b5.svg" class="absolute right-0 bottom-0 md:w-96 w-60 object-cover object-center" />
              </div>
              <div class="absolute right-0 -bottom-5 w-full opacity-80">
                <img src="/public/grid.svg" alt="/grid.svg" class="object-cover object-center w-full h-full" />
              </div>
              <div class="justify-center md:justify-start lg:justify-center group-hover/bento:translate-x-2 transition duration-200 relative md:h-full min-h-40 flex flex-col p-5 lg:p-10">
                <div class="font-sans font-extralight max-sm:max-w-32 md:max-w-32 md:text-xs lg:max-w-40  lg:text-base text-sm text-[#C1C2D3] z-10">{"Rocketseat OneBitCode"}</div>
                  <div class="font-sans text-lg lg:text-3xl max-w-96 font-bold z-10">{"Atualmente cursando React e JavaScript Full Stack"}</div>
                </div>
              </div>
            </div>

            <div class="lg:col-span-2 md:col-span-3 md:row-span-1 row-span-1 relative overflow-hidden rounded-3xl border border-white/[0.1] group/bento hover:shadow-xl transition duration-200 shadow-input dark:shadow-none justify-between flex flex-col space-y-4 text-white" style="background:rgb(2,0,36);background-color:linear-gradient(90deg, rgba(2,0,36,1) 0%, rgba(59,59,68,1) 50%, rgba(93,108,111,1) 100%)">
              <div class="flex justify-center h-full'">
                <div class="w-full h-full absolute"></div>
                <div class="absolute right-0 -bottom-5 false"></div>
                <div class="w-full h-full absolute overflow-hidden top-0 left-0 bg-[linear-gradient(40deg,var(--gradient-background-start),var(--gradient-background-end))]">
                  <svg class="hidden">
                    <defs>
                      <filter id="blurMe">
                      </filter>
                    </defs>
                  </svg>
                  <div class="">
                    <div class="absolute z-50 inset-0 flex items-center justify-center text-white font-bold px-4 pointer-events-none text-3xl text-center md:text-4xl lg:text-7xl"></div>
                  </div>
                  <div class="gradients-container h-full w-full blur-lg [filter:url(#blurMe)_blur(40px)]">
                    <div class="absolute [background:radial-gradient(circle_at_center,_var(--first-color)_0,_var(--first-color)_50%)_no-repeat] [mix-blend-mode:var(--blending-value)] w-[var(--size)] h-[var(--size)] top-[calc(50%-var(--size)/2)] left-[calc(50%-var(--size)/2)] [transform-origin:center_center] animate-first opacity-100"></div>
                    <div class="absolute [background:radial-gradient(circle_at_center,_rgba(var(--second-color),_0.8)_0,_rgba(var(--second-color),_0)_50%)_no-repeat] [mix-blend-mode:var(--blending-value)] w-[var(--size)] h-[var(--size)] top-[calc(50%-var(--size)/2)] left-[calc(50%-var(--size)/2)] [transform-origin:calc(50%-400px)] animate-second opacity-100"></div>
                    <div class="absolute [background:radial-gradient(circle_at_center,_rgba(var(--third-color),_0.8)_0,_rgba(var(--third-color),_0)_50%)_no-repeat] [mix-blend-mode:var(--blending-value)] w-[var(--size)] h-[var(--size)] top-[calc(50%-var(--size)/2)] left-[calc(50%-var(--size)/2)] [transform-origin:calc(50%+400px)] animate-third opacity-100"></div>
                    <div class="absolute [background:radial-gradient(circle_at_center,_rgba(var(--fourth-color),_0.8)_0,_rgba(var(--fourth-color),_0)_50%)_no-repeat] [mix-blend-mode:var(--blending-value)] w-[var(--size)] h-[var(--size)] top-[calc(50%-var(--size)/2)] left-[calc(50%-var(--size)/2)] [transform-origin:calc(50%-200px)] animate-fourth opacity-70"></div>
                    <div class="absolute [background:radial-gradient(circle_at_center,_rgba(var(--fifth-color),_0.8)_0,_rgba(var(--fifth-color),_0)_50%)_no-repeat] [mix-blend-mode:var(--blending-value)] w-[var(--size)] h-[var(--size)] top-[calc(50%-var(--size)/2)] left-[calc(50%-var(--size)/2)] [transform-origin:calc(50%-800px)_calc(50%+800px)] animate-fifth opacity-100"></div>
                    <div class="absolute [background:radial-gradient(circle_at_center,_rgba(var(--pointer-color),_0.8)_0,_rgba(var(--pointer-color),_0)_50%)_no-repeat] [mix-blend-mode:var(--blending-value)] w-full h-full -top-1/2 -left-1/2 opacity-70" style="transform: translate(0px, 0px);"></div>
                  </div>
                </div>
                <div class="justify-center md:max-w-full max-w-60 text-center group-hover/bento:translate-x-2 transition duration-200 relative md:h-full min-h-40 flex flex-col p-5 lg:p-10">
                  <div class="font-sans font-extralight max-sm:max-w-32 md:max-w-32 md:text-xs lg:max-w-40  lg:text-base text-sm text-[#C1C2D3] z-10"></div>
                  <div class="font-sans text-lg lg:text-3xl max-w-96 font-bold z-10">{"Quer começar um projeto juntos?"}</div>
                  <div class="mt-5 relative">
                    <div class="absolute -bottom-5 right-0 block ">
                      <div style="width:400px;height:200px;overflow:hidden;margin:0 auto;outline:none" title="" role="button" aria-label="animation" tabindex="0">
                      </div>
                    </div>
                    <button class="relative inline-flex h-12 w-full overflow-hidden rounded-lg  p-[1px] focus:outline-none md:w-60 md:mt-10">
                      {"Copiar E-mail"}
                    </button>
                  </div>
                </div>
              </div>
            </div>
      </div>
    </section>
      
    }
  }

}