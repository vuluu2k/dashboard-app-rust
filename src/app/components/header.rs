use leptos::prelude::*;

const INPUT_STYLE: &str = "border-b-0 border-[#7734e7] h-8 text-white mx-4 hover:border-b-2";
const INPUT_STYLE_SELECTED: &str =
	"border-b-2 border-[#9734e7] h-8 text-white mx-4 hover:border-b-2";

/// Documentation for [`Header`]
#[component]
pub fn Header() -> impl IntoView {
	let (current_path, set_current_path) = signal(String::new());

	view! {
    <div class="header">
      <h1>"Header"</h1>
    </div>
  }
}
