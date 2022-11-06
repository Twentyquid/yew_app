use yew_router::prelude::*;
pub mod components;


#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/bank")]
    Bank,
    #[at("/account/withdraw")]
    Withdraw,
    #[at("/account/deposit")]
    Deposit,
    #[at("/404")]
    NotFound,
}