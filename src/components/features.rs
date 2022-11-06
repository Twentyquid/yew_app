use yew::prelude::*;
use yew_router::prelude::*;
use crate::*;

pub enum Msg {
    AddOne,
}

pub struct Features {
    value: i64,
}

impl Component for Features {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self  {
        Self {
            value: 0,
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="flex w-full space-x-3 text-white">
                <Link<Route> to={Route::Withdraw} classes="rounded-xl bg-indigo-400 px-4 py-3 flex justify-between items-center flex-1  fill-white">
                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" width="24" height="24"><path fill="none" d="M0 0h24v24H0z"/><path d="M13 10h5l-6 6-6-6h5V3h2v7zm-9 9h16v-7h2v8a1 1 0 0 1-1 1H3a1 1 0 0 1-1-1v-8h2v7z"/></svg>
                    <p>{"Withdraw"}</p>
                </Link<Route>>
                <Link<Route> to={Route::Deposit} classes="rounded-xl bg-gray-100 fill-indigo-400 text-indigo-400 px-4 py-3 flex justify-between items-center flex-1">
                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" width="24" height="24"><path fill="none" d="M0 0h24v24H0z"/><path d="M11 11V7h2v4h4v2h-4v4h-2v-4H7v-2h4zm1 11C6.477 22 2 17.523 2 12S6.477 2 12 2s10 4.477 10 10-4.477 10-10 10zm0-2a8 8 0 1 0 0-16 8 8 0 0 0 0 16z"/></svg>
                    <p>{"Deposit"}</p>
                </Link<Route>>
                <div class="rounded-xl bg-gray-100 fill-indigo-400 text-indigo-400 px-4 py-3 flex justify-between items-center">
                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" width="24" height="24"><path fill="none" d="M0 0h24v24H0z"/><path d="M12 3c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2zm0 14c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2zm0-7c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2z"/></svg>
                </div>
            </div>
        }
    }
}
