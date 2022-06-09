//! Structs of unknown origin that aren't picked
//! up by the codegen.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize, JsonSchema)]
pub struct CountrySpecSupportedBankAccountCurrencies {}

#[derive(Clone, Debug, Default, Deserialize, Serialize, JsonSchema)]
pub struct ExchangeRateRates {}

#[derive(Clone, Debug, Default, Deserialize, Serialize, JsonSchema)]
pub struct IssuingAuthorizationAmountDetails {}

#[derive(Clone, Debug, Default, Deserialize, Serialize, JsonSchema)]
pub struct NotificationEventData {}

#[derive(Clone, Debug, Default, Deserialize, Serialize, JsonSchema)]
pub struct PaymentMethodDetailsCardInstallmentsPlan {}

#[derive(Clone, Debug, Default, Deserialize, Serialize, JsonSchema)]
pub struct PaymentMethodDetailsCardPresent {}

#[derive(Clone, Debug, Default, Deserialize, Serialize, JsonSchema)]
pub struct PaymentMethodDetailsCardWallet {}

#[derive(Clone, Debug, Default, Deserialize, Serialize, JsonSchema)]
pub struct RadarValueListItem {}

#[derive(Clone, Debug, Default, Deserialize, Serialize, JsonSchema)]
pub struct SubscriptionTransferData {}

#[derive(Clone, Debug, Default, Deserialize, Serialize, JsonSchema)]
pub struct ThreeDSecureDetails {}
