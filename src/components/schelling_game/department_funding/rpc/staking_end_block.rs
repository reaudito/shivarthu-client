use crate::constants::constant::NODE_URL;
use icondata;
use jsonrpsee_core::{client::ClientT, rpc_params};
use jsonrpsee_wasm_client::WasmClientBuilder;
use leptos::prelude::*;
use leptos_icons::*;
use leptos_use::use_interval_fn;
use leptos_use::utils::Pausable;

async fn load_data(department_required_fund_id: u64, set_end_period: WriteSignal<Option<u32>>) {
    let client = WasmClientBuilder::default().build(NODE_URL).await.unwrap();
    let result: Option<u32> = client
        .request(
            "departmentfunding_stakingperiodendblock",
            rpc_params![department_required_fund_id],
        )
        .await
        .unwrap();
    set_end_period(result);
}

#[component]
pub fn StakingEndBlock(department_required_fund_id: u64) -> impl IntoView {
    let (end_period, set_end_period) = signal::<Option<u32>>(None);

    let action: Action<(u64, WriteSignal<Option<u32>>), (), LocalStorage> = Action::new_unsync(
        |(department_required_fund_id, set_end_period): &(u64, WriteSignal<Option<u32>>)| {
            let department_required_fund_id = department_required_fund_id.clone();
            let set_end_period = set_end_period.clone();
            async move { load_data(department_required_fund_id, set_end_period).await }
        },
    );

    let Pausable { .. } = use_interval_fn(
        move || {
            action.dispatch((department_required_fund_id.clone(), set_end_period));
        },
        5000,
    );

    view! {
        <div>
            {move || {
                if end_period().is_some() {
                    view! {
                        <div>
                            {"Staking Period ends: "}
                            <span id="end-period-time">{move || end_period()}</span>
                        </div>
                    }
                        .into_any()
                } else {
                    view! {
                        <div>
                            {"Staking Period ends: "} <span id="end-period-time">
                                <Icon icon={icondata::ImSpinner6} style="color: green" />
                            </span>
                        </div>
                    }
                        .into_any()
                }
            }}

        </div>
    }
}
