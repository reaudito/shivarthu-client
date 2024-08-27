use crate::components::schelling_game::department_funding::apply_jurors_sign_in::SignTransaction;
use crate::components::schelling_game::department_funding::change_period::ChangePeriod;
use crate::components::schelling_game::department_funding::rpc::staking_end_block::StakingEndBlock;
use crate::components::schelling_game::department_funding::storage::get_period::GetPeriod;
use crate::services::common_imp::View;
use crate::services::error::ErrorString;
use leptos::ev::SubmitEvent;
use leptos::*;

#[component]
pub fn ApplyJurors(department_required_fund_id: u64) -> impl IntoView {
    // gloo::console::log!(department_required_fund_id());
    let (current_view, set_current_view) = create_signal(View::Form);
    let (juror_stake, set_juror_stake) = create_signal::<Result<u128, ErrorString>>(Ok(0));
    let submit_click = move |e: SubmitEvent| {
        e.prevent_default();

        set_current_view(View::Success);
    };

    let stake_value = move |value: String| {
        let stake = value.parse::<u128>().expect("Invalid input");
        gloo::console::log!(stake);

        set_juror_stake(Ok(stake));
    };

    let render_view = move || match current_view() {
        View::Form => {
            view! {
                <div class="container mx-auto px-10">
                    <GetPeriod department_required_fund_id=department_required_fund_id.clone()/>
                    <StakingEndBlock department_required_fund_id=department_required_fund_id
                        .clone()/>
                    <ChangePeriod department_required_fund_id=department_required_fund_id.clone()/>
                    <form

                        id="apply-juror-submit-from"
                        on:submit=submit_click
                    >
                        <div class="mb-5">
                            <label
                                for="juror-stake"
                                class="block mb-2 text-sm font-medium text-gray-900 dark:text-white"
                            >
                                Juror Stake
                            </label>
                            <input
                                type="number"
                                id="juror-stake"
                                class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                                required
                                on:input=move |e| stake_value(event_target_value(&e))
                            />
                        </div>
                        <button
                            type="submit"
                            id="apply-juror-submit"
                            class="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm w-full sm:w-auto px-5 py-2.5 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800"
                        >

                            Submit
                        </button>
                    </form>
                </div>
            }
        }
        View::Success => {
            view! {
                <div>
                    <SignTransaction
                        stake=juror_stake().unwrap()
                        department_required_fund_id=department_required_fund_id.clone()
                    />

                </div>
            }
        }
    };

    view! { <div>{move || render_view()}</div> }
}
