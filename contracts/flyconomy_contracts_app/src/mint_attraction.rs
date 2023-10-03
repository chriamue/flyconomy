use flyconomy_contracts_client::Attraction;
use leaflet::LatLng;
use web_sys::console;
use yew::prelude::*;

use crate::{attraction_details::AttractionDetails, map_component::MapComponent, web3};

pub enum Msg {
    Mint,
    ToggleVisibility,
    MapClicked(Option<LatLng>),
    UpdateAttraction(Attraction),
    AttractionUpdated(Attraction),
}

pub struct MintAttraction {
    attraction: Attraction,
    latlng: Option<LatLng>,
    visible: bool,
}

impl Component for MintAttraction {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            attraction: Attraction {
                name: "Greenwich Park".to_string(),
                description: "Give it a description.".to_string(),
                lat: 51.477928,
                lon: 0.0,
                ..Default::default()
            },
            latlng: None,
            visible: false,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Mint => {
                let attraction = self.attraction.clone();
                ctx.link().send_future(async move {
                    let attraction = attraction.clone();
                    web3::mint(attraction.clone()).await.unwrap();
                    Msg::AttractionUpdated(attraction)
                });
            }
            Msg::ToggleVisibility => {
                self.visible = !self.visible;
            }
            Msg::MapClicked(latlng) => {
                self.latlng = latlng;
            }
            Msg::UpdateAttraction(attraction) => {
                self.attraction = attraction;
            }
            Msg::AttractionUpdated(attraction) => {
                console::log_1(&format!("Minted attraction: {:?}", attraction).into());
            }
        };
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let map_clicked_cb = ctx.link().callback(Msg::MapClicked);
        let update_attraction_cb = ctx.link().callback(Msg::UpdateAttraction);

        let map_component = 
            html! {
                <MapComponent attraction={self.attraction.clone()} on_click={map_clicked_cb} />
            };

        let latlng: Option<(f64, f64)> = match &self.latlng {
            Some(latlng) => Some((latlng.lat(), latlng.lng())),
            None => None,
        };

        let attraction_details = html!{
                <AttractionDetails attraction={self.attraction.clone()} selected_latlng={latlng} on_update={update_attraction_cb} />
            };

        let mint_attraction_content = if self.visible {
            html! {
                <>
                    {map_component}
                    {attraction_details}
                    <button onclick={ctx.link().callback(|_| Msg::Mint)}>{"Mint"}</button>
                </>
            }
        } else {
            html! {}
        };

        html! {
            <>
                {mint_attraction_content}
                <button onclick={ctx.link().callback(|_| Msg::ToggleVisibility)}>
                    {if self.visible { "Hide Mint Attraction" } else { "Show Mint Attraction" }}
                </button>
            </>
        }
    }
}
