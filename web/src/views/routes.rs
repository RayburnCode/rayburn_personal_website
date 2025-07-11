use crate::Route;
use dioxus::prelude::{Router as DRouter, *};
use crate::api::auth::get_user;


/// Register the protected state of routes here
fn is_guarded(current: Route) -> bool {
    match current {
        Route::Protected {} => true,
        _ => false,
    }
}

#[component]
pub fn Router() -> Element {
    rsx! {
        DRouter::<Route> {
            config: || {
                RouterConfig::default()
                    .on_update(|state| {
                        if is_guarded(state.current()) {
                            on_not_authorized(move |_| {
                                GuardContext::set_next(state.current());
                            });
                        }
                        None
                    })
            },
        }
    }
}

#[derive(Default)]
pub struct GuardContext {
    next: Option<Route>,
}

impl GuardContext {
    pub fn set_next(next: Route) {
        let mut guard = use_context::<Signal<GuardContext>>();
        guard.write().next = Some(next);
    }

    pub fn redirect_next_or_home() {
        let nav = navigator();
        let mut guard = use_context::<Signal<GuardContext>>();
        let next_maybe = guard.write().next.take();
        if let Some(next) = next_maybe {
            nav.push(next);
        } else {
            nav.push(Route::Home {});
        }
    }
}

fn on_not_authorized<F>(f: F)
where
    F: Fn(()) + 'static,
{
    spawn(async move {
        let user = get_user().await;
        if user.is_none() {
            f(());
        }
    });
}

/// Declare a page view protected
pub fn protected(redirect: Route, next: Route) {
    spawn(async move {
        let user = get_user().await;
        if user.is_none() {
            GuardContext::set_next(next);
            let nav = navigator();
            nav.replace(redirect);
        }
    });
}

#[cfg(target_arch = "wasm32")]
use instant as _;