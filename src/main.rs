use leptos::prelude::*;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::path;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    view! {
        <style>
            {"body {
                background-color: #1e1e1e;
                color: #d4d4d4;
                margin: 0;
                padding: 0;
            }"}
        </style>

        <Router>
          <main>
            <Routes fallback=|| "Not found.">
            <Route path=path!("/mind-palace") view=|| MarkdownViewer/>
            //<Route path=path!("/users") view=Users/>
            //<Route path=path!("/users/:id") view=UserProfile/>
            <Route path=path!("/*any") view=|| view! { <h1>"Not Found"</h1> }/>
          </Routes>
          </main>
        </Router>
    }
}

#[derive(Clone, Debug)]
pub enum PrismTheme {
    Default,
    Dark,
    Okaidia,
    Twilight,
    Coy,
    Solarized,
    Tomorrow,
    Funky,
}

impl PrismTheme {
    fn to_css_url(&self) -> &'static str {
        match self {
            PrismTheme::Default => "https://cdnjs.cloudflare.com/ajax/libs/prism/1.29.0/themes/prism.min.css",
            PrismTheme::Dark => "https://cdnjs.cloudflare.com/ajax/libs/prism/1.29.0/themes/prism-dark.min.css",
            PrismTheme::Okaidia => "https://cdnjs.cloudflare.com/ajax/libs/prism/1.29.0/themes/prism-okaidia.min.css",
            PrismTheme::Twilight => "https://cdnjs.cloudflare.com/ajax/libs/prism/1.29.0/themes/prism-twilight.min.css",
            PrismTheme::Coy => "https://cdnjs.cloudflare.com/ajax/libs/prism/1.29.0/themes/prism-coy.min.css",
            PrismTheme::Solarized => "https://cdnjs.cloudflare.com/ajax/libs/prism/1.29.0/themes/prism-solarizedlight.min.css",
            PrismTheme::Tomorrow => "https://cdnjs.cloudflare.com/ajax/libs/prism/1.29.0/themes/prism-tomorrow.min.css",
            PrismTheme::Funky => "https://cdnjs.cloudflare.com/ajax/libs/prism/1.29.0/themes/prism-funky.min.css",
        }
    }
}

#[component]
fn MarkdownViewer() -> impl IntoView {
    let html_content = markdown::to_html(include_str!("../static/home.md"));
    let styles = include_str!("../static/markdown.css");
    let theme = PrismTheme::Tomorrow;

    view! {
        // Add Prism.js CSS and JS in the head
        <script src="https://cdnjs.cloudflare.com/ajax/libs/prism/1.29.0/prism.min.js"/>
        <script src="https://cdnjs.cloudflare.com/ajax/libs/prism/1.29.0/components/prism-rust.min.js"/>
        <script src="https://cdnjs.cloudflare.com/ajax/libs/prism/1.29.0/components/prism-typescript.min.js"/>
        // Add more language components as needed

        <link
            rel="stylesheet"
            href=theme.to_css_url()
        />
        // Optional: Choose a different theme like prism-okaidia.min.css

        // Custom styles for markdown content
        <style>
            {styles}
        </style>

        // Markdown content container
        <div
            class="markdown-content"
            // Use dangerous_inner_html to render the markdown HTML
            inner_html=html_content
            // Add a script to trigger Prism highlighting after content updates
            on:mounted=move |_: web_sys::Event| {
                js_sys::eval("Prism.highlightAll()").unwrap();
            }
        />
    }
}
