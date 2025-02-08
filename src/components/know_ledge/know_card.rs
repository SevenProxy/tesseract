use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PropsKnowCard {
  pub title: String,
  pub name: String,
  pub description: String,
  pub icon: String,
}

pub struct KnowCard;

impl Component for KnowCard {
  type Message = ();
  type Properties = PropsKnowCard;

  fn create(_: &Context<Self>) -> Self {
    KnowCard
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    html! {
      <li class="w-[80vw] max-w-full relative rounded-2xl border flex-shrink-0 border-slate-800 p-5 md:p-16 md:w-[60vw]" style="background:rgb(2,0,36);background-color:linear-gradient(90deg, rgba(2,0,36,1) 0%, rgba(59,59,68,1) 50%, rgba(93,108,111,1) 100%)">
        <blockquote>
          <div aria-hidden="true" class="user-select-none -z-1 pointer-events-none absolute -left-0.5 -top-0.5 h-[calc(100%_+_4px)] w-[calc(100%_+_4px)]"></div>
          <span class="relative z-20 text-sm md:text-lg leading-[1.6] text-white font-normal">{ format!("{}", ctx.props().description) }</span>
          <div class="relative z-20 mt-6 flex flex-row items-center">
            <div class="me-3">
              <img src={ format!("{}", ctx.props().icon) } alt="logo" width="50" class="rounded-lg" />
            </div>
            <span class="flex flex-col gap-1">
              <span class="text-xl font-bold leading-[1.6] text-white">{ format!("{}", ctx.props().title) }</span>
              <span class=" text-sm leading-[1.6] text-white font-normal">{ format!("{}", ctx.props().name) }</span>
            </span>
          </div>
        </blockquote>
      </li>
    }
  }

}