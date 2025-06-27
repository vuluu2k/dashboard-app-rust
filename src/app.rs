pub mod components;
pub mod db;
pub mod models;
pub mod server_functions;
pub mod pages;
use pages::{ HomePage, TeamPage };

use leptos::prelude::*;
use leptos_meta::{ provide_meta_context, Stylesheet, Title };
use leptos_router::{ components::{ Route, Router, Routes }, StaticSegment, WildcardSegment };

#[component]
pub fn App() -> impl IntoView {
	// Provides context that manages stylesheets, titles, meta tags, etc.
	provide_meta_context();

	view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/dashboard-app-rust.css"/>
        <link data-truck rel="tailwind-css" href="/style/input.css"></link>

        // sets the document title
        <Title text="Fullstack Rust App"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes fallback=move || "Not found.">
									<Route path=StaticSegment("") view=move || view! { <HomePage/> }/>
									<Route path=StaticSegment("/team") view=move || view! { <TeamPage/> } />
									<Route path=WildcardSegment("any") view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
	// set an HTTP status code 404
	// this is feature gated because it can only be done during
	// initial server-side rendering
	// if you navigate to the 404 page subsequently, the status
	// code will not be set because there is not a new HTTP request
	// to the server
	#[cfg(feature = "ssr")]
	{
		// this can be done inline because it's synchronous
		// if it were async, we'd use a server function
		let resp = expect_context::<leptos_actix::ResponseOptions>();
		resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
	}

	view! {
        <h1>"Not Found"</h1>
    }
}
