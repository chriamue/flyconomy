use flyconomy_contracts_client::Attraction;
use web_sys::HtmlInputElement;
use yew::prelude::*;

pub enum Msg {
    Update,
    NameChanged(String),
    DescriptionChanged(String),
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub attraction: Attraction,
    pub selected_latlng: Option<(f64, f64)>,
    pub on_update: Option<Callback<Attraction>>,
}

pub struct AttractionDetails {
    name: String,
    description: String,
}

impl Component for AttractionDetails {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            name: ctx.props().attraction.name.clone(),
            description: ctx.props().attraction.description.clone(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Update => {
                let mut attraction = ctx.props().attraction.clone();
                attraction.name = self.name.clone();
                attraction.description = self.description.clone();
                match ctx.props().selected_latlng {
                    Some(latlng) => {
                        attraction.lat = latlng.0;
                        attraction.lon = latlng.1;
                    }
                    None => {}
                }
                if let Some(cb) = &ctx.props().on_update {
                    cb.emit(attraction);
                }
            }
            Msg::NameChanged(new_name) => {
                self.name = new_name;
            }
            Msg::DescriptionChanged(new_description) => {
                self.description = new_description;
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let attraction = &ctx.props().attraction;

        let on_name_input = ctx.link().callback(move |event: InputEvent| {
            let input_element = event.target_dyn_into::<HtmlInputElement>().unwrap();
            let value = input_element.value();
            Msg::NameChanged(value)
        });

        let on_description_input = ctx.link().callback(move |event: InputEvent| {
            let input_element = event.target_dyn_into::<HtmlInputElement>().unwrap();
            let value = input_element.value();
            Msg::DescriptionChanged(value)
        });

        let selected_latlng = match ctx.props().selected_latlng {
            Some(latlng) => html! {
                <p>{ format!("Selected latlng: {}, {}", latlng.0, latlng.1) }</p>
            },
            None => html! {},
        };

        html! {
            <div>
                <h2>
                    <input type="text" oninput={on_name_input} value={self.name.clone()} />
                </h2>
                <p>{ format!("ID: {}", attraction.id) }</p>
                <p>{ format!("Latitude: {}", attraction.lat) }</p>
                <p>{ format!("Longitude: {}", attraction.lon) }</p>
                <p>
                    <textarea oninput={on_description_input} value={self.description.clone()}></textarea>
                </p>
                { selected_latlng }
                <button onclick={ctx.link().callback(move |_| Msg::Update)}>{"Update"}</button>
            </div>
        }
    }
}
