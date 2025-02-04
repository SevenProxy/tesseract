use yew::prelude::*;

pub struct ProjectsRoot;

impl Component for ProjectsRoot {
  type Message = ();
  type Properties = ();

  fn create(_: &Context<Self>) -> Self {
    ProjectsRoot
  }

  fn view(&self, _: &Context<Self>) -> Html {
    html! {
      <div class="pt-40 pb-40" id="projects">
        <h1 class="heading">
          <span class="text-white">{"Galeria dos"}</span>
          <span class="bg-gradient-to-r from-blue-600 to-purple bg-clip-text text-transparent">{"Projetos Recentes"}</span>
        </h1>
        <div class="flex flex-wrap items-center justify-center p-4 gap-x-24 gap-y-8 mt-10">
          <div class=" sm-[41rem] h-[32rem] lg:min-h-[32.5rem] flex items-center  justify-center sm:w-[570px] w-[80vw]"><a
              class="relative group/pin z-50 cursor-pointer" target="_blank"
              href="https://newportifoliocardosofiles.netlify.app/">
              <div style="perspective:1000px;transform:rotateX(70deg) translateZ(0deg)"
                class="absolute left-1/2 top-1/2 ml-[0.09375rem] mt-4 -translate-x-1/2 -translate-y-1/2">
                <div style="transform: translate(-50%, -50%) rotateX(0deg) scale(1);"
                  class="absolute left-1/2 p-4 top-1/2  flex justify-start items-start  rounded-2xl  shadow-[0_8px_16px_rgb(0_0_0/0.4)] border border-white/[0.1] group-hover/pin:border-white/[0.2] transition duration-700 overflow-hidden">
                  <div class="relative z-50">
                    <div
                      class="relative flex items-center justify-center sm:w-[570px]  w-[80vw] overflow-hidden sm:h-[40vh] h-[30vh] lg:h-[32vh] mb-10">
                      <div class="relative w-full h-full overflow-hidden lg:rounded-3xl bg-[#13162d] ">
                        <img src="/web-portifolio.png" alt="bg-img" />
                      </div>
                      <img src="/web-portifolio.png" alt="Portifólio WEB"
                        class="z-10 w-full h-full object-cover absolute bottom-0" />
                    </div>
                    <h1 class="font-bold lg:text-2xl md:text-xl text-base line-clamp-1 text-white">{"Portifólio WEB"}</h1>
                    <p class="lg:text-xl lg:font-normal font-light text-sm line-clamp-2 text-neutral-400">
                    {"Desenvolvido com
                      HTML, CSS, jQuery e PHP, este projeto visa apresentar meu currículo e certificados de forma acessível e
                      organizada para recrutadores."}</p>
                    <div class="flex items-center justify-between mt-7 mb-3">
                      <div class="flex items-center">
                        <div
                          class="border border-white/[0.2] rounded-full bg-black lg:w-10  lg:h-10 w-8 h-8 flex justify-center items-center"
                          style="transform:translateX(-2px)">
                          <img src="/html.svg" alt="/html.svg" class="p-2" /></div>
                        <div
                          class="border border-white/[0.2] rounded-full bg-black lg:w-10  lg:h-10 w-8 h-8 flex justify-center items-center"
                          style="transform:translateX(-7px)">
                          <img src="/css.svg" alt="/css.svg" class="p-2" /></div>
                        <div
                          class="border border-white/[0.2] rounded-full bg-black lg:w-10  lg:h-10 w-8 h-8 flex justify-center items-center"
                          style="transform:translateX(-12px)">
                          <img src="/js.svg" alt="/js.svg" class="p-2"/></div>
                        <div
                          class="border border-white/[0.2] rounded-full bg-black lg:w-10  lg:h-10 w-8 h-8 flex justify-center items-center"
                          style="transform:translateX(-17px)">
                          <img src="/three.svg" alt="/three.svg" class="p-2"/></div>
                        <div
                          class="border border-white/[0.2] rounded-full bg-black lg:w-10  lg:h-10 w-8 h-8 flex justify-center items-center"
                          style="transform:translateX(-22px)">
                          <img src="/fm.svg" alt="/fm.svg" class="p-2"/></div>
                      </div>
                      <div class="flex flex-row items-center justify-center mr-2">
                        <p class="flex lg:text-xl md:text-xs text-sm text-purple">{"Verifique o Site"}</p><svg
                          stroke="currentColor" fill="currentColor" stroke-width="0" viewBox="0 0 512 512" class="ms-3"
                          color="#CBACF9" style="color:#CBACF9" height="1em" width="1em" xmlns="http://www.w3.org/2000/svg">
                          <path
                            d="M444.52 3.52L28.74 195.42c-47.97 22.39-31.98 92.75 19.19 92.75h175.91v175.91c0 51.17 70.36 67.17 92.75 19.19l191.9-415.78c15.99-38.39-25.59-79.97-63.97-63.97z">
                          </path>
                        </svg>
                      </div>
                    </div>
                  </div>
                </div>
              </div>
              <div
                class="pointer-events-none w-full h-80 flex items-center justify-center opacity-0 group-hover/pin:opacity-100 z-[60] transition duration-500">
                <div class=" w-full h-full -mt-7 flex-none  inset-0">
                  <div class="absolute top-0 inset-x-0  flex justify-center">
                    <div
                      class="relative flex space-x-2 items-center z-10 rounded-full bg-zinc-950 py-0.5 px-4 ring-1 ring-white/10 ">
                      <span
                        class="relative z-20 text-white text-xs font-bold inline-block py-0.5">{"https://newportifoliocardosofiles.netlify.app/"}</span><span
                        class="absolute -bottom-0 left-[1.125rem] h-px w-[calc(100%-2.25rem)] bg-gradient-to-r from-emerald-400/0 via-emerald-400/90 to-emerald-400/0 transition-opacity duration-500 group-hover/btn:opacity-40"></span>
                    </div>
                  </div>
                  <div style="perspective:1000px;transform:rotateX(70deg) translateZ(0)"
                    class="absolute left-1/2 top-1/2 ml-[0.09375rem] mt-4 -translate-x-1/2 -translate-y-1/2">
                    <div
                      class="absolute left-1/2 top-1/2  h-[11.25rem] w-[11.25rem] rounded-[50%] bg-sky-500/[0.08] shadow-[0_8px_16px_rgb(0_0_0/0.4)]"
                      style="opacity: 0; transform: translateX(-50%) translateY(-50%) translateZ(0px) scale(0.975653);"></div>
                    <div
                      class="absolute left-1/2 top-1/2  h-[11.25rem] w-[11.25rem] rounded-[50%] bg-sky-500/[0.08] shadow-[0_8px_16px_rgb(0_0_0/0.4)]"
                      style="opacity: 0; transform: translateX(-50%) translateY(-50%) translateZ(0px) scale(0.732347);"></div>
                    <div
                      class="absolute left-1/2 top-1/2  h-[11.25rem] w-[11.25rem] rounded-[50%] bg-sky-500/[0.08] shadow-[0_8px_16px_rgb(0_0_0/0.4)]"
                      style="opacity: 0; transform: translateX(-50%) translateY(-50%) translateZ(0px) scale(0.326399);"></div>
                  </div>
                  <div
                    class="absolute right-1/2 bottom-1/2 bg-gradient-to-b from-transparent to-cyan-500 translate-y-[14px] w-px h-20 group-hover/pin:h-40 blur-[2px]">
                  </div>
                  <div
                    class="absolute right-1/2 bottom-1/2 bg-gradient-to-b from-transparent to-cyan-500 translate-y-[14px] w-px h-20 group-hover/pin:h-40  ">
                  </div>
                  <div
                    class="absolute right-1/2 translate-x-[1.5px] bottom-1/2 bg-cyan-600 translate-y-[14px] w-[4px] h-[4px] rounded-full z-40 blur-[3px]">
                  </div>
                  <div
                    class="absolute right-1/2 translate-x-[0.5px] bottom-1/2 bg-cyan-300 translate-y-[14px] w-[2px] h-[2px] rounded-full z-40 ">
                  </div>
                </div>
              </div>
            </a></div>
          </div>
      </div>
    }
  }

}