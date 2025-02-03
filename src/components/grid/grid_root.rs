use yew::prelude::*;
use crate::components::ui::{MagicButton};

#[derive(Properties, PartialEq)]
pub struct PropsGridRoot {
  #[prop_or_default]
  pub children: Children,
}

pub struct GridRoot;

impl Component for GridRoot {
    type Message = ();
    type Properties = PropsGridRoot;

    fn create(_: &Context<Self>) -> Self {
        GridRoot
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
          <section id="about" class="mt-8">
            <div class="grid grid-cols-1 md:grid-cols-6 lg:grid-cols-5 md:grid-row-7 gap-4 lg:gap-8 mx-auto">
              { for ctx.props().children.iter() }

                <div class="md:col-span-3 md:row-span-2 row-span-1 relative overflow-hidden rounded-3xl border border-white/[0.1] group/bento hover:shadow-xl transition duration-200 shadow-input dark:shadow-none justify-between flex flex-col space-y-4 text-white" style="background:rgb(2,0,36);background-color:linear-gradient(90deg, rgba(2,0,36,1) 0%, rgba(59,59,68,1) 50%, rgba(93,108,111,1) 100%)">
                  <div class="false h-full'"><div class="w-full h-full absolute">
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

                <div class="lg:col-span-2 md:col-span-3 md:row-span-1 row-span-1 relative overflow-hidden rounded-3xl border border-white/[0.1] group/bento hover:shadow-xl transition duration-200 shadow-input dark:shadow-none justify-between flex flex-col space-y-4 text-white" style="background:rgb(2,0,36);background-color:linear-gradient(90deg, rgba(2,0,36,1) 0%, rgba(59,59,68,1) 50%, rgba(93,108,111,1) 100%)">
                  <div class="flex justify-center h-full'">
                    <div class="w-full h-full absolute"></div>
                    <div class="absolute right-0 -bottom-5 false"></div>
                    <div style="background-image: linear-gradient(40deg,rgb(108, 0, 162),rgb(0, 17, 82));" class="w-full h-full absolute overflow-hidden top-0 left-0 bg-[linear-gradient(40deg,rgb(108, 0, 162),rgb(0, 17, 82))]">
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
                      <div class="font-sans font-extralight max-sm:max-w-[8rem] md:max-w-[8rem] md:text-xs lg:max-w-40  lg:text-base text-sm text-[#C1C2D3] z-10"></div>
                      <div class="font-sans text-lg lg:text-3xl max-w-96 font-bold z-10">{"Quer começar um projeto juntos?"}</div>
                      <div class="mt-5 relative">
                        <div class="absolute -bottom-5 right-0 block ">
                          <div style="width:400px;height:200px;overflow:hidden;margin:0 auto;outline:none" title="" role="button" aria-label="animation" tabindex="0">
                          </div>
                        </div>
                        <MagicButton>
                          <svg stroke="currentColor" fill="currentColor" stroke-width="0" viewBox="0 0 512 512" height="1em" width="1em" xmlns="http://www.w3.org/2000/svg">
                            <rect width="336" height="336" x="128" y="128" fill="none" stroke-linejoin="round" stroke-width="32" rx="57" ry="57"></rect>
                            <path fill="none" stroke-linecap="round" stroke-linejoin="round" stroke-width="32" d="m383.5 128 .5-24a56.16 56.16 0 0 0-56-56H112a64.19 64.19 0 0 0-64 64v216a56.16 56.16 0 0 0 56 56h24"></path>
                          </svg>
                          {"Copiar E-mail"}
                        </MagicButton>
                      </div>
                    </div>
                  </div>
                </div>
          </div>
        </section>

        }
    }
}
