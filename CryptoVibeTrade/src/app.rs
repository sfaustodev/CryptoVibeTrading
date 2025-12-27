#![forbid(unsafe_code)]

use leptos::*;
use leptos_meta::*;
use leptos_router::{Route, Router, Routes};

use crate::routes::{LandingRoute, AdminRoute, AdminDashboardRoute, LoginRoute, RegisterRoute};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Title text="Crypto Vibe Trade"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1"/>
        <Meta charset="UTF-8"/>

        <Router>
            <main>
                <Routes>
                    <Route path="/" view=LandingRoute/>
                    <Route path="/auth/login" view=LoginRoute/>
                    <Route path="/auth/register" view=RegisterRoute/>
                    <Route path="/admin" view=AdminRoute/>
                    <Route path="/admin/dashboard" view=AdminDashboardRoute/>
                </Routes>
            </main>
        </Router>
    }
}
