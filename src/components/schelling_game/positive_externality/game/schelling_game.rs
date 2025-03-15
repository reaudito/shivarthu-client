use crate::components::navigation::nav::Nav;
use crate::components::schelling_game::positive_externality::apply_jurors::ApplyJurors;

use crate::components::schelling_game::positive_externality::commit_vote::CommitVote;
use crate::components::schelling_game::positive_externality::draw_jurors::DrawJurors;
use crate::components::schelling_game::positive_externality::reveal_vote::RevealVote;
use crate::components::schelling_game::positive_externality::storage::get_period_fn::get_period_fn;
use crate::services::common_services::polkadot::runtime_types::pallet_schelling_game_shared::types::Period;
use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

#[component]
pub fn SchellingGame() -> impl IntoView {
    let params = use_params_map();

    let user_to_calculate =
        move || params.with(|params| params.get("user_to_calculate").unwrap_or_default());

    let account = untrack(move || user_to_calculate());

    view! {
        <div>
            <SchellingGameComponent user_to_calculate={account} />
        </div>
    }
}

#[component]
pub fn SchellingGameComponent(user_to_calculate: String) -> impl IntoView {
    let user_to_calculate = move || user_to_calculate.clone();

    let period = get_period_fn(user_to_calculate());

    let myview = move || {
        {
            {
                // let period_read_signal = period();
                if let Some(period) = period() {
                    let view = match period {
                        Period::Evidence => view! {
                            <div>
                            </div>
                        }
                        .into_any(),
                        Period::Staking => view! {
                            <div>
                                <ApplyJurors user_to_calculate={user_to_calculate()} />
                            </div>
                        }
                        .into_any(),
                        Period::Drawing => view! {
                            <div>
                                <DrawJurors user_to_calculate={user_to_calculate()} />
                            </div>
                        }
                        .into_any(),
                        Period::Commit => view! {
                            <div>
                                <CommitVote user_to_calculate={user_to_calculate()} />
                            </div>
                        }
                        .into_any(),
                        Period::Vote => view! {
                            <div>
                                <RevealVote user_to_calculate={user_to_calculate()} />
                            </div>
                        }
                        .into_any(),
                        Period::Appeal => view! { <div></div> }.into_any(),
                        Period::Execution => {
                            view! { <div>You are in Execution phase. Get your incentives</div> }
                        }
                        .into_any(),
                    };
                    view
                } else {
                    view! {
                        <div class="container">
                            <p>{format!("{:?}", period())}</p>
                            <p>{"No period"}</p>
                        </div>
                    }
                    .into_any()
                }
            }
        }
    };

    view! {
        <div>
            <Nav />
            // {move || account()}
            // {move || format!("{:?}", period())}
            {move || myview()}
        </div>
    }
}
