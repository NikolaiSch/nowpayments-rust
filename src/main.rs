use nowpayments_rust::now_payments::UnverifiedNowPayments;

#[tokio::main]
async fn main() {
    let unverified_now_payments = UnverifiedNowPayments::new();
    let api_status = unverified_now_payments.get_api_status().await;
    println!("{:?}", api_status);

    let verified_now_payments = unverified_now_payments
        .verify("KM28NFV-5EEMD1X-N57XZH8-PCG48HC".to_string())
        .await
        .unwrap();

    let currencies = verified_now_payments.get_merchant_currencies().await;
    println!("{:?}", currencies);
}
