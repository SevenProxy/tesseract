use yew::prelude::*;

pub struct FooterInformation;

impl Component for FooterInformation {
  type Message = ();
  type Properties = ();

  fn create(_: &Context<Self>) -> Self {
    FooterInformation
  }

  fn view(&self, _: &Context<Self>) -> Html {
    html! {
      <div class="flex space-y-5 mt-16 md:mb-5 md:flex-row flex-col justify-between items-center">
        <div class="flex items-center md:gap-3 gap-6">
          <a href="linkedin.com/in/deivid-braga-086601264/" target="_blank" class="w-10 h-10 cursor-pointer flex justify-center items-center backdrop-filter backdrop-blur-lg saturate-180 bg-opacity-75 bg-black-200 rounded-lg border border-black-300">
            <img src="/public/link.svg" alt="icons" width="20" height="20" />
          </a>
          <a href="https://github.com/Cardosofiles" target="_blank" class="w-10 h-10 cursor-pointer flex justify-center items-center backdrop-filter backdrop-blur-lg saturate-180 bg-opacity-75 bg-black-200 rounded-lg border border-black-300">
            <img src="/public/git.svg" alt="icons" width="20" height="20" />
          </a>
          <a href="discord.com/users/1193791291125940309" target="_blank" class="w-10 h-10 cursor-pointer flex justify-center items-center backdrop-filter backdrop-blur-lg saturate-180 bg-opacity-75 bg-black-200 rounded-lg border border-black-300">
            <img src="/public/discord.svg" alt="icons" width="20" height="20" />
          </a>
        </div>
        <p class="md:text-base text-sm md:mt-4 md:font-normal font-light text-white">{"Copyright © 2024 Seven Proxy"}</p>
      </div>
    }
  }
}