use yew::prelude::*;

pub struct NavbarContext;

impl Component for NavbarContext {
    type Message = ();
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        NavbarContext
    }

    fn view(&self, _: &Context<Self>) -> Html {
        html! {
          <div class="flex max-w-fit md:min-w-[70vw] lg:min-w-fit fixed z-[5000] top-10 inset-x-0 mx-auto px-3 md:px-10 py-5 rounded-lg border border-black/.1 shadow-[0px_2px_3px_-1px_rgba(0,0,0,0.1),0px_1px_0px_0px_rgba(25,28,33,0.02),0px_0px_0px_1px_rgba(25,28,33,0.08)] items-center justify-center space-x-4" style="backdrop-filter: blur(16px) saturate(180%); background-color: rgba(17, 25, 40, 0.75); border-radius: 12px; border: 1px solid rgba(255, 255, 255, 0.125); opacity: 1; transform: none;">
            <a class="relative dark:text-neutral-50 items-center flex space-x-1 text-white dark:hover:text-neutral-300 hover:text-neutral-500" href="#projects">
              <span class="block sm:hidden"></span>
              <span class=" text-sm !cursor-pointer">{"Home"}</span>
            </a>
            <a class="relative dark:text-neutral-50 items-center flex space-x-1 text-white dark:hover:text-neutral-300 hover:text-neutral-500" href="#about">
              <span class="block sm:hidden"></span>
              <span class=" text-sm !cursor-pointer">{"Sobre"}</span>
            </a>
            <a class="relative dark:text-neutral-50 items-center flex space-x-1 text-white dark:hover:text-neutral-300 hover:text-neutral-500" href="#projects">
              <span class="block sm:hidden"></span>
              <span class=" text-sm !cursor-pointer">{"Projetos"}</span>
            </a>
            <a class="relative dark:text-neutral-50 items-center flex space-x-1 text-white dark:hover:text-neutral-300 hover:text-neutral-500" href="#knowledge">
              <span class="block sm:hidden"></span>
              <span class=" text-sm !cursor-pointer">{"Cursos"}</span>
            </a>
          </div>
        }
    }
}
