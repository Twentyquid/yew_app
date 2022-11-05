use yew::prelude::*;
use crate::components::*;

pub struct TransactionDetails {
    title: String,
    expense: f32,
}

pub enum Msg {
    AddOne,
}

pub struct Transactions {
    values: Vec<TransactionDetails>,
}

impl Component for Transactions {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            values: vec![
                TransactionDetails {
                    title: String::from("Food & Driks"),
                    expense: 12.00,
                },
                TransactionDetails {
                    title: String::from("Petrol"),
                    expense: 20.00,
                },
                TransactionDetails {
                    title: String::from("Insurance"),
                    expense: 50.00,
                }
            ]
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="w-full mt-4">
                <h3 class="text-xl font-semibold text-gray-800 mb-4">{"Transactions"}</h3>
                {
                    self.values.iter().map(|item| {
                        html! {
                            <transaction_card::TransactionCard title={item.title.clone()} expense={item.expense} />
                        }
                    }).collect::<Vec<Html>>()
                }
                // <transaction_card::TransactionCard />
            </div>
        }
    }
}