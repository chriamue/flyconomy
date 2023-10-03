use flyconomy_contracts_client::Attraction;
use yew::prelude::*;

pub enum Msg {
    Update,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub attraction: Attraction,
    pub selected_latlng: Option<(f32, f32)>,
}

pub struct AttractionDetails {}

impl Component for AttractionDetails {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Update => {}
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let attraction = &ctx.props().attraction;

        let selected_latlng = match ctx.props().selected_latlng {
            Some(latlng) => html! {
                <p>{ format!("Selected latlng: {}, {}", latlng.0, latlng.1) }</p>
            },
            None => html! {
                <></>
            },
        };

        html! {
            <div>
                <h2>{ ctx.props().attraction.name.clone() }</h2>
                <p>{ format!("ID: {}", attraction.id) }</p>
                <p>{ format!("Latitude: {}", attraction.lat) }</p>
                <p>{ format!("Longitude: {}", attraction.lon) }</p>
                <p>{ attraction.description.clone() }</p>
                { selected_latlng }
                <button onclick={ctx.link().callback(move |_| Msg::Update)}>{"Update"}</button>
            </div>
        }
    }
}
