use dioxus::prelude::*;

use crate::utils;

#[inline_props]
pub fn CallPage(cx: Scope) -> Element {
    let call_options = [
        ("Informacje ogólne", "/call"),
        ("Wyszukaj rozmowę", "/call/search"),
        ("Dodaj rozmowę", "/call/add"),
    ];

    cx.render(rsx! {
        div { class: "text-background flex border-spacing-2 flex-col items-center pt-5 pl-5",
            utils::MinimalMenu {
                image: r#"          <svg class="mt-4 h-16 w-full fill-inherit stroke-inherit" viewBox="0 -0.5 17 17" version="1.1" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" class="si-glyph si-glyph-phone-fax">
            <g id="SVGRepo_bgCarrier" stroke-width="0"></g>
            <g id="SVGRepo_tracerCarrier" stroke-linecap="round" stroke-linejoin="round"></g>
            <g id="SVGRepo_iconCarrier">
              <title>1071</title>
              <defs></defs>
              <g stroke="none" stroke-width="1" fill-rule="evenodd">
                <g transform="translate(1.000000, 0.000000)">
                  <path d="M2.964,13.945 C2.964,14.51 2.488,14.968 1.901,14.968 L1.094,14.968 C0.508,14.968 0.032,14.51 0.032,13.945 L0.032,1.054 C0.032,0.49 0.509,0.031 1.094,0.031 L1.901,0.031 C2.489,0.031 2.964,0.49 2.964,1.054 L2.964,13.945 L2.964,13.945 Z" class="si-glyph-fill"></path>
                  <path d="M14.881,0 L5.034,0 C4.463,0 4,0.459 4,1.023 L4,13.914 C4,14.479 4.463,14.937 5.034,14.937 L5.958,14.937 L5.958,12.916 L14,12.916 L14,14.937 L14.881,14.937 C15.454,14.937 15.916,14.479 15.916,13.914 L15.916,1.023 C15.916,0.459 15.454,0 14.881,0 L14.881,0 Z M7,12.021 L5.969,12.021 L5.969,10.968 L7,10.968 L7,12.021 L7,12.021 Z M7,10.021 L5.969,10.021 L5.969,8.968 L7,8.968 L7,10.021 L7,10.021 Z M7,8.021 L5.969,8.021 L5.969,6.984 L7,6.984 L7,8.021 L7,8.021 Z M9,12.021 L7.969,12.021 L7.969,10.968 L9,10.968 L9,12.021 L9,12.021 Z M9,10.021 L7.969,10.021 L7.969,8.953 L9,8.953 L9,10.021 L9,10.021 Z M9,8.021 L7.969,8.021 L7.969,6.968 L9,6.968 L9,8.021 L9,8.021 Z M11,12.021 L9.969,12.021 L9.969,10.984 L11,10.984 L11,12.021 L11,12.021 Z M11,10.021 L9.969,10.021 L9.969,8.984 L11,8.984 L11,10.021 L11,10.021 Z M11,8.021 L9.969,8.021 L9.969,6.953 L11,6.953 L11,8.021 L11,8.021 Z M14.016,8.975 L14.016,10.022 L11.969,10.022 L11.969,8.975 L14.016,8.975 L14.016,8.975 Z M11.969,8.021 L11.969,6.959 L14.016,6.959 L14.016,8.021 L11.969,8.021 L11.969,8.021 Z M14.016,4.031 L5.969,4.031 L5.969,1.969 L14.016,1.969 L14.016,4.031 L14.016,4.031 Z" class="si-glyph-fill"></path>
                  <path d="M12.969,15.969 L12.967,14.022 L7.018,14.022 L7.02,15.969 L12.969,15.969 L12.969,15.969 Z" class="si-glyph-fill"></path>
                </g>
              </g>
            </g>
          </svg>"#,
                options: call_options.into()
            }
        }
    })
}