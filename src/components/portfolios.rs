use yew::prelude::*;
use crate::components::*;

pub enum Msg {
    AddOne,
}

pub struct Portfolios {
    value: i64,
}

impl Component for Portfolios {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            value: 0,
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="mt-10">
               
                <div class="flex flex-col justify-between items-center w-full">
                    <div class="flex justify-between w-full">
                        <h3 class="text-xl font-semibold">{"Portfolios"}</h3>
                        <p class="text-lg font-semibold text-indigo-400">{"see all"}</p>
                    </div>
                    <div class="flex space-x-3 w-full overflow-x-scroll">
                        <portfoliocard::PortfolioCard />
                        <portfoliocard::PortfolioCard />
                    </div>
                </div>
            </div>
        }
    }
}