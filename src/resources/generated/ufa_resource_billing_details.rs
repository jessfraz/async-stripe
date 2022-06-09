// ======================================
// This file was automatically generated.
// ======================================

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::resources::Address;

/// The resource representing a Stripe "UFAResourceBillingDetails".
#[derive(Clone, Debug, Default, Deserialize, Serialize, JsonSchema)]
pub struct UfaResourceBillingDetails {
    pub address: Address,

    /// Email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    /// Full name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
