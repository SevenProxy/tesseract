use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PropsCertificatesCard {
  pub title: String,
  pub description: String,
  pub icon: String,
  pub url: String,
}

pub struct CertificatesCard;

impl Component for CertificatesCard {
  type Message = ();
  type Properties = PropsCertificatesCard;

  fn create(_: &Context<Self>) -> Self {
    CertificatesCard
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    html! {
      <a target="_blank" about="certificados" class="grid lg:col-span-2 w-full" href={format!("{}", ctx.props().url)}>
        <button class="bg-transparent relative text-xl p-[1px] overflow-hidden md:col-span-2 md:w-full" style="border-radius:1.75rem">
          <div class="absolute inset-0" style="border-radius:calc(1.75rem * 0.96)">
            <svg xmlns="http://www.w3.org/2000/svg" preserveAspectRatio="none" class="absolute h-full w-full" width="100%" height="100%">
              <rect fill="none" width="100%" height="100%" rx="30%" ry="30%"></rect>
            </svg>
            <div style="position: absolute; top: 0px; left: 0px; display: inline-block; transform: translateX(59.3431px) translateY(169.914px) translateX(-50%) translateY(-50%);">
              <div class="h-20 w-20 opacity-[0.8] bg-[radial-gradient(var(--sky-500)_40%,transparent_60%)]"></div>
            </div>
          </div>
          <div class="relative border backdrop-blur-xl flex items-center justify-center w-full text-sm antialiased text-white border-slate-800 h-[180px] p-5" style="border-radius:calc(1.75rem * 0.96);background:rgb(2,0,36);background-color:linear-gradient(90deg, rgba(2,0,36,1) 0%, rgba(59,59,68,1) 50%, rgba(93,108,111,1) 100%)">
            <div class="flex items-center justify-center gap-4 max-sm:gap-5">
              <img src={format!("{}", ctx.props().icon)} alt="Icon Card" class="lg:w-32 md:w-20 w-16" />
              <div class="lg:ms-5">
                <h1 class="text-start text-xl md:text-2xl font-bold">{format!("{}", ctx.props().title)}</h1>
                <p class="text-start text-white-100 mt-3 font-semibold ">{format!("{}", ctx.props().description)}</p>
                <p class="flex items-center lg:text-sm md:text-xs text-xs text-purple mt-3">{"Verifique o Link"}
                  <svg stroke="currentColor" fill="currentColor" stroke-width="0" viewBox="0 0 512 512" class="ms-3" color="#CBACF9" style="color:#CBACF9" height="1em" width="1em" xmlns="http://www.w3.org/2000/svg">
                    <path d="M444.52 3.52L28.74 195.42c-47.97 22.39-31.98 92.75 19.19 92.75h175.91v175.91c0 51.17 70.36 67.17 92.75 19.19l191.9-415.78c15.99-38.39-25.59-79.97-63.97-63.97z"></path>
                  </svg>
                </p>
              </div>
            </div>
          </div>
        </button>
      </a>
    }
  }
}