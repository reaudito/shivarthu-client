use leptos::ev::SubmitEvent;
use leptos::*;
use leptos_router::*;

#[component]
pub fn ChangePeriod() -> impl IntoView {
    let params = use_params_map();
    let navigate = leptos_router::use_navigate();

    
    let user_to_calculate = move || {
        params.with(|params| {
            params
                .get("user_to_calculate")
                .cloned()
                .unwrap_or_default()
        })
    };
    

    
    let submit_click = move |e: SubmitEvent| {
        e.prevent_default();
        navigate(
            &format!("positive-externality-validation-change-period/{}", user_to_calculate()),
            Default::default(),
        );
    };

    view! {
        <div>
            <form
                class="max-w-5xl mx-auto max-md:mx-10"
                id="apply-juror-submit-from"
                on:submit=submit_click
            >
                <button
                    type="submit"
                    class="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm w-full sm:w-auto px-5 py-2.5 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800"
                >

                    Change Period
                </button>
            </form>
        </div>
    }
}