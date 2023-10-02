use gloo_utils::document;
use leaflet::{LatLng, Map, TileLayer, Circle};
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{Element, HtmlElement, Node};
use yew::{html::ImplicitClone, prelude::*};

use flyconomy_contracts_client::Attraction;

pub enum Msg {}

pub struct MapComponent {
    map: Map,
    lat: Point,
    container: HtmlElement,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Point(pub f64, pub f64);

#[derive(PartialEq, Clone, Debug)]
pub struct City {}

impl ImplicitClone for City {}

#[derive(PartialEq, Properties, Clone)]
pub struct Props {
    pub attraction: Attraction,
}

impl MapComponent {
    fn render_map(&self) -> Html {
        let node: &Node = &self.container.clone().into();
        Html::VRef(node.clone())
    }
}

impl Component for MapComponent {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let props = ctx.props();

        let container: Element = document().create_element("div").unwrap();
        let container: HtmlElement = container.dyn_into().unwrap();
        container.set_class_name("map");
        let leaflet_map = Map::new_with_element(&container, &JsValue::NULL);
        let lat = Point(props.attraction.lat, props.attraction.lon);

        Self {
            map: leaflet_map,
            lat,
            container,
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if first_render {
            self.map.setView(&LatLng::new(self.lat.0, self.lat.1), 14.0);
            Circle::new(&LatLng::new(self.lat.0, self.lat.1)).addTo(&mut self.map);
            add_tile_layer(&self.map);
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        let props = ctx.props();
        let lat = Point(props.attraction.lat, props.attraction.lon);

        if self.lat != lat {
            self.lat = lat;
            self.map.setView(&LatLng::new(self.lat.0, self.lat.1), 14.0);
            Circle::new(&LatLng::new(self.lat.0, self.lat.1)).addTo(&mut self.map);
        }

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="map-container component-container">
                {ctx.props().attraction.name.clone()}
                {self.render_map()}
            </div>
        }
    }
}

fn add_tile_layer(map: &Map) {
    TileLayer::new(
        "https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png",
        &JsValue::NULL,
    )
    .addTo(map);
}
