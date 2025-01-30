use yew::prelude::*;

pub struct Spotlight;

impl Component for Spotlight {
  type Message = ();
  type Properties = ();

  fn create(_: &Context<Self>) -> Self {
    Spotlight
  }

  fn view(&self, _: &Context<Self>) -> Html {
    html! {
      <div class="top-10 left-full h-[80vh] w-[50vw]">
        <svg
          class="animate-spotlight pointer-events-none absolute z-[1]  h-[169%] w-[138%] lg:w-[84%] opacity-0"
          xmlns="http://www.w3.org/2000/svg"
          viewBox="0 0 3787 2842"
          fill="none"
        >
          <g filter="url(#filter)">
            <ellipse
              cx="1924.71"
              cy="273.501"
              rx="1924.71"
              ry="273.501"
              transform="matrix(-0.822377 -0.568943 -0.568943 0.822377 3631.88 2291.09)"
              fill="white"
              fillOpacity="0.21"
            ></ellipse>
          </g>
          <defs>
            <filter
              id="filter"
              x="0.860352"
              y="0.838989"
              width="3785.16"
              height="2840.26"
              filterUnits="userSpaceOnUse"
              colorInterpolationFilters="sRGB"
            >
              
            </filter>
          </defs>
        </svg>
      </div>
    }
  }

}