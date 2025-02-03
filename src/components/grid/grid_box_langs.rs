use yew::prelude::*;

pub struct GridBoxLang;

impl Component for GridBoxLang {
  type Message = ();
  type Properties = ();

  fn create(_: &Context<Self>) -> Self {
    GridBoxLang
  }

  fn view(&self, _: &Context<Self>) -> Html {
    html! {
      <div class="lg:col-span-2 md:col-span-3 md:row-span-2 row-span-1 relative overflow-hidden rounded-3xl border border-white/[0.1] group/bento hover:shadow-xl transition duration-200 shadow-input dark:shadow-none justify-between flex flex-col space-y-4 text-white" style="background:rgb(2,0,36);background-color:linear-gradient(90deg, rgba(2,0,36,1) 0%, rgba(59,59,68,1) 50%, rgba(93,108,111,1) 100%)">
        <div class="false h-full'">
          <div class="w-full h-full absolute"></div><div class="absolute right-0 -bottom-5 false"></div>
          <div class="justify-center group-hover/bento:translate-x-2 transition duration-200 relative md:h-full min-h-40 flex flex-col p-5 lg:p-10">
            <div class="font-sans font-extralight sm:max-w-[8rem] md:max-w-[8rem] md:text-xs lg:max-w-40  lg:text-base text-sm text-[#C1C2D3] z-10">{"Estou constantemente me atualizando com as"}</div>
              <div style="max-width: 24rem;" class="font-sans text-lg lg:text-3xl max-w-96 font-bold z-10">{"Linguagens"}</div>
              <div class="flex gap-1 lg:gap-5 w-fit absolute -right-3 lg:-right-2 ">
              <div class="flex flex-col gap-3 lg:gap-8">
                <span class="lg:py-4 lg:px-3 py-2 px-3 text-xs lg:text-base opacity-50  lg:opacity-100 rounded-lg text-center bg-[#10132E]">{"TypeScript"}</span>
                <span class="lg:py-4 lg:px-3 py-2 px-3 text-xs lg:text-base opacity-50  lg:opacity-100 rounded-lg text-center bg-[#10132E]">{"Python"}</span>
                <span class="lg:py-4 lg:px-3 py-2 px-3 text-xs lg:text-base opacity-50  lg:opacity-100 rounded-lg text-center bg-[#10132E]">{"Elixir"}</span>
                <span class="lg:py-4 lg:px-3 py-4 px-3 rounded-lg text-center bg-[#10132e]"></span>
              </div>
              <div class="flex flex-col gap-3 md:gap-3 lg:gap-8">
                <span class="lg:py-4 lg:px-3 py-4 px-3 rounded-lg text-center bg-[#10132E]"></span>
                <span class="lg:py-4 lg:px-3 py-2 px-3 text-xs lg:text-base opacity-50  lg:opacity-100 rounded-lg text-center bg-[#10132E]">{"Golang"}</span>
                <span class="lg:py-4 lg:px-3 py-2 px-3 text-xs lg:text-base opacity-50  lg:opacity-100 rounded-lg text-center bg-[#10132E]">{"Rust"}</span>
                <span class="lg:py-4 lg:px-3 py-2 px-3 text-xs lg:text-base opacity-50  lg:opacity-100 rounded-lg text-center bg-[#10132E]">{"PostgreSQL"}</span>
              </div>
            </div>
          </div>
        </div>
      </div>
    }
  }
  
}