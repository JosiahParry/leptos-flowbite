use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;


#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {


        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/flowbite-leptos.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        
        <main class="flex flex-col h-screen">
            <div class="flex flex-1 overflow-hidden">
              <div class="flex bg-gray-100">
                <crate::sidebar::SideBar/>
              </div>
              <div class="flex flex-1 flex-col">
                <div class="flex">
                    <crate::navbar::NavBar/>
                </div>
                <div class="flex flex-1 paragraph px-4">
                  <crate::content::ContentBody/>
                </div>
              </div>
            </div>
          </main>
          <crate::footer::Footer/>
    }
}

