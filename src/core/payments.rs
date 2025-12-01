pub mod payments;

struct Payment {
    amount: f64,
    currency: String,
    recipient: String,
}

pub fn check_amount(payment: &Payment) -> bool {
    payment.amount > 0.0
}