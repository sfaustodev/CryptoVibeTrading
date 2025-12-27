pub mod landing;
pub mod admin;
pub mod dashboard;

use leptos::*;
use crate::components::landing::LandingPage;
use crate::components::admin::LoginPage;
use crate::components::dashboard::DashboardPage;

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
