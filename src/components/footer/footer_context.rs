use yew::prelude::*;
use crate::components::ui::MagicButton;

pub struct FooterContext;

impl Component for FooterContext {
  type Message = ();
  type Properties = ();

  fn create(_: &Context<Self>) -> Self {
    FooterContext
  }

  fn view(&self, _: &Context<Self>) -> Html {
    html! {
      <div class="flex flex-col items-center">
        <h1 class="tracking-widest text-xl text-center text-blue-200 heading lg:max-w-[45vw] ">
          {"Pronto para levar seus "}
          <span class="bg-gradient-to-r from-blue-600 to-purple bg-clip-text text-transparent">{"Websites "}</span>
          {"ao proximo nível?"}
        </h1>
        <p class="text-white-100 md:mt-10 my-5 text-center">{"Entre em contato comigo, vamos trabalhar juntos para alcançar o sucesso."}</p>
        <a class="flex items-center justify-center" href="mailto:deivbraga244@gmail.com">
          <MagicButton>
            {"Contact-me"}
            <svg stroke="currentColor" fill="currentColor" stroke-width="0" viewBox="0 0 512 512" height="1em" width="1em" xmlns="http://www.w3.org/2000/svg">
              <path d="M444.52 3.52L28.74 195.42c-47.97 22.39-31.98 92.75 19.19 92.75h175.91v175.91c0 51.17 70.36 67.17 92.75 19.19l191.9-415.78c15.99-38.39-25.59-79.97-63.97-63.97z"></path>
            </svg>
          </MagicButton>
        </a>
      </div>
    }
  }
}