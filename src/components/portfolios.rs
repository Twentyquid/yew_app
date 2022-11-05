use yew::prelude::*;
use crate::components::*;

#[derive(PartialEq, Clone)]
pub struct PortfolioDetails {
    pub title: String,
    pub initial_value: f32,
    pub final_value: f32,
}


// #[derive(Properties, PartialEq, Clone)]
// pub struct PortfoloioProps {
//     pub portfolio_details: PortfolioDetails,
// }


pub enum Msg {
    AddOne,
}

pub struct Portfolios {
    values: Vec<PortfolioDetails>
}




impl Component for Portfolios {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            values:  vec![ 
        PortfolioDetails {
        title: String::from("New Home"),
        initial_value: 60_000.0,
        final_value: 30_000.0,
    },
    PortfolioDetails {
        title: String::from("New Bike"),
        initial_value: 6000.0,
        final_value: 3200.0,
    }],
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
                    {
                        self.values.iter().map(|item| {
                            html! {
                                <portfoliocard::PortfolioCard title={item.title.clone()} initial_value={item.initial_value} final_value={item.final_value} />
                            }
                        }).collect::<Vec<Html>>()
                    }
                        
                        // <portfoliocard::PortfolioCard />
                    </div>
                </div>
            </div>
        }
    }
}