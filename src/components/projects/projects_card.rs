use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PropsProjectsCard {
  pub banner: String,
  pub title: String,
  pub description: String,
  pub url: String,
  pub icon_tech: Vec<String>,
}

pub struct ProjectsCard;

impl Component for ProjectsCard {
  type Message = ();
  type Properties = PropsProjectsCard;

  fn create(_: &Context<Self>) -> Self {
    ProjectsCard
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    html! {
      <div class=" sm-[41rem] h-[32rem] lg:min-h-[32.5rem] flex items-center  justify-center sm:w-[570px] w-[80vw]">
        <a class="relative group/pin z-50 cursor-pointer" target="_blank" href="https://newportifoliocardosofiles.netlify.app/">
          <div style="perspective:1000px;transform:rotateX(70deg) translateZ(0deg)" class="absolute left-1/2 top-1/2 ml-[0.09375rem] mt-4 -translate-x-1/2 -translate-y-1/2">
            <div style="transform: translate(-50%, -50%) rotateX(0deg) scale(1);" class="absolute left-1/2 p-4 top-1/2  flex justify-start items-start  rounded-2xl  shadow-[0_8px_16px_rgb(0_0_0/0.4)] border border-white/[0.1] group-hover/pin:border-white/[0.2] transition duration-700 overflow-hidden">
              <div class="relative z-50">
                <div class="relative flex items-center justify-center sm:w-[570px]  w-[80vw] overflow-hidden sm:h-[40vh] h-[30vh] lg:h-[32vh] mb-10">
                  <div class="relative w-full h-full overflow-hidden lg:rounded-3xl bg-[#13162d] ">
                    <img src={format!("{}", ctx.props().banner )} alt="bg-img" />
                  </div>
                  <img src="/web-portifolio.png" alt="Portifólio WEB" class="z-10 w-full h-full object-cover absolute bottom-0" />
                </div>
                <h1 class="font-bold lg:text-2xl md:text-xl text-base line-clamp-1 text-white">{ format!("{}", ctx.props().title ) }</h1>
                <p class="lg:text-xl lg:font-normal font-light text-sm line-clamp-2 text-neutral-400">{ format!("{}", ctx.props().description ) }</p>
                <div class="flex items-center justify-between mt-7 mb-3">
                  <div class="flex items-center">
                    { for ctx.props().icon_tech.iter().map(|icon| html! {
                      <div class="border border-white/[0.2] rounded-full bg-black lg:w-10  lg:h-10 w-8 h-8 flex justify-center items-center" style="transform:translateX(-2px)">
                        <img src={format!("{}", icon )} alt="/html.svg" class="p-2" />
                      </div>
                    }) }
                  </div>
                  <div class="flex flex-row items-center justify-center mr-2">
                      <p class="flex lg:text-xl md:text-xs text-sm text-purple">{"Verifique o Site"}</p>
                      <svg stroke="currentColor" fill="currentColor" stroke-width="0" viewBox="0 0 512 512" class="ms-3" color="#CBACF9" style="color:#CBACF9" height="1em" width="1em" xmlns="http://www.w3.org/2000/svg">
                        <path
                          d="M444.52 3.52L28.74 195.42c-47.97 22.39-31.98 92.75 19.19 92.75h175.91v175.91c0 51.17 70.36 67.17 92.75 19.19l191.9-415.78c15.99-38.39-25.59-79.97-63.97-63.97z">
                        </path>
                      </svg>
                    </div>
                  </div>
                </div>
              </div>
            </div>
            <div class="pointer-events-none w-full h-80 flex items-center justify-center opacity-0 group-hover/pin:opacity-100 z-[60] transition duration-500">
              <div class=" w-full h-full -mt-7 flex-none  inset-0">
                <div class="absolute top-0 inset-x-0  flex justify-center">
                  <div class="relative flex space-x-2 items-center z-10 rounded-full bg-zinc-950 py-0.5 px-4 ring-1 ring-white/10 ">
                    <span class="relative z-20 text-white text-xs font-bold inline-block py-0.5">{ format!("{}", ctx.props().url ) }</span>
                    <span class="absolute -bottom-0 left-[1.125rem] h-px w-[calc(100%-2.25rem)] bg-gradient-to-r from-emerald-400/0 via-emerald-400/90 to-emerald-400/0 transition-opacity duration-500 group-hover/btn:opacity-40"></span>
                  </div>
                </div>
                <div style="perspective:1000px;transform:rotateX(70deg) translateZ(0)" class="absolute left-1/2 top-1/2 ml-[0.09375rem] mt-4 -translate-x-1/2 -translate-y-1/2">
                  <div class="absolute left-1/2 top-1/2  h-[11.25rem] w-[11.25rem] rounded-[50%] bg-sky-500/[0.08] shadow-[0_8px_16px_rgb(0_0_0/0.4)]" style="opacity: 0; transform: translateX(-50%) translateY(-50%) translateZ(0px) scale(0.975653);"></div>
                  <div class="absolute left-1/2 top-1/2  h-[11.25rem] w-[11.25rem] rounded-[50%] bg-sky-500/[0.08] shadow-[0_8px_16px_rgb(0_0_0/0.4)]" style="opacity: 0; transform: translateX(-50%) translateY(-50%) translateZ(0px) scale(0.732347);"></div>
                  <div class="absolute left-1/2 top-1/2  h-[11.25rem] w-[11.25rem] rounded-[50%] bg-sky-500/[0.08] shadow-[0_8px_16px_rgb(0_0_0/0.4)]" style="opacity: 0; transform: translateX(-50%) translateY(-50%) translateZ(0px) scale(0.326399);"></div>
                </div>
                <div class="absolute right-1/2 bottom-1/2 bg-gradient-to-b from-transparent to-cyan-500 translate-y-[14px] w-px h-20 group-hover/pin:h-40 blur-[2px]"></div>
                <div class="absolute right-1/2 bottom-1/2 bg-gradient-to-b from-transparent to-cyan-500 translate-y-[14px] w-px h-20 group-hover/pin:h-40  "></div>
                <div class="absolute right-1/2 translate-x-[1.5px] bottom-1/2 bg-cyan-600 translate-y-[14px] w-[4px] h-[4px] rounded-full z-40 blur-[3px]"></div>
                <div class="absolute right-1/2 translate-x-[0.5px] bottom-1/2 bg-cyan-300 translate-y-[14px] w-[2px] h-[2px] rounded-full z-40 "></div>
              </div>
            </div>
        </a>
      </div>
    }
  }

}