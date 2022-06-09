// ======================================
// This file was automatically generated.
// ======================================

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::ids::TreasuryInboundTransferId;
use crate::params::{Expandable, Metadata, Object, Timestamp};
use crate::resources::{Currency, TreasuryTransaction, UfaResourceBillingDetails};

/// The resource representing a Stripe "InboundTransfersResourceTreasuryInboundTransfer".
#[derive(Clone, Debug, Default, Deserialize, Serialize, JsonSchema)]
pub struct TreasuryInboundTransfer {
    /// Unique identifier for the object.
    pub id: TreasuryInboundTransferId,

    /// Amount (in cents) transferred.
    pub amount: i64,

    /// Returns `true` if the InboundTransfer is able to be canceled.
    pub cancelable: bool,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: Currency,

    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Details about this InboundTransfer's failure.
    ///
    /// Only set when status is `failed`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_details: Option<InboundTransfersResourceFailureDetails>,

    /// The FinancialAccount that received the funds.
    pub financial_account: String,

    /// A hosted transaction receipt URL that is provided when money movement is considered regulated under Stripe's money transmission licenses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosted_regulatory_receipt_url: Option<String>,

    pub linked_flows: InboundTransfersResourceTreasuryInboundTransferResourceLinkedFlows,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Metadata,

    /// The origin payment method to be debited for an InboundTransfer.
    pub origin_payment_method: String,

    /// Details about the PaymentMethod for an InboundTransfer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_payment_method_details: Option<InboundTransfers>,

    /// Returns `true` if the funds for an InboundTransfer were returned after the InboundTransfer went to the `succeeded` state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub returned: Option<bool>,

    /// Statement descriptor shown when funds are debited from the source.
    ///
    /// Not all payment networks support `statement_descriptor`.
    pub statement_descriptor: String,

    /// Status of the InboundTransfer: `processing`, `succeeded`, `failed`, and `canceled`.
    ///
    /// An InboundTransfer is `processing` if it is created and pending.
    /// The status changes to `succeeded` once the funds have been "confirmed" and a `transaction` is created and posted.
    /// The status changes to `failed` if the transfer fails.
    pub status: TreasuryInboundTransferStatus,

    pub status_transitions: InboundTransfersResourceInboundTransferResourceStatusTransitions,

    /// The Transaction associated with this object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction: Option<Expandable<TreasuryTransaction>>,
}

impl Object for TreasuryInboundTransfer {
    type Id = TreasuryInboundTransferId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "treasury.inbound_transfer"
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, JsonSchema)]
pub struct InboundTransfers {
    pub billing_details: UfaResourceBillingDetails,

    /// The type of the payment method used in the InboundTransfer.
    #[serde(rename = "type")]
    pub type_: InboundTransfersType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<InboundTransfersPaymentMethodDetailsUsBankAccount>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, JsonSchema)]
pub struct InboundTransfersPaymentMethodDetailsUsBankAccount {
    /// Account holder type: individual or company.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_holder_type:
        Option<InboundTransfersPaymentMethodDetailsUsBankAccountAccountHolderType>,

    /// Account type: checkings or savings.
    ///
    /// Defaults to checking if omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<InboundTransfersPaymentMethodDetailsUsBankAccountAccountType>,

    /// Name of the bank associated with the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_name: Option<String>,

    /// Uniquely identifies this particular bank account.
    ///
    /// You can use this attribute to check whether two bank accounts are the same.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,

    /// Last four digits of the bank account number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last4: Option<String>,

    /// The US bank account network used to debit funds.
    pub network: InboundTransfersPaymentMethodDetailsUsBankAccountNetwork,

    /// Routing number of the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_number: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, JsonSchema)]
pub struct InboundTransfersResourceFailureDetails {
    /// Reason for the failure.
    pub code: InboundTransfersResourceFailureDetailsCode,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, JsonSchema)]
pub struct InboundTransfersResourceInboundTransferResourceStatusTransitions {
    /// Timestamp describing when an InboundTransfer changed status to `canceled`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canceled_at: Option<Timestamp>,

    /// Timestamp describing when an InboundTransfer changed status to `failed`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_at: Option<Timestamp>,

    /// Timestamp describing when an InboundTransfer changed status to `succeeded`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub succeeded_at: Option<Timestamp>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, JsonSchema)]
pub struct InboundTransfersResourceTreasuryInboundTransferResourceLinkedFlows {
    /// If funds for this flow were returned after the flow went to the `succeeded` state, this field contains a reference to the ReceivedDebit return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub received_debit: Option<String>,
}

/// An enum representing the possible values of an `InboundTransfersPaymentMethodDetailsUsBankAccount`'s `account_holder_type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum InboundTransfersPaymentMethodDetailsUsBankAccountAccountHolderType {
    Company,
    Individual,
}

impl InboundTransfersPaymentMethodDetailsUsBankAccountAccountHolderType {
    pub fn as_str(self) -> &'static str {
        match self {
            InboundTransfersPaymentMethodDetailsUsBankAccountAccountHolderType::Company => {
                "company"
            }
            InboundTransfersPaymentMethodDetailsUsBankAccountAccountHolderType::Individual => {
                "individual"
            }
        }
    }
}

impl AsRef<str> for InboundTransfersPaymentMethodDetailsUsBankAccountAccountHolderType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for InboundTransfersPaymentMethodDetailsUsBankAccountAccountHolderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for InboundTransfersPaymentMethodDetailsUsBankAccountAccountHolderType {
    fn default() -> Self {
        Self::Company
    }
}

/// An enum representing the possible values of an `InboundTransfersPaymentMethodDetailsUsBankAccount`'s `account_type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum InboundTransfersPaymentMethodDetailsUsBankAccountAccountType {
    Checking,
    Savings,
}

impl InboundTransfersPaymentMethodDetailsUsBankAccountAccountType {
    pub fn as_str(self) -> &'static str {
        match self {
            InboundTransfersPaymentMethodDetailsUsBankAccountAccountType::Checking => "checking",
            InboundTransfersPaymentMethodDetailsUsBankAccountAccountType::Savings => "savings",
        }
    }
}

impl AsRef<str> for InboundTransfersPaymentMethodDetailsUsBankAccountAccountType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for InboundTransfersPaymentMethodDetailsUsBankAccountAccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for InboundTransfersPaymentMethodDetailsUsBankAccountAccountType {
    fn default() -> Self {
        Self::Checking
    }
}

/// An enum representing the possible values of an `InboundTransfersPaymentMethodDetailsUsBankAccount`'s `network` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum InboundTransfersPaymentMethodDetailsUsBankAccountNetwork {
    Ach,
}

impl InboundTransfersPaymentMethodDetailsUsBankAccountNetwork {
    pub fn as_str(self) -> &'static str {
        match self {
            InboundTransfersPaymentMethodDetailsUsBankAccountNetwork::Ach => "ach",
        }
    }
}

impl AsRef<str> for InboundTransfersPaymentMethodDetailsUsBankAccountNetwork {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for InboundTransfersPaymentMethodDetailsUsBankAccountNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for InboundTransfersPaymentMethodDetailsUsBankAccountNetwork {
    fn default() -> Self {
        Self::Ach
    }
}

/// An enum representing the possible values of an `InboundTransfersResourceFailureDetails`'s `code` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum InboundTransfersResourceFailureDetailsCode {
    AccountClosed,
    AccountFrozen,
    BankAccountRestricted,
    BankOwnershipChanged,
    DebitNotAuthorized,
    IncorrectAccountHolderAddress,
    IncorrectAccountHolderName,
    IncorrectAccountHolderTaxId,
    InsufficientFunds,
    InvalidAccountNumber,
    InvalidCurrency,
    NoAccount,
    Other,
}

impl InboundTransfersResourceFailureDetailsCode {
    pub fn as_str(self) -> &'static str {
        match self {
            InboundTransfersResourceFailureDetailsCode::AccountClosed => "account_closed",
            InboundTransfersResourceFailureDetailsCode::AccountFrozen => "account_frozen",
            InboundTransfersResourceFailureDetailsCode::BankAccountRestricted => {
                "bank_account_restricted"
            }
            InboundTransfersResourceFailureDetailsCode::BankOwnershipChanged => {
                "bank_ownership_changed"
            }
            InboundTransfersResourceFailureDetailsCode::DebitNotAuthorized => {
                "debit_not_authorized"
            }
            InboundTransfersResourceFailureDetailsCode::IncorrectAccountHolderAddress => {
                "incorrect_account_holder_address"
            }
            InboundTransfersResourceFailureDetailsCode::IncorrectAccountHolderName => {
                "incorrect_account_holder_name"
            }
            InboundTransfersResourceFailureDetailsCode::IncorrectAccountHolderTaxId => {
                "incorrect_account_holder_tax_id"
            }
            InboundTransfersResourceFailureDetailsCode::InsufficientFunds => "insufficient_funds",
            InboundTransfersResourceFailureDetailsCode::InvalidAccountNumber => {
                "invalid_account_number"
            }
            InboundTransfersResourceFailureDetailsCode::InvalidCurrency => "invalid_currency",
            InboundTransfersResourceFailureDetailsCode::NoAccount => "no_account",
            InboundTransfersResourceFailureDetailsCode::Other => "other",
        }
    }
}

impl AsRef<str> for InboundTransfersResourceFailureDetailsCode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for InboundTransfersResourceFailureDetailsCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for InboundTransfersResourceFailureDetailsCode {
    fn default() -> Self {
        Self::AccountClosed
    }
}

/// An enum representing the possible values of an `InboundTransfers`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum InboundTransfersType {
    UsBankAccount,
}

impl InboundTransfersType {
    pub fn as_str(self) -> &'static str {
        match self {
            InboundTransfersType::UsBankAccount => "us_bank_account",
        }
    }
}

impl AsRef<str> for InboundTransfersType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for InboundTransfersType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for InboundTransfersType {
    fn default() -> Self {
        Self::UsBankAccount
    }
}

/// An enum representing the possible values of an `TreasuryInboundTransfer`'s `status` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum TreasuryInboundTransferStatus {
    Canceled,
    Failed,
    Processing,
    Succeeded,
}

impl TreasuryInboundTransferStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            TreasuryInboundTransferStatus::Canceled => "canceled",
            TreasuryInboundTransferStatus::Failed => "failed",
            TreasuryInboundTransferStatus::Processing => "processing",
            TreasuryInboundTransferStatus::Succeeded => "succeeded",
        }
    }
}

impl AsRef<str> for TreasuryInboundTransferStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TreasuryInboundTransferStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for TreasuryInboundTransferStatus {
    fn default() -> Self {
        Self::Canceled
    }
}
