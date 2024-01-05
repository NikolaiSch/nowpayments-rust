use crate::response_types::*;
use anyhow::Result;
use reqwest::Method;

const URL: &str = "https://api.nowpayments.io/v1";

pub struct UnverifiedNowPayments {}

pub struct VerifiedNowPayments {
    api_key: String,
    pub client: reqwest::Client,
}

impl UnverifiedNowPayments {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn get_api_status(&self) -> Result<Status> {
        let url = format!("{}/status", URL);
        let resp = reqwest::get(&url).await?;
        let text = resp.text().await?;
        let status: Status = serde_json::from_str(&text)?;
        Ok(status)
    }

    pub async fn verify(self, api_key: String) -> Result<VerifiedNowPayments> {
        let verified = VerifiedNowPayments {
            api_key,
            client: reqwest::Client::new(),
        };
        Ok(verified)
    }
}

impl VerifiedNowPayments {
    pub async fn get_api_status(&self) -> Result<Status> {
        let url = format!("{}/status", URL);
        let resp = reqwest::get(&url).await?;
        let text = resp.text().await?;
        let status: Status = serde_json::from_str(&text)?;
        Ok(status)
    }

    pub async fn get_currencies(&self) -> Result<Currencies> {
        let url = format!("{}/currencies", URL);
        let resp = self
            .client
            .request(Method::GET, &url)
            .header("x-api-key", &self.api_key)
            .query(&[("fixed_rate", true)])
            .send()
            .await?;

        let text = resp.text().await?;
        let currencies: Currencies = serde_json::from_str(&text)?;
        Ok(currencies)
    }

    pub async fn get_full_currencies(&self) -> Result<CurrencyInfos> {
        let url = format!("{}/full-currencies", URL);
        let resp = self
            .client
            .request(Method::GET, &url)
            .header("x-api-key", &self.api_key)
            .send()
            .await?;

        let text = resp.text().await?;
        let currencies: CurrencyInfos = serde_json::from_str(&text)?;
        Ok(currencies)
    }

    pub async fn get_merchant_currencies(&self) -> Result<MerchantCurrencies> {
        let url = format!("{}/merchant/coins", URL);
        let resp = self
            .client
            .request(Method::GET, &url)
            .header("x-api-key", &self.api_key)
            .send()
            .await?;

        let text = resp.text().await?;
        dbg!(&text);
        let currencies: MerchantCurrencies = serde_json::from_str(&text)?;
        Ok(currencies)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn api_is_valid() -> Result<()> {
        let unverified_now_payments = UnverifiedNowPayments::new();
        let api_status = unverified_now_payments.get_api_status().await;
        assert_eq!(api_status.is_ok(), true);
        Ok(())
    }

    #[tokio::test]
    async fn create_verified_now_payments() -> Result<()> {
        let unverified_now_payments = UnverifiedNowPayments::new();
        let verified_now_payments = unverified_now_payments
            .verify("api_key".to_string())
            .await?;

        verified_now_payments.get_api_status().await?;
        Ok(())
    }
}
