use nowpayments_rust::now_payments::UnverifiedNowPayments;
use nowpayments_rust::request_types::CreateInvoice;

#[tokio::main]
async fn main() {
    let unverified_now_payments = UnverifiedNowPayments::new();
    let api_status = unverified_now_payments.get_api_status().await;
    println!("{:?}", api_status);

    let verified_now_payments = unverified_now_payments
        .verify("APIKEY".to_string())
        .await
        .unwrap();

    let i = verified_now_payments
        .create_invoice(CreateInvoice::default())
        .await
        .unwrap();
    println!("{:?}", i);
}
