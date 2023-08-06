use anyhow::anyhow;
use futures::FutureExt;
use std::str::FromStr;
use subxt::utils::AccountId32;
use subxt::OnlineClient;
use subxt::PolkadotConfig;
use wasm_bindgen::JsCast;
use web_sys::EventTarget;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::services::get_accounts;
use crate::services::Account;
use crate::services::TokenService;

pub struct TokenComponent {
    account: Option<String>,
    balance: Option<u128>,
    online_client: Option<OnlineClient<PolkadotConfig>>,
    stage: TokenStage,
    token_service: Option<TokenService>,
}

pub enum TokenStage {
    Error(String),
    CreatingOnlineClient,
    EnterAccount,
    RequestingBalance,
    DisplayBalance(u128), // The balance can be u128 for illustrative purposes
    RequestingAccounts,
    SelectAccount(Vec<Account>),
    Signing(Account),
}

pub enum Message {
    Error(anyhow::Error),
    OnlineClientCreated(OnlineClient<PolkadotConfig>),
    TokenServiceCreated(TokenService),
    RequestAccounts,
    ReceivedAccounts(Vec<Account>),
    SignWithAccount(usize),
    RequestBalance,
    ReceivedBalance(u128),
}

impl Component for TokenComponent {
    type Message = Message;

    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_future(OnlineClient::<PolkadotConfig>::new().map(|res| {
            match res {
                Ok(online_client) => Message::OnlineClientCreated(online_client),
                Err(err) => Message::Error(anyhow!("Online Client could not be created. Make sure you have a local node running:\n{err}")),
            }
        }));
        ctx.link()
            .send_future(TokenService::new().map(|res| match res {
                Ok(service) => Message::TokenServiceCreated(service),
                Err(err) => Message::Error(anyhow!("Failed to create TokenService: {}", err)),
            }));
        TokenComponent {
            account: None,
            balance: None,
            stage: TokenStage::CreatingOnlineClient,
            online_client: None,
            token_service: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::OnlineClientCreated(online_client) => {
                self.online_client = Some(online_client);
                self.stage = TokenStage::EnterAccount;
            }
            Message::TokenServiceCreated(service) => {
                self.token_service = Some(service);
            }
            Message::SignWithAccount(i) => {
                if let TokenStage::SelectAccount(accounts) = &self.stage {
                    let account = accounts.get(i).unwrap();
                    let account_address = account.address.clone();
                    let account_source = account.source.clone();
                    let account_id: AccountId32 = account_address.parse().unwrap();
                    self.account = Some(account_address);

                    self.stage = TokenStage::Signing(account.clone());
                    ctx.link()
                        .send_future(async move { Message::RequestBalance });
                }
            }
            Message::RequestBalance => {
                if let Some(account) = &self.account {
                    let account_clone = account.clone();
                    let service_clone = self.token_service.as_ref().unwrap().clone();
                    let link = ctx.link();

                    link.send_future(async move {
                        let account_id: AccountId32 =
                            AccountId32::from_str(&account_clone).unwrap();
                        match service_clone
                            .get_balance_of(&account_id, account_clone.clone())
                            .await
                        {
                            Ok(balance) => Message::ReceivedBalance(balance),
                            Err(_) => {
                                Message::Error(anyhow!("Failed to fetch balance.".to_string()))
                            }
                        }
                    });
                }
            }
            Message::ReceivedBalance(balance) => {
                self.balance = Some(balance);
                self.stage = TokenStage::DisplayBalance(balance);
            }
            Message::Error(err) => self.stage = TokenStage::Error(err.to_string()),
            Message::RequestAccounts => {
                self.stage = TokenStage::RequestingAccounts;
                ctx.link().send_future(get_accounts().map(
                    |accounts_or_err| match accounts_or_err {
                        Ok(accounts) => Message::ReceivedAccounts(accounts),
                        Err(err) => Message::Error(err),
                    },
                ));
            }
            Message::ReceivedAccounts(accounts) => {
                self.stage = TokenStage::SelectAccount(accounts);
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let stage_html: Html = match &self.stage {
            TokenStage::Error(error_message) => {
                html!(<div class="error"> {"Error: "} {error_message} </div>)
            }
            TokenStage::CreatingOnlineClient => {
                html!(<div>{"Creating Online Client..."}</div>)
            }
            TokenStage::EnterAccount => {
                let get_accounts_click = ctx.link().callback(|_| Message::RequestAccounts);
                html!(<>
                    <div>
                        <button onclick={get_accounts_click}> {"=> Select an Account"} </button>
                    </div>
                </>)
            }
            TokenStage::RequestingAccounts => {
                html!(<div>{"Querying extensions for accounts..."}</div>)
            }
            TokenStage::SelectAccount(accounts) => {
                if accounts.is_empty() {
                    html!(<div>{"No Web3 extension accounts found. Install Talisman or the Polkadot.js extension and add an account."}</div>)
                } else {
                    html!(
                        <>
                            <div class="mb"><b>{"Select an account you want to use for signing:"}</b></div>
                            { for accounts.iter().enumerate().map(|(i, account)| {
                                let sign_with_account = ctx.link().callback(move |_| Message::SignWithAccount(i));
                                html! {
                                    <button onclick={sign_with_account}>
                                        {&account.source} {" | "} {&account.name}<br/>
                                        <small>{&account.address}</small>
                                    </button>
                                }
                            }) }
                        </>
                    )
                }
            }
            TokenStage::Signing(_) => {
                html!(<div>{"Singing message with browser extension..."}</div>)
            }
            TokenStage::RequestingBalance => {
                html!(<div>{"Requesting balance for the account..."}</div>)
            }
            TokenStage::DisplayBalance(balance) => {
                html!(<div>{"Balance: "} {balance} </div>)
            }
        };

        html! {
            <div>
                <a href="/"> <button>{"<= Back"}</button></a>
                <h1>{"Token Management"}</h1>
                {stage_html}
            </div>
        }
    }
}
