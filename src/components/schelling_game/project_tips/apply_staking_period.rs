use crate::components::navigation::nav::Nav;
use crate::components::schelling_game::project_tips::apply_staking_period_sign_in::SignTransaction;
use crate::services::common_imp::View;
use crate::services::error::ErrorString;
use leptos::ev::SubmitEvent;
use leptos::prelude::*;

#[component]
pub fn ApplyStakingPeriod(project_id: u64) -> impl IntoView {
    // gloo::console::log!(project_id());
    let (current_view, set_current_view) = signal(View::Form);
    let submit_click = move |e: SubmitEvent| {
        e.prevent_default();

        set_current_view(View::Success);
    };

    let render_view = move || {
        match current_view() {
        View::Form => {
            view! {
                <div>
                    <Nav />
                    <div class="max-w-5xl mx-auto max-md:mx-10">
                        <form id="apply-staking-period-from" on:submit={submit_click}>
                            <div class="dark:text-white text-gray-800">Apply Staking Period</div>
                            <button
                                type="submit"
                                id="apply-juror-submit"
                                class="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm w-full sm:w-auto px-5 py-2.5 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800"
                            >

                                Submit
                            </button>
                        </form>
                    </div>
                </div>
            }.into_any()
        }
        View::Success => {
            view! {
                <div>
                    <SignTransaction project_id={project_id.clone()} />

                </div>
            }.into_any()
        }
    }
    };

    view! { <div>{move || render_view()}</div> }
}
