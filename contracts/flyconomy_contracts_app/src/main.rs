use flyconomy_contracts_client::Attraction;
use leaflet::LatLng;
use web_sys::console;
use yew::prelude::*;

mod attraction_details;
mod attraction_list;
mod map_component;
mod web3;

use attraction_details::AttractionDetails;
use attraction_list::AttractionList;
use map_component::MapComponent;

enum Msg {
    SelectedAttraction(Attraction),
    MapClicked(Option<LatLng>),
    UpdateAttraction(Attraction),
    AttractionUpdated(Attraction),
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

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SelectedAttraction(attraction) => {
                self.attraction = Some(attraction);
            }
            Msg::MapClicked(latlng) => {
                self.latlng = latlng;
            }
            Msg::UpdateAttraction(attraction) => {
                let attraction = attraction.clone();
                ctx.link().send_future(async move {
                    let attraction = attraction.clone();
                    web3::update(attraction.clone()).await.unwrap();
                    Msg::AttractionUpdated(attraction)
                });
            }
            Msg::AttractionUpdated(attraction) => {
                console::log_1(&format!("Updated attraction: {:?}", attraction).into());
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let select_attraction_cb = ctx.link().callback(Msg::SelectedAttraction);
        let map_clicked_cb = ctx.link().callback(Msg::MapClicked);
        let update_attraction_cb = ctx.link().callback(Msg::UpdateAttraction);

        let map_component = match &self.attraction {
            Some(attraction) => html! {
                <MapComponent attraction={attraction.clone()} on_click={map_clicked_cb} />
            },
            None => html! {
                <></>
            },
        };

        let latlng: Option<(f64, f64)> = match &self.latlng {
            Some(latlng) => Some((latlng.lat(), latlng.lng())),
            None => None,
        };

        let attraction_details = match &self.attraction {
            Some(attraction) => html! {
                <AttractionDetails attraction={attraction.clone()} selected_latlng={latlng} on_update={update_attraction_cb} />
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
