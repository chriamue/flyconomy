use flyconomy_contracts_client::AttractionContract;
use flyconomy_contracts_client::DEFAULT_CONTRACT_ADDRESS;
use flyconomy_contracts_client::DEFAULT_NODE_URL;
use yew::prelude::*;

use flyconomy_contracts_client::Attraction;
use flyconomy_contracts_client::Web3Contract;

#[derive(Debug)]
pub enum Msg {
    Error(Box<dyn std::error::Error>),
    GetAttractions,
    ReceiveResponse(Result<Vec<Attraction>, anyhow::Error>),
}

pub struct AttractionList {
    attractions: Vec<Attraction>,
    error: Option<String>,
}

impl Component for AttractionList {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(Msg::GetAttractions);
        Self {
            attractions: Vec::new(),
            error: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::GetAttractions => {
                ctx.link().send_future(async move {
                    let result: Result<Vec<Attraction>, Box<dyn std::error::Error>> = async {
                        let contract =
                            Web3Contract::new(DEFAULT_NODE_URL, DEFAULT_CONTRACT_ADDRESS).await?;
                        let attractions = contract.get_all_locations().await?;
                        let attractions: Vec<Attraction> = attractions
                            .into_iter()
                            .map(|attraction| Attraction {
                                id: attraction.id,
                                name: attraction.name,
                                description: attraction.description,
                                lat: attraction.lat,
                                lon: attraction.lon,
                            })
                            .collect();

                        Ok(attractions)
                    }
                    .await;

                    match result {
                        Ok(attractions) => Msg::ReceiveResponse(Ok(attractions)),
                        Err(e) => Msg::Error(e),
                    }
                });
                false
            }
            Msg::ReceiveResponse(response) => {
                match response {
                    Ok(attractions) => self.attractions = attractions,
                    Err(error) => ctx.link().send_message(Msg::Error(error.into())),
                }
                true
            }
            Msg::Error(err) => {
                self.error = Some(err.to_string());
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let error_text = match &self.error {
            Some(error) => error,
            None => "",
        };

        html! {
            <div>
                <h1>{ "Attractions" }</h1>
                <p>{ "Here are some attractions" }</p>
                <ul>
                    { for self.attractions.iter().map(|attraction| self.view_attraction(attraction)) }
                </ul>
                <p>{ error_text }</p>
            </div>
        }
    }
}

impl AttractionList {
    fn view_attraction(&self, attraction: &Attraction) -> Html {
        html! {
            <li key={attraction.id}>
                <h2>{ &attraction.name }</h2>
                <p>{ &attraction.description }</p>
                <p>{ format!("Latitude: {}", &attraction.lat) }</p>
                <p>{ format!("Longitude: {}", &attraction.lon) }</p>
            </li>
        }
    }
}
