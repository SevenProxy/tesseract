use yew::prelude::*;

pub struct LanguagensCardFront;

impl Component for LanguagensCardFront {
  type Message = ();
  type Properties = ();

  fn create(_: &Context<Self>) -> Self {
    LanguagensCardFront
  }

  fn view(&self, _: &Context<Self>) -> Html {
    html! {
      <div class="max-w-sm md:max-w-full w-full mx-auto p-8 rounded-xl bordergroup border border-white/[0.1] group-hover/pin:border-white/[0.2] flex flex-col items-center justify-center" style="background:rgb(2,0,36);background-color:linear-gradient(90deg, rgba(2,0,36,1) 0%, rgba(59,59,68,1) 50%, rgba(93,108,111,1) 100%)">
        <div class="h-[15rem] md:h-[20rem] rounded-xl z-40 bg-[rgba(40,40,40,0.70)] [mask-image:radial-gradient(50%_50%_at_50%_50%,white_0%,transparent_100%)]">
          <div class="p-8 overflow-hidden h-full relative flex items-center justify-center">
            <div class="flex flex-row flex-shrink-0 justify-center items-center gap-2">
            <div class="md:h-24 md:w-24 lg:h-10 lg:w-10 rounded-full flex items-center justify-center h-8 w-8 circle-1" style="background: rgb(2, 0, 36); transform: translateY(0px);">
              <svg xmlns="http://www.w3.org/2000/svg" class="lg:h-4 lg:w-4 md:h-4 md:w-4 sm:h-4 max-sm:w-4" viewBox="0 -17.5 256 256" preserveAspectRatio="xMinYMin meet">
                <path d="M204.8 0H256L128 220.8 0 0h97.92L128 51.2 157.44 0h47.36z" fill="#41B883"/>
                <path d="M0 0l128 220.8L256 0h-51.2L128 132.48 50.56 0H0z" fill="#41B883"/><path d="M50.56 0L128 133.12 204.8 0h-47.36L128 51.2 97.92 0H50.56z" fill="#35495E"/>
              </svg>
            </div>
            
            <div class="md:h-24 md:w-24 lg:h-10 lg:w-10 rounded-full flex items-center justify-center h-12 w-12 circle-2" style="background: rgb(2, 0, 36); transform: translateY(0px);">
              <svg width="24" height="24" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" class="lg:h-5 lg:w-5 md:h-7 md:w-7">
                <g clip-path="url(#clip0_3307_794)">
                  <path d="M20.4 0H3.6C1.61177 0 0 1.61177 0 3.6V20.4C0 22.3882 1.61177 24 3.6 24H20.4C22.3882 24 24 22.3882 24 20.4V3.6C24 1.61177 22.3882 0 20.4 0Z" fill="#3178C6"></path>
                  <path d="M10.9219 13.3126H13.9219V11.3908H5.53125V13.3126H8.53125V21.8908H10.9219V13.3126ZM14.8594 21.422C15.2391 21.6189 15.7031 21.7642 16.2187 21.8626C16.7344 21.9611 17.2969 22.0079 17.8594 22.0079C18.4219 22.0079 18.9375 21.9564 19.4531 21.8486C19.9687 21.7408 20.3906 21.5626 20.7656 21.3329C21.1453 21.0845 21.4687 20.7704 21.6562 20.3486C21.8437 19.9267 21.9891 19.4579 21.9891 18.8486C21.9891 18.422 21.9234 18.0517 21.7969 17.7236C21.6703 17.3954 21.4875 17.1142 21.2344 16.8798C20.9953 16.6314 20.7187 16.4111 20.3906 16.2236C20.0625 16.0361 19.6875 15.8392 19.2656 15.6611C18.9562 15.5345 18.7031 15.4126 18.4219 15.2908C18.1781 15.1689 17.9672 15.047 17.8125 14.9251C17.6391 14.7986 17.5078 14.6673 17.4141 14.5314C17.3203 14.3908 17.2734 14.2361 17.2734 14.0626C17.2734 13.9033 17.3152 13.7579 17.4 13.6267C17.4848 13.4954 17.6016 13.3876 17.7516 13.2939C17.9016 13.2001 18.0891 13.1298 18.3141 13.0783C18.5344 13.0267 18.7781 13.0033 19.0641 13.0033C19.2609 13.0033 19.4672 13.0178 19.6734 13.0473C19.8891 13.0768 20.1094 13.1223 20.3297 13.1833C20.55 13.2442 20.7656 13.3192 20.9859 13.4129C21.1922 13.5067 21.3844 13.6145 21.5484 13.7364V11.5333C21.1922 11.3973 20.7984 11.2942 20.3766 11.2286C19.9547 11.1629 19.4859 11.1301 18.9234 11.1301C18.3609 11.1301 17.8453 11.1911 17.3297 11.3083C16.8141 11.4254 16.3922 11.6129 16.0172 11.8708C15.6375 12.1239 15.3609 12.4333 15.1266 12.8551C14.9062 13.2489 14.7984 13.6989 14.7984 14.2614C14.7984 14.9645 15 15.5739 15.4078 16.0426C15.8109 16.5583 16.4391 16.9333 17.2359 17.3083C17.5594 17.4395 17.8453 17.5708 18.1266 17.6973C18.4078 17.8239 18.6422 17.9551 18.8297 18.0911C19.0312 18.227 19.1906 18.377 19.2984 18.5364C19.4156 18.6958 19.4766 18.8833 19.4766 19.0989C19.4766 19.2489 19.44 19.3895 19.3687 19.5208C19.2975 19.652 19.1859 19.7645 19.0359 19.8583C18.8859 19.952 18.7031 20.027 18.4734 20.0833C18.2531 20.1348 18.0047 20.1629 17.6766 20.1629C17.1609 20.1629 16.6453 20.0739 16.1766 19.8958C15.6609 19.7176 15.1922 19.4504 14.8594 19.172V21.422Z" fill="white"></path>
                </g>
                <defs>
                  <clipPath id="clip0_3307_794">
                    <rect width="24" height="24" fill="white"></rect>
                  </clipPath>
                </defs>
              </svg>
            </div>
            <div class="h-16 w-16 md:h-24 md:w-24 lg:h-10 lg:w-10 rounded-full flex items-center justify-center circle-3" style="background: rgb(2, 0, 36); transform: translateY(-0.41581px);">
              <svg width="27" height="24" viewBox="0 0 27 24" fill="none" xmlns="http://www.w3.org/2000/svg" class="lg:h-8 lg:w-8 md:h-10 md:w-10">
                <path d="M13.485 14.3832C14.8154 14.3832 15.894 13.3042 15.894 11.9731C15.894 10.642 14.8154 9.56299 13.485 9.56299C12.1545 9.56299 11.0759 10.642 11.0759 11.9731C11.0759 13.3042 12.1545 14.3832 13.485 14.3832Z" fill="#00D8FF"></path>
                <path d="M13.4849 17.4943C10.1017 17.4943 7.14555 17.0958 4.93326 16.3373C3.49839 15.8476 2.27947 15.1946 1.41087 14.4505C0.489487 13.6631 0 12.8037 0 11.9731C0 10.3792 1.7468 8.81889 4.67892 7.80107C7.07836 6.9657 10.2072 6.5 13.4801 6.5C16.6953 6.5 19.781 6.95129 22.1661 7.77707C23.5625 8.25717 24.7383 8.8813 25.5733 9.57265C26.4803 10.3312 26.9602 11.1618 26.9602 11.9731C26.9602 13.6295 25.007 15.305 21.8589 16.3469C19.6323 17.0862 16.6569 17.4943 13.4849 17.4943ZM13.4849 7.65224C10.38 7.65224 7.30871 8.10354 5.06283 8.8861C2.36585 9.8271 1.15653 11.1138 1.15653 11.9731C1.15653 12.8661 2.45703 14.2728 5.30757 15.2474C7.39989 15.9628 10.2312 16.3421 13.4849 16.3421C16.537 16.3421 19.3827 15.958 21.499 15.2522C24.4599 14.268 25.8132 12.8613 25.8132 11.9731C25.8132 11.5171 25.4677 10.9793 24.8391 10.456C24.1144 9.8511 23.0587 9.29899 21.7966 8.8621C19.5267 8.08433 16.5754 7.65224 13.4849 7.65224Z" fill="#00D8FF"></path>
                <path d="M8.01911 23.9852C7.52962 23.9852 7.09772 23.8796 6.733 23.6684C5.35572 22.8714 4.87583 20.5765 5.4565 17.5279C5.93159 15.0266 7.09292 12.0883 8.72934 9.25095C10.337 6.46636 12.2661 4.01785 14.1713 2.3615C15.2846 1.3917 16.4124 0.681146 17.4297 0.306667C18.5383 -0.101419 19.4981 -0.101419 20.1987 0.301866C21.6336 1.12764 22.1087 3.66257 21.4368 6.90806C20.9617 9.21254 19.8292 11.9923 18.2455 14.7433C16.5563 17.6719 14.7328 20.034 12.9716 21.5751C11.8294 22.5738 10.6537 23.3035 9.57875 23.6828C9.01728 23.8844 8.4894 23.9852 8.01911 23.9852ZM9.22363 9.53901L9.72271 9.82707C8.17267 12.5156 7.02574 15.401 6.58424 17.7439C6.05156 20.5525 6.56504 22.2425 7.30407 22.6698C7.48643 22.7754 7.72637 22.833 8.01911 22.833C8.97409 22.833 10.4761 22.2281 12.2133 20.711C13.8786 19.2563 15.6206 16.9902 17.2474 14.172C18.7734 11.5266 19.858 8.86687 20.3091 6.68241C20.9377 3.62416 20.3955 1.75177 19.6228 1.30528C19.2293 1.07963 18.5911 1.10844 17.8232 1.3917C16.9354 1.71816 15.9325 2.3567 14.9247 3.23528C13.1155 4.81001 11.268 7.15291 9.72271 9.83187L9.22363 9.53901Z" fill="#00D8FF"></path>
                <path d="M18.9511 24.0001C17.6458 24.0001 15.9902 23.2127 14.2626 21.7244C12.3334 20.0633 10.3707 17.5908 8.72946 14.7534C7.11704 11.9688 5.9653 9.07377 5.48061 6.59165C5.19748 5.14175 5.14469 3.81187 5.32705 2.74124C5.5286 1.5746 6.00369 0.744021 6.70913 0.335935C8.1392 -0.494639 10.5722 0.35994 13.0485 2.5636C14.8049 4.12393 16.6476 6.49563 18.2361 9.24181C19.9301 12.1704 21.0674 14.931 21.5185 17.2259C21.8112 18.7142 21.8592 20.0969 21.6529 21.2203C21.4321 22.4158 20.933 23.2656 20.2132 23.6832C19.8533 23.8945 19.4262 24.0001 18.9511 24.0001ZM9.72763 14.1772C11.2825 16.8658 13.2116 19.2999 15.016 20.8506C17.1803 22.7134 18.8983 23.1167 19.6421 22.6846C20.4148 22.2381 20.981 20.409 20.3956 17.4515C19.9637 15.2815 18.8743 12.6409 17.2427 9.82273C15.7118 7.17737 13.9507 4.9065 12.2854 3.42779C9.95318 1.35375 8.06242 0.888051 7.28979 1.33455C6.89629 1.56019 6.60355 2.13151 6.46439 2.93808C6.30602 3.86948 6.35401 5.06013 6.61315 6.3708C7.07385 8.7281 8.17759 11.4983 9.72763 14.1772Z" fill="#00D8FF"></path>
              </svg>
            </div>
            <div class="md:h-24 md:w-24 lg:h-10 lg:w-10 rounded-full flex items-center justify-center h-12 w-12 circle-4" style="background: rgb(2, 0, 36); transform: translateY(0px);">
              <svg width="25" height="25" viewBox="0 0 25 25" fill="none" xmlns="http://www.w3.org/2000/svg">
                <g clip-path="url(#clip0_3307_1736)">
                  <path d="M11.7433 0.280287C11.6905 0.285086 11.5225 0.30188 11.3713 0.313876C7.8848 0.628171 4.61902 2.50914 2.55061 5.40017C1.39883 7.00763 0.662164 8.83102 0.383817 10.7624C0.285435 11.4365 0.273438 11.6357 0.273438 12.5498C0.273438 13.4639 0.285435 13.663 0.383817 14.3372C1.05089 18.946 4.33107 22.8183 8.77984 24.2531C9.57649 24.5098 10.4163 24.6849 11.3713 24.7905C11.7433 24.8313 13.351 24.8313 13.7229 24.7905C15.3714 24.6081 16.7679 24.2003 18.1453 23.4973C18.3564 23.3893 18.3972 23.3606 18.3684 23.3366C18.3492 23.3222 17.4494 22.1154 16.3696 20.6567L14.4068 18.0055L11.9472 14.366C10.5939 12.365 9.4805 10.7288 9.47091 10.7288C9.46131 10.7264 9.45171 12.3434 9.44691 14.318C9.43971 17.7752 9.43731 17.9144 9.39412 17.9959C9.33173 18.1135 9.28374 18.1615 9.18296 18.2143C9.10618 18.2527 9.03899 18.2599 8.67666 18.2599H8.26153L8.15115 18.1903C8.07917 18.1447 8.02638 18.0847 7.99039 18.0151L7.93999 17.9072L7.94479 13.0968L7.95199 8.284L8.02638 8.19044C8.06477 8.14005 8.14636 8.07527 8.20394 8.04409C8.30233 7.9961 8.34072 7.9913 8.75584 7.9913C9.24535 7.9913 9.32693 8.0105 9.45411 8.14965C9.4901 8.18804 10.8219 10.1938 12.4152 12.6098C14.0085 15.0257 16.1872 18.3246 17.2574 19.9441L19.2011 22.8879L19.2995 22.8231C20.1705 22.2569 21.0919 21.4508 21.8214 20.6111C23.3739 18.8285 24.3745 16.6548 24.7104 14.3372C24.8088 13.663 24.8208 13.4639 24.8208 12.5498C24.8208 11.6357 24.8088 11.4365 24.7104 10.7624C24.0434 6.15352 20.7632 2.28122 16.3144 0.846498C15.5298 0.592183 14.6947 0.417042 13.7589 0.311477C13.5285 0.287485 11.9424 0.261094 11.7433 0.280287ZM16.7679 7.7034C16.8831 7.76098 16.9767 7.87134 17.0103 7.9865C17.0295 8.04888 17.0343 9.38284 17.0295 12.389L17.0223 16.7028L16.2616 15.5368L15.4986 14.3708V11.235C15.4986 9.2077 15.5082 8.06808 15.5226 8.0129C15.561 7.87854 15.6449 7.77298 15.7601 7.7106C15.8585 7.66021 15.8945 7.65542 16.2712 7.65542C16.6264 7.65542 16.6887 7.66021 16.7679 7.7034Z" fill="white"></path>
                </g>
                <defs>
                  <clipPath id="clip0_3307_1736">
                    <rect width="24.5474" height="24.5474" fill="white" transform="translate(0.273438 0.273682)"></rect>
                  </clipPath>
                </defs>
              </svg>
            </div>
            <div class="md:h-24 md:w-24 lg:h-10 lg:w-10 rounded-full flex items-center justify-center h-8 w-8 circle-5" style="background: rgb(2, 0, 
            36); transform: translateY(0px);">
              <svg width="27" height="16" viewBox="0 0 27 16" fill="none" xmlns="http://www.w3.org/2000/svg">
                <path fill-rule="evenodd" clip-rule="evenodd" d="M13.5164 0.251465C10.0939 0.251465 7.9553 1.97348 7.09928 5.41752C8.38281 3.6955 9.8801 3.04987 11.5911 3.48012C12.5674 3.72554 13.2655 4.43838 14.0375 5.2276C15.2959 6.51275 16.7524 8.00004 19.9331 8.00004C23.3552 8.00004 25.4942 6.27802 26.3498 2.83449C25.0667 4.55651 23.5694 5.20214 21.8584 4.77138C20.8817 4.52596 20.1836 3.81312 19.4115 3.02441C18.1532 1.73825 16.6971 0.251465 13.5164 0.251465ZM7.09928 8.00004C3.67719 8.00004 1.53814 9.72205 0.682617 13.1661C1.96615 11.4441 3.46344 10.7984 5.17398 11.2287C6.15071 11.4741 6.84881 12.187 7.62084 12.9762C8.87922 14.2613 10.3353 15.7486 13.5164 15.7486C16.9385 15.7486 19.0776 14.0271 19.9331 10.5831C18.6496 12.3051 17.1523 12.9507 15.4412 12.5205C14.465 12.2745 13.7669 11.5617 12.9949 10.773C11.7365 9.48783 10.2799 8.00004 7.09928 8.00004Z" fill="#38BDF8"></path>
              </svg>
            </div>
          </div>
          <div class="h-40 w-px absolute top-20 m-auto z-40 bg-gradient-to-b from-transparent via-cyan-500 to-transparent animate-move"><div class="w-10 h-32 top-1/2 -translate-y-1/2 absolute -left-10">
            <div class="absolute inset-0">
                <span class="inline-block bg-black dark:bg-white" style="position: absolute; top: 55.7611px; left: 25.4556px; width: 2px; height: 2px; border-radius: 50%; z-index: 1; opacity: 1; transform: scale(0.7575) translateZ(0px);"></span>
                <span class="inline-block bg-black dark:bg-white" style="position: absolute; top: 45.528px; left: 25.7869px; width: 2px; height: 2px; border-radius: 50%; z-index: 1; opacity: 1; transform: scale(1.09971) translateZ(0px);"></span>
                <span class="inline-block bg-black dark:bg-white" style="position: absolute; top: 112.273px; left: 24.641px; width: 2px; height: 2px; border-radius: 50%; z-index: 1; opacity: 1; transform: scale(1.12555) translateZ(0px);"></span>
                <span class="inline-block bg-black dark:bg-white" style="position: absolute; top: 84.9973px; left: 2.94476px; width: 2px; height: 2px; border-radius: 50%; z-index: 1; opacity: 1; transform: scale(1.04626) translateZ(0px);"></span>
                <span class="inline-block bg-black dark:bg-white" style="position: absolute; top: 64.9557px; left: 21.4321px; width: 2px; height: 2px; border-radius: 50%; z-index: 1; opacity: 1; transform: scale(0.875982) translateZ(0px);"></span>
                <span class="inline-block bg-black dark:bg-white" style="position: absolute; top: 102.72px; left: 27.3998px; width: 2px; height: 2px; border-radius: 50%; z-index: 1; opacity: 1; transform: scale(1.13989) translateZ(0px);"></span>
                <span class="inline-block bg-black dark:bg-white" style="position: absolute; top: 69.5924px; left: 29.3426px; width: 2px; height: 2px; border-radius: 50%; z-index: 1; opacity: 1; transform: scale(1.05795) translateZ(0px);"></span>
                <span class="inline-block bg-black dark:bg-white" style="position: absolute; top: 98.4754px; left: 22.9847px; width: 2px; height: 2px; border-radius: 50%; z-index: 1; opacity: 1; transform: scale(1.1867) translateZ(0px);"></span>
                <span class="inline-block bg-black dark:bg-white" style="position: absolute; top: 116.966px; left: 21.6758px; width: 2px; height: 2px; border-radius: 50%; z-index: 1; opacity: 1; transform: scale(1.05136) translateZ(0px);"></span>
                <span class="inline-block bg-black dark:bg-white" style="position: absolute; top: 77.5791px; left: 36.2307px; width: 2px; height: 2px; border-radius: 50%; z-index: 1; opacity: 1; transform: scale(0.127453) translateZ(0px);"></span>
                <span class="inline-block bg-black dark:bg-white" style="position: absolute; top: 113.101px; left: 29.0703px; width: 2px; height: 2px; border-radius: 50%; z-index: 1; opacity: 1; transform: scale(0.640048) translateZ(0px);"></span>
                <span class="inline-block bg-black dark:bg-white" style="position: absolute; top: 100.776px; left: 10.6157px; width: 2px; height: 2px; border-radius: 50%; z-index: 1; opacity: 1; transform: scale(1.10346) translateZ(0px);"></span>
            </div>
          </div>
        </div>
        </div>
      </div>
      <h3 class="lg:text-lg font-semibold text-white md:text-3xl py-2">{"Front-end"}</h3>
      <p class="lg:text-sm md:text-xl font-normal text-neutral-400 max-w-sm">{"Linguagens para Desenvolvimento WEB, com Conceitos Modernos de UI/UX."}</p>
    </div>
    }
  }

}