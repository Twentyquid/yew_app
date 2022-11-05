use yew::prelude::*;
use thousands::Separable;

#[derive(Properties, PartialEq)]
pub struct CardProps {
    pub current_balance: f32,
    pub last_balance: f32,
}

#[function_component(Card)]
pub fn card(props: &CardProps) -> Html {
    let CardProps { current_balance, last_balance} = props;
    html! {
            <div class="bg-indigo-400 text-white flex flex-col px-4 py-4 rounded-md my-8 space-y-3">
            <div>
                <p>{"My Balance"}</p>
            </div>
            <div class="flex items-center space-x-6">
                <p class="text-3xl font-semibold">{"$"}{current_balance.separate_with_commas()}</p>
                <div class="px-1 py-0.5 rounded-lg bg-white text-indigo-400 text-sm font-bold">{((last_balance / current_balance) * 100.0) as u32}{"%"}</div>
            </div>
        </div>
        }
}