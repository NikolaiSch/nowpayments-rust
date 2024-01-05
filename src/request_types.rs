use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateInvoice {
    price_amount: f64,
    price_currency: String,
    ipn_callback_url: Option<String>,
    order_description: String,
    success_url: Option<String>,
    cancel_url: Option<String>,
    is_fixed_rate: bool,
}

impl Default for CreateInvoice {
    fn default() -> Self {
        Self {
            price_amount: 100.0,
            price_currency: "usd".to_string(),
            order_description: " ".to_string(),
            ipn_callback_url: Default::default(),
            success_url: Default::default(),
            cancel_url: Default::default(),
            is_fixed_rate: false,
        }
    }
}
