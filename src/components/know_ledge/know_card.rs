use yew::prelude::*;

pub struct KnowCard;

impl Component for KnowCard {
  type Message = ();
  type Properties = ();

  fn create(_: &Context<Self>) -> Self {
    KnowCard
  }

  fn view(&self, _: &Context<Self>) -> Html {
    html! {
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
    }
  }

}