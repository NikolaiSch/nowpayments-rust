use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Status {
    message: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Currencies {
    pub currencies: Vec<Currency>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Currency {
    pub currency: String,
    pub min_amount: f64,
    pub max_amount: f64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CurrencyInfos {
    pub currencies: Vec<CurrencyInfo>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CurrencyInfo {
    id: usize,
    code: String,
    name: String,
    enable: bool,
    wallet_regex: String,
    priority: usize,
    extra_id_exists: bool,
    extra_id_regex: Option<String>,
    logo_url: String,
    track: bool,
    cg_id: String,
    is_maxlimit: bool,
    network: Option<String>,
    smart_contract: Option<String>,
    network_precision: Option<String>,
    explorer_link_hash: Option<String>,
    precision: usize,
    ticker: Option<String>,
    is_defi: bool,
    is_popular: bool,
    is_stable: bool,
    available_for_to_conversion: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MerchantCurrencies {
    #[serde(rename = "selectedCurrencies")]
    pub selected_currencies: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MinimumPaymentAmount {
    pub min_amount: f64,
    pub currency_from: String,
    pub currency_to: String,
    pub fiat_equivalent: f64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Invoice {
    order_id: Option<String>,
    order_description: String,
    price_amount: String,
    price_currency: String,
    pay_currency: Option<String>,
    ipn_callback_url: Option<String>,
    invoice_url: Option<String>,
    success_url: Option<String>,
    cancel_url: Option<String>,
    created_at: Option<String>,
    updated_at: Option<String>,
}
