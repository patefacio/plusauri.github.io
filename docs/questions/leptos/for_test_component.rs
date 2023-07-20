//! Module for for_test_component leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
#[allow(unused_imports)]
use leptos::log;
use leptos::{component, view, IntoView, Scope};
#[allow(unused_imports)]
use leptos_dom::console_log;

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// Used to figure out how the <For...> component works
///
///   * **cx** - Context
///   * **updatable** - The collection of data
///   * _return_ - View for for_test_component
#[component]
pub fn ForTestComponent(
    /// Context
    cx: Scope,
    /// The collection of data
    updatable: Vec<String>,
) -> impl IntoView {
    // α <fn for_test_component>

    use leptos::create_signal;
    use leptos::create_rw_signal;
    use leptos::For;
    use leptos::SignalWith;
    use leptos::SignalGet;
    use leptos::SignalUpdate;

    #[derive(Debug, Clone)]
    struct Data {
        key: String,
        value: String,
    }

    let (strings, set_strings) = create_signal(
        cx,
        updatable
            .iter()
            .map(|s| {
                create_rw_signal(cx, Data {
                    key: s.clone(),
                    value: s.clone(),
                })
            })
            .collect::<Vec<_>>(),
    );

    let (i, set_i) = create_signal(cx, 1);

    view! {
        cx,
        <ul>
        <For
            each=move || strings.get()
            key=|s| s.with(|datum| datum.key.clone())
            view = move |cx, datum| {
                log!("RERENDERING {datum:?}");
                view! {
                    cx,
                    <li>
                    <button
                        on:click= move |_| log!("{:?} clicked", datum.get())
                    >"Value: " { move || {
                        log!("1- Digging into {datum:?} -> {:?}", datum.get());
                        let text = datum.with(|datum| datum.value.clone());
                        log!("2- Digging into {datum:?} -> {:?}", datum.get());
                        text
                    }}</button>
                    </li>
                }
            }
        />
        </ul>
        <button
            on:click=move |_| {
                set_strings.update(|strings| {
                    strings.push(create_rw_signal(cx, Data{
                        key: format!("FOO -> {}", i.get()),
                        value: format!("Foo -> {}", i.get())
                    }));
                });
                set_i.update(|i_| *i_ += 1);
            }
        >"Add String"</button>
        <button
            on:click=move |_| {
                log!("Update first");
                set_strings.update(|strings| {
                    if let Some(first) = strings.first().cloned() {
                        log!("Attempting to update first -> {first:?}");
                        first.update(|first| first.value = format!("A -> {}", i.get()));
                        log!("Update first -> {first:?}");
                    }
                });
                log!("Vec -> {:?}", strings.get());
            }
        >"Update First"</button>

    }

    // ω <fn for_test_component>
}

// α <mod-def for_test_component>
// ω <mod-def for_test_component>
