use leptos::*;
use crate::components::landing::LandingPage;
use crate::components::dashboard::DashboardPage;
use crate::components::auth::{RegisterPage, LoginPage};
use crate::components::whiteboard::Whiteboard;

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

#[component]
pub fn WhiteboardRoute() -> impl IntoView {
    view! { <Whiteboard width=1920 height=1080 /> }
}
