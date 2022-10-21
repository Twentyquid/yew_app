use yew::prelude::*;

pub enum Msg {
    Nothing,
}

pub struct Titlebar {
    value: i64,
}

impl Component for Titlebar {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            value: 0,
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="bg-white flex justify-between items-center">
                <div class="flex items-center space-x-5">
                    <div class="w-10 h-10 overflow-hidden rounded-full">
                        <img class="object-cover w-full h-full" src="images/profile.jpg" alt="Amanda Ker" />
                    </div>
                    <div>
                        <p class="font-semibold text-gray-700">{"Amanda Ker"}</p>
                        <p class="text-xs font-semibold text-gray-500">{"10 Active savings"}</p>
                    </div>
                </div>
                <div class="fill-blue-500">
                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" width="24" height="24"><path fill="none" d="M0 0h24v24H0z"/><path d="M5 18h14v-6.969C19 7.148 15.866 4 12 4s-7 3.148-7 7.031V18zm7-16c4.97 0 9 4.043 9 9.031V20H3v-8.969C3 6.043 7.03 2 12 2zM9.5 21h5a2.5 2.5 0 1 1-5 0z"/></svg>
                </div>
            </div>
            
        }
    }
}