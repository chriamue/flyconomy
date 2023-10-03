use flyconomy_contracts_client::Attraction;
use leaflet::LatLng;
use yew::prelude::*;

mod attraction_details;
mod attraction_list;
mod map_component;

use attraction_details::AttractionDetails;
use attraction_list::AttractionList;
use map_component::MapComponent;

enum Msg {
    SelectedAttraction(Attraction),
    MapClicked(Option<LatLng>),
}

struct App {
    attraction: Option<Attraction>,
    latlng: Option<LatLng>,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            attraction: None,
            latlng: None,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SelectedAttraction(attraction) => {
                self.attraction = Some(attraction);
            }
            Msg::MapClicked(latlng) => {
                self.latlng = latlng;
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let select_attraction_cb = ctx.link().callback(Msg::SelectedAttraction);
        let map_clicked_cb = ctx.link().callback(Msg::MapClicked);

        let map_component = match &self.attraction {
            Some(attraction) => html! {
                <MapComponent attraction={attraction.clone()} on_click={map_clicked_cb} />
            },
            None => html! {
                <></>
            },
        };

        let latlng: Option<(f32, f32)> = match &self.latlng {
            Some(latlng) => Some((latlng.lat() as f32, latlng.lng() as f32)),
            None => None,
        };

        let attraction_details = match &self.attraction {
            Some(attraction) => html! {
                <AttractionDetails attraction={attraction.clone()} selected_latlng={latlng} />
            },
            None => html! {
                <></>
            },
        };

        html! {
            <>
                {map_component}
                {attraction_details}
                <AttractionList select_attraction={select_attraction_cb} />
            </>
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
