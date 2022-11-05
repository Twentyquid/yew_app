use yew::prelude::*;
pub use yewapp_v2::components::*;

#[function_component(Model)]
fn model() -> Html {
     html! {
            <div class="min-h-screen bg-white px-5 py-4 max-w-md mx-auto">
                <titlebar::Titlebar />
                <card::Card current_balance={12_000.50} last_balance={10_000.00} />
                <features::Features />
                <portfolios::Portfolios />
                <transactions::Transactions />
            </div>
        }
}

fn main() {
    yew::start_app::<Model>();
}