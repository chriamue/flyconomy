use yew::prelude::*;
use subxt::OnlineClient;
use subxt::PolkadotConfig;
use anyhow::anyhow;
use web_sys::HtmlInputElement;
use futures::FutureExt;
use subxt::utils::AccountId32;
use web_sys::EventTarget;
use wasm_bindgen::JsCast;

use crate::services::get_accounts;
use crate::services::Account;

pub struct TokenComponent {
    account: Option<String>,
    balance: Option<u128>,
    online_client: Option<OnlineClient<PolkadotConfig>>,
    stage: TokenStage,
}

pub enum TokenStage {
    Error(String),
    CreatingOnlineClient,
    EnterAccount,
    RequestingBalance,
    DisplayBalance(u128), // The balance can be u128 for illustrative purposes
    RequestingAccounts,
    SelectAccount(Vec<Account>),
}

pub enum Message {
    Error(anyhow::Error),
    OnlineClientCreated(OnlineClient<PolkadotConfig>),
    ChangeAccount(String),
    RequestBalance,
    ReceivedBalance(u128),
    RequestAccounts,
    ReceivedAccounts(Vec<Account>),
    SignWithAccount(usize),
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
        TokenComponent {
            account: None,
            balance: None,
            stage: TokenStage::CreatingOnlineClient,
            online_client: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::OnlineClientCreated(online_client) => {
                self.online_client = Some(online_client);
                self.stage = TokenStage::EnterAccount;
            }
            Message::ChangeAccount(account_str) => {
                // Here you can convert the account string to AccountId32 and update self.account
            }
            Message::SignWithAccount(i) => {
                if let TokenStage::SelectAccount(accounts) = &self.stage {
                    let account = accounts.get(i).unwrap();
                    let account_address = account.address.clone();
                    let account_source = account.source.clone();
                    let account_id: AccountId32 = account_address.parse().unwrap();
                    self.account = Some(account_address);
                }
            }
            Message::RequestBalance => {
                self.stage = TokenStage::RequestingBalance;
                // Here you would send a request to fetch the balance for the given account
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
                // For simplicity, if only one account is returned, set it directly.
                if accounts.len() == 1 {
                    self.account = Some(accounts[0].address.to_string());
                    self.stage = TokenStage::EnterAccount;
                } else {
                    self.stage = TokenStage::SelectAccount(accounts);
                }
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
            TokenStage::RequestingAccounts => {
                html!(<div>{"Querying extensions for accounts..."}</div>)
            }
            TokenStage::EnterAccount => {
                html!(<>
                    <div>
                        <input type="text"
                            value={self.account.clone()}
                        />
                        <button onclick={ctx.link().callback(|_| Message::RequestAccounts)}>
                            {"Get Accounts"}
                        </button>
                    </div>
                </>)
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
