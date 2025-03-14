use crate::components::login::get_login_account::AccountState;
use codee::string::JsonSerdeCodec;
use icondata;
use leptos::html::Button;
use leptos::prelude::*;
use leptos_icons::*;
use leptos_use::storage::use_local_storage;
use leptos_use::{
    use_clipboard_with_options, use_permission, UseClipboardOptions, UseClipboardReturn,
};
use leptos_use::{use_element_bounding, UseElementBoundingReturn};

#[component]
pub fn Nav() -> impl IntoView {
    let el = create_node_ref::<Button>();
    let UseElementBoundingReturn { x, y, .. } = use_element_bounding(el);
    let (drop_down, set_drop_down) = signal(false);
    let (nav_multi_level, set_nav_multi_level) = signal(false);
    let (account_state, set_account_state, reset_account) =
        use_local_storage::<AccountState, JsonSerdeCodec>("account-state");
    let UseClipboardReturn {
        is_supported,
        text,
        copied,
        copy,
    } = use_clipboard_with_options(UseClipboardOptions::default().read(true));

    view! {
        <>
            <nav class="bg-white border-gray-200 dark:bg-gray-900 dark:border-gray-700">
                <div class="max-w-screen-xl flex flex-wrap items-center justify-between mx-auto p-4">
                    <a href="#" class="flex items-center space-x-3 rtl:space-x-reverse">
                        <a class="btn btn-ghost text-xl dark:text-white">"Shivarthu"</a>
                    </a>
                    <button
                        on:click={move |_| {
                            set_nav_multi_level.update(|n| *n = !*n);
                        }}

                        data-collapse-toggle="navbar-multi-level"
                        type="button"
                        class="inline-flex items-center p-2 w-10 h-10 justify-center text-sm text-gray-500 rounded-lg lg:hidden hover:bg-gray-100 focus:outline-none focus:ring-2 focus:ring-gray-200 dark:text-gray-400 dark:hover:bg-gray-700 dark:focus:ring-gray-600"
                        aria-controls="navbar-multi-level"
                        aria-expanded="false"
                    >
                        <span class="sr-only">"Open main menu"</span>
                        <svg
                            class="w-5 h-5"
                            aria-hidden="true"
                            xmlns="http://www.w3.org/2000/svg"
                            fill="none"
                            viewBox="0 0 17 14"
                        >
                            <path
                                stroke="currentColor"
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                stroke-width="2"
                                d="M1 1h15M1 7h15M1 13h15"
                            ></path>
                        </svg>
                    </button>
                    <div class={move || {
                        if nav_multi_level() == true {
                            "w-full lg:block lg:w-auto text-3xl"
                        } else {
                            "hidden w-full lg:block lg:w-auto"
                        }
                    }}>
                        <ul class="flex flex-col font-medium p-4 lg:p-0 mt-4 border border-gray-100 rounded-lg bg-gray-50 lg:space-x-8 rtl:space-x-reverse lg:flex-row lg:mt-0 lg:border-0 lg:bg-white dark:bg-gray-800 lg:dark:bg-gray-900 dark:border-gray-700">
                            <li class={move || {
                                let base_classes = "py-2 px-3 text-white bg-blue-700 rounded lg:bg-transparent lg:text-blue-700 lg:p-0 lg:dark:text-blue-500 dark:bg-blue-600 lg:dark:bg-transparent";
                                let height_class = if nav_multi_level() == true {
                                    "h-24 flex justify-start items-center"
                                } else {
                                    ""
                                };
                                format!("{} {}", base_classes, height_class)
                            }}>

                                <a
                                    href="/"

                                    aria-current="page"
                                >
                                    "Home"
                                </a>
                            </li>
                            <li class={move || {
                                let base_classes = "";
                                let height_class = if nav_multi_level() == true {
                                    "h-24 flex justify-start items-center"
                                } else {
                                    ""
                                };
                                format!("{} {}", base_classes, height_class)
                            }}>
                                <button
                                    node_ref={el}
                                    on:click={move |_| {
                                        set_drop_down.update(|n| *n = !*n);
                                    }}

                                    id="dropdownNavbarLink"
                                    data-dropdown-toggle="dropdownNavbar"
                                    class="flex items-center justify-between w-full py-2 px-3 text-gray-900 hover:bg-gray-100 lg:hover:bg-transparent lg:border-0 lg:hover:text-blue-700 lg:p-0 lg:w-auto dark:text-white lg:dark:hover:text-blue-500 dark:focus:text-white dark:hover:bg-gray-700 lg:dark:hover:bg-transparent"
                                >
                                    "Dropdown"
                                    <svg
                                        class="w-2.5 h-2.5 ms-2.5"
                                        aria-hidden="true"
                                        xmlns="http://www.w3.org/2000/svg"
                                        fill="none"
                                        viewBox="0 0 10 6"
                                    >
                                        <path
                                            stroke="currentColor"
                                            stroke-linecap="round"
                                            stroke-linejoin="round"
                                            stroke-width="2"
                                            d="m1 1 4 4 4-4"
                                        ></path>
                                    </svg>
                                </button>
                                <div
                                    id="dropdownNavbar"
                                    class={move || {
                                        if drop_down() == true {
                                            "z-10 font-normal bg-white divide-y divide-gray-100 rounded-lg shadow w-44 dark:bg-gray-700 dark:divide-gray-600   block"
                                        } else {
                                            "z-10 hidden font-normal bg-white divide-y divide-gray-100 rounded-lg shadow w-44 dark:bg-gray-700 dark:divide-gray-600"
                                        }
                                    }}

                                    style={move || {
                                        if drop_down() == true {
                                            format!(
                                                "position: absolute; inset: 0px auto auto 0px; margin: 0px; transform: translate({}px, {}px);",
                                                x(),
                                                y() + 40.0,
                                            )
                                        } else {
                                            format!(
                                                "position: absolute; inset: 0px auto auto 0px; margin: 0px; transform: translate({}px, {}px);",
                                                x() + 88.0,
                                                y(),
                                            )
                                        }
                                    }}
                                >

                                    <ul
                                        class="py-2 text-sm text-gray-700 dark:text-gray-200"
                                        aria-labelledby="dropdownLargeButton"
                                    >
                                        <li>
                                            <a
                                                href="#"
                                                class="block px-4 py-2 hover:bg-gray-100 dark:hover:bg-gray-600 dark:hover:text-white"
                                            >
                                                "Dashboard"
                                            </a>
                                        </li>
                                        <li aria-labelledby="dropdownNavbarLink">
                                            <button
                                                id="doubleDropdownButton"
                                                data-dropdown-toggle="doubleDropdown"
                                                data-dropdown-placement="right-start"
                                                type="button"
                                                class="flex items-center justify-between w-full px-4 py-2 hover:bg-gray-100 dark:hover:bg-gray-600 dark:hover:text-white"
                                            >
                                                "Dropdown"
                                                <svg
                                                    class="w-2.5 h-2.5 ms-2.5"
                                                    aria-hidden="true"
                                                    xmlns="http://www.w3.org/2000/svg"
                                                    fill="none"
                                                    viewBox="0 0 10 6"
                                                >
                                                    <path
                                                        stroke="currentColor"
                                                        stroke-linecap="round"
                                                        stroke-linejoin="round"
                                                        stroke-width="2"
                                                        d="m1 1 4 4 4-4"
                                                    ></path>
                                                </svg>
                                            </button>
                                            <div
                                                id="doubleDropdown"
                                                class="z-10 hidden bg-white divide-y divide-gray-100 rounded-lg shadow w-44 dark:bg-gray-700"
                                            >
                                                <ul
                                                    class="py-2 text-sm text-gray-700 dark:text-gray-200"
                                                    aria-labelledby="doubleDropdownButton"
                                                >
                                                    <li>
                                                        <a
                                                            href="#"
                                                            class="block px-4 py-2 hover:bg-gray-100 dark:hover:bg-gray-600 dark:text-gray-200 dark:hover:text-white"
                                                        >
                                                            "Overview"
                                                        </a>
                                                    </li>
                                                    <li>
                                                        <a
                                                            href="#"
                                                            class="block px-4 py-2 hover:bg-gray-100 dark:hover:bg-gray-600 dark:text-gray-200 dark:hover:text-white"
                                                        >
                                                            "My downloads"
                                                        </a>
                                                    </li>
                                                    <li>
                                                        <a
                                                            href="#"
                                                            class="block px-4 py-2 hover:bg-gray-100 dark:hover:bg-gray-600 dark:text-gray-200 dark:hover:text-white"
                                                        >
                                                            "Billing"
                                                        </a>
                                                    </li>
                                                    <li>
                                                        <a
                                                            href="#"
                                                            class="block px-4 py-2 hover:bg-gray-100 dark:hover:bg-gray-600 dark:text-gray-200 dark:hover:text-white"
                                                        >
                                                            "Rewards"
                                                        </a>
                                                    </li>
                                                </ul>
                                            </div>
                                        </li>
                                        <li>
                                            <a
                                                href="#"
                                                class="block px-4 py-2 hover:bg-gray-100 dark:hover:bg-gray-600 dark:hover:text-white"
                                            >
                                                "Earnings"
                                            </a>
                                        </li>
                                    </ul>
                                    <div class="py-1">
                                        <a
                                            href="#"
                                            class="block px-4 py-2 text-sm text-gray-700 hover:bg-gray-100 dark:hover:bg-gray-600 dark:text-gray-200 dark:hover:text-white"
                                        >
                                            "Sign out"
                                        </a>
                                    </div>
                                </div>
                            </li>
                            <li class={move || {
                                let base_classes = "block py-2 px-3 text-gray-900 rounded hover:bg-gray-100 lg:hover:bg-transparent lg:border-0 lg:hover:text-blue-700 lg:p-0 dark:text-white lg:dark:hover:text-blue-500 dark:hover:bg-gray-700 dark:hover:text-white lg:dark:hover:bg-transparent";
                                let height_class = if nav_multi_level() == true {
                                    "h-24 flex justify-start items-center"
                                } else {
                                    ""
                                };
                                format!("{} {}", base_classes, height_class)
                            }}>
                                <a href="/add-profile">"Add Profile"</a>
                            </li>
                            <li class={move || {
                                let base_classes = "block py-2 px-3 text-gray-900 rounded hover:bg-gray-100 lg:hover:bg-transparent lg:border-0 lg:hover:text-blue-700 lg:p-0 dark:text-white lg:dark:hover:text-blue-500 dark:hover:bg-gray-700 dark:hover:text-white lg:dark:hover:bg-transparent";
                                let height_class = if nav_multi_level() == true {
                                    "h-24 flex justify-start items-center"
                                } else {
                                    ""
                                };
                                format!("{} {}", base_classes, height_class)
                            }}>
                                <a href="/positive-externality/create-post">

                                    "Positive Externality"
                                </a>
                            </li>
                            <li class={move || {
                                let base_classes = "block py-2 px-3 text-gray-900 rounded hover:bg-gray-100 lg:hover:bg-transparent lg:border-0 lg:hover:text-blue-700 lg:p-0 dark:text-white lg:dark:hover:text-blue-500 dark:hover:bg-gray-700 dark:hover:text-white lg:dark:hover:bg-transparent";
                                let height_class = if nav_multi_level() == true {
                                    "h-24 flex justify-start items-center"
                                } else {
                                    ""
                                };
                                format!("{} {}", base_classes, height_class)
                            }}>
                                <a href="/create-department">"Create Department"</a>
                            </li>
                            <li class={move || {
                                let base_classes = "block py-2 px-3 text-gray-900 rounded hover:bg-gray-100 lg:hover:bg-transparent lg:border-0 lg:hover:text-blue-700 lg:p-0 dark:text-white lg:dark:hover:text-blue-500 dark:hover:bg-gray-700 dark:hover:text-white lg:dark:hover:bg-transparent";
                                let height_class = if nav_multi_level() == true {
                                    "h-24 flex justify-start items-center"
                                } else {
                                    ""
                                };
                                format!("{} {}", base_classes, height_class)
                            }}>
                                <a href="#">"Department Funding"</a>
                            </li>
                            // <li class=move || {
                            // let base_classes = "block py-2 px-3 text-gray-900 rounded hover:bg-gray-100 lg:hover:bg-transparent lg:border-0 lg:hover:text-blue-700 lg:p-0 dark:text-white lg:dark:hover:text-blue-500 dark:hover:bg-gray-700 dark:hover:text-white lg:dark:hover:bg-transparent";
                            // let height_class = if nav_multi_level() == true {
                            // "h-24 flex justify-start items-center"
                            // } else {
                            // ""
                            // };
                            // format!("{} {}", base_classes, height_class)
                            // }>
                            // <a href="#" class="">
                            // "Create Project"
                            // </a>
                            // </li>
                            <li>
                                {move || {
                                    let full_id = account_state().account_id.clone();
                                    let shortened_id = if full_id.len() > 8 {
                                        format!(
                                            "{}...{}",
                                            &full_id[..8],
                                            &full_id[full_id.len() - 4..],
                                        )
                                    } else {
                                        full_id.clone()
                                    };
                                    if !shortened_id.is_empty() {
                                        view! {
                                            // Display the shortened account ID
                                            <>
                                                <span>{shortened_id}</span>

                                                <button on:click={
                                                    let copy = copy.clone();
                                                    move |_| copy(&full_id)
                                                }>
                                                    <Show
                                                        when={copied}
                                                        fallback={|| {
                                                            view! { <Icon icon={icondata::AiCopyOutlined} /> }
                                                        }}
                                                    >

                                                        Copied!
                                                    </Show>
                                                </button>
                                            </>
                                        }
                                    } else {
                                        view! {
                                            // Display the shortened account ID

                                            // Display the shortened account ID

                                            // Display the shortened account ID

                                            // Display the shortened account ID

                                            // Display the shortened account ID

                                            // Display the shortened account ID

                                            <>
                                                <div></div>
                                            </>
                                        }
                                    }
                                }}

                            </li>
                        </ul>
                    </div>
                </div>
            </nav>
        </>
    }
}
