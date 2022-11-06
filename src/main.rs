use yew::prelude::*;
use yew_router::prelude::*;
pub use yewapp_v2::components::*;
use yewapp_v2::*;



fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! { <mainpage::MainPage /> },
        Route::Bank => html! { <h1 class="text-2xl">{"This is the bank page"}</h1>},
        Route::Withdraw => html! { <h1 class="text-2xl">{"This is the withdraw page"}</h1>},
        Route::Deposit => html! { <h1 class="text-2xl">{"This is the deposit page"}</h1>},
        Route::NotFound => html! { <h1 class="text-2xl">{"Page Not Found"}</h1> },
    }
}

#[function_component(App)]
fn app() -> Html {
     html! {
           <BrowserRouter>
            <div class="min-h-screen bg-white max-w-md">
                <Switch<Route> render={Switch::render(switch)} />
            </div>
            </BrowserRouter>
        }
}

fn main() {
    yew::start_app::<App>();
}