use yew::prelude::*;
use thousands::Separable;

pub enum Msg {
    AddOne,
}

pub struct PortfolioCard {
    value: i64,
}

#[derive(Properties, PartialEq)]
pub struct PortfolioCardProps {
    pub title: String,
    pub initial_value: f32,
    pub final_value: f32,
}

impl Component for PortfolioCard {
    type Message = Msg;
    type Properties = PortfolioCardProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            value: 0,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let PortfolioCardProps { title, initial_value, final_value} = ctx.props().clone();
        html! {
            <div class="flex justify-between basis-4/5 min-w-[90%] flex-col  my-3 bg-violet-100 px-3 py-4 rounded-xl shadow-lg">
                <div class="flex w-full justify-between items-center mb-4">
                    <div class="flex items-center space-x-3">
                        <div>
                            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" width="24" height="24"><path fill="none" d="M0 0h24v24H0z"/><path d="M6 19h12V9.157l-6-5.454-6 5.454V19zm13 2H5a1 1 0 0 1-1-1v-9H1l10.327-9.388a1 1 0 0 1 1.346 0L23 11h-3v9a1 1 0 0 1-1 1zM7.5 13h2a2.5 2.5 0 1 0 5 0h2a4.5 4.5 0 1 1-9 0z"/></svg>
                        </div>
                        <div>
                            <h3 class="font-semibold text-lg">{title.clone()}</h3>
                            <p class="text-sm font-semibold text-gray-500">{"10/10/2022 7:30"}</p>
                        </div>
                    </div>
                    <div class="">
                        <p class="font-semibold text-indigo-600">{((final_value / initial_value) * 100.0) as u16}{"%"}</p>
                    </div>
                </div>
                <div class="flex flex-col">
                    <div class="w-full h-1 relative bg-gray-300 rounded-full">
                        <div class="absolute inset-0 w-1/2 h-1 bg-indigo-600 rounded-full"></div>
                    </div>
                    <div class="flex justify-between mt-3">
                        <p class="text-gray-600">{"$"}{final_value.separate_with_commas()}</p>
                        <p class="font-semibold">{"$"}{initial_value.separate_with_commas()}</p>
                    </div>
                </div>
            </div>
        }
    }
}