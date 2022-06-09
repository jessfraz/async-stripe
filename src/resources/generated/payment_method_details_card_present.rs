// ======================================
// This file was automatically generated.
// ======================================

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::params::Timestamp;

/// The resource representing a Stripe "payment_method_details_card_present".
#[derive(Clone, Debug, Default, Deserialize, Serialize, JsonSchema)]
pub struct PaymentMethodDetailsCardPresent {
    /// The authorized amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_authorized: Option<i64>,

    /// Card brand.
    ///
    /// Can be `amex`, `diners`, `discover`, `jcb`, `mastercard`, `unionpay`, `visa`, or `unknown`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand: Option<String>,

    /// When using manual capture, a future timestamp after which the charge will be automatically refunded if uncaptured.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_before: Option<Timestamp>,

    /// The cardholder name as read from the card, in [ISO 7813](https://en.wikipedia.org/wiki/ISO/IEC_7813) format.
    ///
    /// May include alphanumeric characters, special characters and first/last name separator (`/`).
    /// In some cases, the cardholder name may not be available depending on how the issuer has configured the card.
    /// Cardholder name is typically not available on swipe or contactless payments, such as those made with Apple Pay and Google Pay.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cardholder_name: Option<String>,

    /// Two-letter ISO code representing the country of the card.
    ///
    /// You could use this attribute to get a sense of the international breakdown of cards you've collected.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,

    /// Authorization response cryptogram.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emv_auth_data: Option<String>,

    /// Two-digit number representing the card's expiration month.
    pub exp_month: i64,

    /// Four-digit number representing the card's expiration year.
    pub exp_year: i64,

    /// Uniquely identifies this particular card number.
    ///
    /// You can use this attribute to check whether two customers who’ve signed up with you are using the same card number, for example.
    /// For payment methods that tokenize card information (Apple Pay, Google Pay), the tokenized number might be provided instead of the underlying card number.  *Starting May 1, 2021, card fingerprint in India for Connect will change to allow two fingerprints for the same card --- one for India and one for the rest of the world.*.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,

    /// Card funding type.
    ///
    /// Can be `credit`, `debit`, `prepaid`, or `unknown`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub funding: Option<String>,

    /// ID of a card PaymentMethod generated from the card_present PaymentMethod that may be attached to a Customer for future transactions.
    ///
    /// Only present if it was possible to generate a card PaymentMethod.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generated_card: Option<String>,

    /// Whether this [PaymentIntent](https://stripe.com/docs/api/payment_intents) is eligible for incremental authorizations.
    ///
    /// Request support using [request_incremental_authorization_support](https://stripe.com/docs/api/payment_intents/create#create_payment_intent-payment_method_options-card_present-request_incremental_authorization_support).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incremental_authorization_supported: Option<bool>,

    /// The last four digits of the card.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last4: Option<String>,

    /// Identifies which network this charge was processed on.
    ///
    /// Can be `amex`, `cartes_bancaires`, `diners`, `discover`, `interac`, `jcb`, `mastercard`, `unionpay`, `visa`, or `unknown`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,

    /// Defines whether the authorized amount can be over-captured or not.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overcapture_supported: Option<bool>,

    /// How card details were read in this transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_method: Option<PaymentMethodDetailsCardPresentReadMethod>,

    /// A collection of fields required to be displayed on receipts.
    ///
    /// Only required for EMV transactions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt: Option<PaymentMethodDetailsCardPresentReceipt>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, JsonSchema)]
pub struct PaymentMethodDetailsCardPresentReceipt {
    /// The type of account being debited or credited.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<PaymentMethodDetailsCardPresentReceiptAccountType>,

    /// EMV tag 9F26, cryptogram generated by the integrated circuit chip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_cryptogram: Option<String>,

    /// Mnenomic of the Application Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_preferred_name: Option<String>,

    /// Identifier for this transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_code: Option<String>,

    /// EMV tag 8A.
    ///
    /// A code returned by the card issuer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_response_code: Option<String>,

    /// How the cardholder verified ownership of the card.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cardholder_verification_method: Option<String>,

    /// EMV tag 84.
    ///
    /// Similar to the application identifier stored on the integrated circuit chip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dedicated_file_name: Option<String>,

    /// The outcome of a series of EMV functions performed by the card reader.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminal_verification_results: Option<String>,

    /// An indication of various EMV functions performed during the transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_status_information: Option<String>,
}

/// An enum representing the possible values of an `PaymentMethodDetailsCardPresent`'s `read_method` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodDetailsCardPresentReadMethod {
    ContactEmv,
    ContactlessEmv,
    ContactlessMagstripeMode,
    MagneticStripeFallback,
    MagneticStripeTrack2,
}

impl PaymentMethodDetailsCardPresentReadMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            PaymentMethodDetailsCardPresentReadMethod::ContactEmv => "contact_emv",
            PaymentMethodDetailsCardPresentReadMethod::ContactlessEmv => "contactless_emv",
            PaymentMethodDetailsCardPresentReadMethod::ContactlessMagstripeMode => {
                "contactless_magstripe_mode"
            }
            PaymentMethodDetailsCardPresentReadMethod::MagneticStripeFallback => {
                "magnetic_stripe_fallback"
            }
            PaymentMethodDetailsCardPresentReadMethod::MagneticStripeTrack2 => {
                "magnetic_stripe_track2"
            }
        }
    }
}

impl AsRef<str> for PaymentMethodDetailsCardPresentReadMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentMethodDetailsCardPresentReadMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for PaymentMethodDetailsCardPresentReadMethod {
    fn default() -> Self {
        Self::ContactEmv
    }
}

/// An enum representing the possible values of an `PaymentMethodDetailsCardPresentReceipt`'s `account_type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodDetailsCardPresentReceiptAccountType {
    Checking,
    Credit,
    Prepaid,
    Unknown,
}

impl PaymentMethodDetailsCardPresentReceiptAccountType {
    pub fn as_str(self) -> &'static str {
        match self {
            PaymentMethodDetailsCardPresentReceiptAccountType::Checking => "checking",
            PaymentMethodDetailsCardPresentReceiptAccountType::Credit => "credit",
            PaymentMethodDetailsCardPresentReceiptAccountType::Prepaid => "prepaid",
            PaymentMethodDetailsCardPresentReceiptAccountType::Unknown => "unknown",
        }
    }
}

impl AsRef<str> for PaymentMethodDetailsCardPresentReceiptAccountType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentMethodDetailsCardPresentReceiptAccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for PaymentMethodDetailsCardPresentReceiptAccountType {
    fn default() -> Self {
        Self::Checking
    }
}
