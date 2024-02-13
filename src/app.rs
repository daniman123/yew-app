use crate::pages::{
    landing_page::LandingPage, test_page::TestPage, user_stats_page::UserStatsPage,
};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/user-stats")]
    UserStats,
    #[at("/test")] // Add a route for the Test page
    Test,
    // Add more routes here if needed
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! { <LandingPage /> },
        Route::UserStats => html! { <UserStatsPage/> },
        Route::Test => html! { <TestPage  /> }, // Handle the Test route
                                                // Handle other routes as needed
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={|routes: Route| switch(&routes)} />
        </BrowserRouter>
    }
}
