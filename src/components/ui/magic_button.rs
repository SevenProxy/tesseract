use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PropsMagicButton {
  #[prop_or_default]
  pub children: Children,
}

pub struct MagicButton;

impl Component for MagicButton {
  type Message = ();
  type Properties = PropsMagicButton;

  fn create(_: &Context<Self>) -> Self {
    MagicButton
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    html! {
      <button
        class="relative inline-flex h-12 w-full overflow-hidden rounded-lg 
      p-[1px] focus:outline-none md:w-60 md:mt-10"
      >
        <span
          class="absolute inset-[-1000%] animate-[spin_2s_linear_infinite] 
        bg-[conic-gradient(from_90deg_at_50%_50%,#E2CBFF_0%,#393BB2_50%,#E2CBFF_100%)]"
        />
        <span
          class="inline-flex h-full w-full cursor-pointer items-center justify-center 
        rounded-lg bg-slate-950 px-7 text-sm font-medium text-white backdrop-blur-3xl gap-2 ${otherClasses}"
        >
          { for ctx.props().children.iter()}
        </span>
      </button>
    }
  }

}