use yew::prelude::*;

pub enum Msg {
    AddOne,
}

pub struct Card {
    value: i64,
}

impl Component for Card {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            value: 0,
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="bg-indigo-400 text-white flex flex-col px-4 py-4 rounded-md my-8 space-y-3">
            <div>
                <p>{"My Balance"}</p>
            </div>
            <div class="flex items-center space-x-6">
                <p class="text-3xl font-semibold">{"$12,778.15"}</p>
                <div class="px-1 py-0.5 rounded-lg bg-white text-indigo-400 text-sm font-bold">{"+15%"}</div>
            </div>
        </div>
        }
        
    }
}