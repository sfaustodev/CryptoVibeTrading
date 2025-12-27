use leptos::*;
use crate::components::landing::LandingPage;
use crate::components::admin::DashboardPage;
use crate::components::auth::{RegisterPage, LoginPage};

#[component]
pub fn LandingRoute() -> impl IntoView {
    view! { <LandingPage/> }
}

#[component]
pub fn AdminRoute() -> impl IntoView {
    view! { <LoginPage/> }
}

#[component]
pub fn AdminDashboardRoute() -> impl IntoView {
    view! { <DashboardPage/> }
}

#[component]
pub fn RegisterRoute() -> impl IntoView {
    view! { <RegisterPage/> }
}

#[component]
pub fn LoginRoute() -> impl IntoView {
    view! { <LoginPage/> }
}
