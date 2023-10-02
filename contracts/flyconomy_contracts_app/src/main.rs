use flyconomy_contracts_client::Attraction;
use yew::prelude::*;

mod attraction_list;
mod map_component;

use attraction_list::AttractionList;
use map_component::MapComponent;

enum Msg {
    SelectedAttraction(Attraction),
}

struct App {
    attraction: Option<Attraction>,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { attraction: None }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SelectedAttraction(attraction) => {
                self.attraction = Some(attraction);
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let cb = ctx.link().callback(Msg::SelectedAttraction);
        
        let map_component = match &self.attraction {
            Some(attraction) => html! {
                <MapComponent attraction={attraction.clone()} />
            },
            None => html! {
                <></>
            },
        };
        
        html! {
            <>
                {map_component}
                <AttractionList select_attraction={cb} />
            </>
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
