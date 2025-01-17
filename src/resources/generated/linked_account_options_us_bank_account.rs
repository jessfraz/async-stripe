// ======================================
// This file was automatically generated.
// ======================================

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "linked_account_options_us_bank_account".
#[derive(Clone, Debug, Default, Deserialize, Serialize, JsonSchema)]
pub struct LinkedAccountOptionsUsBankAccount {
    /// The list of permissions to request.
    ///
    /// The `payment_method` permission must be included.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<LinkedAccountOptionsUsBankAccountPermissions>>,

    /// For webview integrations only.
    ///
    /// Upon completing OAuth login in the native browser, the user will be redirected to this URL to return to your app.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,
}

/// An enum representing the possible values of an `LinkedAccountOptionsUsBankAccount`'s `permissions` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum LinkedAccountOptionsUsBankAccountPermissions {
    Balances,
    Ownership,
    PaymentMethod,
    Transactions,
}

impl LinkedAccountOptionsUsBankAccountPermissions {
    pub fn as_str(self) -> &'static str {
        match self {
            LinkedAccountOptionsUsBankAccountPermissions::Balances => "balances",
            LinkedAccountOptionsUsBankAccountPermissions::Ownership => "ownership",
            LinkedAccountOptionsUsBankAccountPermissions::PaymentMethod => "payment_method",
            LinkedAccountOptionsUsBankAccountPermissions::Transactions => "transactions",
        }
    }
}

impl AsRef<str> for LinkedAccountOptionsUsBankAccountPermissions {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for LinkedAccountOptionsUsBankAccountPermissions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for LinkedAccountOptionsUsBankAccountPermissions {
    fn default() -> Self {
        Self::Balances
    }
}
