use yew::prelude::*;

mod attraction_list;

use attraction_list::AttractionList;

#[function_component]
fn App() -> Html {

    html! {
        <div>
            <AttractionList />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
