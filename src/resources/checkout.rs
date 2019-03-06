use crate::resources::Currency;
use serde_derive::{Deserialize, Serialize};

/// The resource representing a Stripe checkout_beta object
///
/// TODO: Due to it's beta status, documentation is lacking.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CheckoutBeta {
    pub client_reference_id: String,
    pub display_items: Vec<CheckoutBetaDisplayItem>,
    // pub line_items: Vec<?>,
    pub livemode: bool,
    pub payment_intent: String,
    pub subscription: Option<String>,
}

/// The resource representing a Stripe invoice line item.
///
/// TODO: Due to it's beta status, documentation is lacking.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CheckoutBetaDisplayItem {
    pub currency: Currency,
    pub amount: i64,
    #[serde(rename = "type")]
    pub item_type: String, // (plan, ?)
    pub quantity: Option<u64>,
    pub plan: Option<String>,
}

