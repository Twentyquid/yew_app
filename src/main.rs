use yew::prelude::*;
pub use yewapp_v2::components::*;

enum Msg {
    AddOne,
}

struct Model {
    value: i64,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            value: 0,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                // the value has changed so we need to
                // re-render for it to appear on the page
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();
        html! {
            <div class="min-h-screen bg-white px-5 py-4 max-w-md mx-auto">
                <titlebar::Titlebar />
                <card::Card />
                <features::Features />
                <portfolios::Portfolios />
                <transactions::Transactions />
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}