use crate::components::navigation::nav::Nav;
use crate::components::schelling_game::department_funding::rpc::juror_selected::JurorSelected;
use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

#[component]
pub fn JurorSelectedCheck() -> impl IntoView {
    let params = use_params_map();

    let department_required_fund_id = move || {
        params.with(|params| {
            params
                .get("department_required_fund_id")
                .and_then(|value| value.parse::<u64>().ok())
                .unwrap_or_default()
        })
    };

    let (check_account, set_check_account) = signal(String::from(""));

    let account = untrack(move || department_required_fund_id());

    let on_account = move |ev| {
        let account_value = event_target_value(&ev);
        set_check_account(account_value);
    };

    view! {
        <div>
            <Nav />
            <div class="max-w-5xl mx-auto max-md:mx-10">
                <h1>Check if an account selected as juror:</h1>
                <br />
                <input
                    type="text"
                    placeholder="Enter account address here"
                    id="juror-address-checking"
                    class="input input-bordered w-full max-w-xs"
                    on:input={on_account}
                />
                <br />
                <br />
                <JurorSelected
                    department_required_fund_id={account}
                    check_account={check_account}
                />
            </div>
        </div>
    }
}
