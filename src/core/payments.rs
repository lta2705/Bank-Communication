pub mod payments;

struct Payment {
    amount: f64,
    currency: String,
    recipient: String,
}

impl Payment {
    pub fn new(amount: f64, currency: String, recipient: String) -> Self {
        Payment {
            amount,
            currency,
            recipient,
        }
    }
    pub fn check_amount(payment: &Payment) -> bool {
        payment.amount > 0.0
    }
    pub fn process_payment(payment: &Payment) -> Result<(), String> {
        if Payment::check_amount(payment) {
            // Here would be the logic to process the payment
            Ok(())
        } else {
            Err("Invalid payment amount".to_string())
        }
    }
}