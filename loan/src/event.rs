use near_sdk::{env, AccountId};
use serde::Serialize;
use near_sdk::json_types::U128;
use crate::base::TokenId;

// storage

#[must_use]
#[derive(Serialize, Debug, Clone)]
pub struct LoanFtDeposit<'a> {
  pub account_id: &'a AccountId,
  pub amount: &'a U128,
}

impl LoanFtDeposit<'_> {
  pub fn emit(self) {
    Self::emit_many(&[self])
  }

  pub fn emit_many<'a>(data: &'a [LoanFtDeposit<'a>]) {
    new_loan_v1(NepLoanEventKind::LoanFtDeposit(data)).emit()
  }
}

#[must_use]
#[derive(Serialize, Debug, Clone)]
pub struct LoanFtWithdraw<'a> {
  pub account_id: &'a AccountId,
  pub amount: &'a U128,
}

impl LoanFtWithdraw<'_> {
  pub fn emit(self) {
    Self::emit_many(&[self])
  }

  pub fn emit_many<'a>(data: &'a [LoanFtWithdraw<'a>]) {
    new_loan_v1(NepLoanEventKind::LoanFtWithdraw(data)).emit()
  }
}

#[must_use]
#[derive(Serialize, Debug, Clone)]
pub struct LoanFtClaimRewards<'a> {
  pub account_id: &'a AccountId,
  pub amount: &'a U128,
}

impl LoanFtClaimRewards<'_> {
  pub fn emit(self) {
    Self::emit_many(&[self])
  }

  pub fn emit_many<'a>(data: &'a [LoanFtClaimRewards<'a>]) {
    new_loan_v1(NepLoanEventKind::LoanFtClaimRewards(data)).emit()
  }
}

// whitelist

#[must_use]
#[derive(Serialize, Debug, Clone)]
pub struct LoanWhitelistAdd<'a> {
  pub contract_id: &'a AccountId,
}

impl LoanWhitelistAdd<'_> {
  pub fn emit(self) {
    Self::emit_many(&[self])
  }

  pub fn emit_many<'a>(data: &'a [LoanWhitelistAdd<'a>]) {
    new_loan_v1(NepLoanEventKind::LoanWhitelistAdd(data)).emit()
  }
}

#[must_use]
#[derive(Serialize, Debug, Clone)]
pub struct LoanWhitelistRemove<'a> {
  pub contract_id: &'a AccountId,
}

impl LoanWhitelistRemove<'_> {
  pub fn emit(self) {
    Self::emit_many(&[self])
  }

  pub fn emit_many<'a>(data: &'a [LoanWhitelistRemove<'a>]) {
    new_loan_v1(NepLoanEventKind::LoanWhitelistRemove(data)).emit()
  }
}

#[must_use]
#[derive(Serialize, Debug, Clone)]
pub struct LoanWhitelistUpdatePrice<'a> {
  pub contract_id: &'a AccountId,
  pub price: &'a U128,
  pub percent: &'a u64,
}

impl LoanWhitelistUpdatePrice<'_> {
  pub fn emit(self) {
    Self::emit_many(&[self])
  }

  pub fn emit_many<'a>(data: &'a [LoanWhitelistUpdatePrice<'a>]) {
    new_loan_v1(NepLoanEventKind::LoanWhitelistUpdatePrice(data)).emit()
  }
}

// base

#[must_use]
#[derive(Serialize, Debug, Clone)]
pub struct LoanNft<'a> {
  pub owner_id: &'a AccountId,
  pub contract_id: &'a AccountId,
  pub token_id: &'a TokenId,
  pub expire_date: &'a u64,
  pub loan_amount: &'a U128,
  pub price: &'a U128,
}

impl LoanNft<'_> {
  pub fn emit(self) {
    Self::emit_many(&[self])
  }

  pub fn emit_many<'a>(data: &'a [LoanNft<'a>]) {
    new_loan_v1(NepLoanEventKind::LoanNft(data)).emit()
  }
}

#[must_use]
#[derive(Serialize, Debug, Clone)]
pub struct LoanNftPay<'a> {
  pub owner_id: &'a AccountId,
  pub contract_id: &'a AccountId,
  pub token_id: &'a TokenId,
  pub loan_amount: &'a U128,
}

impl LoanNftPay<'_> {
  pub fn emit(self) {
    Self::emit_many(&[self])
  }

  pub fn emit_many<'a>(data: &'a [LoanNftPay<'a>]) {
    new_loan_v1(NepLoanEventKind::LoanNftPay(data)).emit()
  }
}

#[must_use]
#[derive(Serialize, Debug, Clone)]
pub struct LoanNftClaim<'a> {
  pub receiver_id: &'a AccountId,
  pub contract_id: &'a AccountId,
  pub token_id: &'a TokenId,
}

impl LoanNftClaim<'_> {
  pub fn emit(self) {
    Self::emit_many(&[self])
  }

  pub fn emit_many<'a>(data: &'a [LoanNftClaim<'a>]) {
    new_loan_v1(NepLoanEventKind::LoanNftClaim(data)).emit()
  }
}

#[must_use]
#[derive(Serialize, Debug, Clone)]
pub struct LoanNftClaimExpired<'a> {
  pub old_owner_id: &'a AccountId,
  pub contract_id: &'a AccountId,
  pub token_id: &'a TokenId,
}

impl LoanNftClaimExpired<'_> {
  pub fn emit(self) {
    Self::emit_many(&[self])
  }

  pub fn emit_many<'a>(data: &'a [LoanNftClaimExpired<'a>]) {
    new_loan_v1(NepLoanEventKind::LoanNftClaimExpired(data)).emit()
  }
}

//

#[derive(Serialize, Debug)]
pub(crate) struct NepLoanEvent<'a> {
  version: &'static str,
  #[serde(flatten)]
  event_kind: NepLoanEventKind<'a>,
}

#[derive(Serialize, Debug)]
#[serde(tag = "event", content = "data")]
#[serde(rename_all = "snake_case")]
#[allow(clippy::enum_variant_names)]
enum NepLoanEventKind<'a> {
  LoanFtDeposit(&'a [LoanFtDeposit<'a>]),
  LoanFtWithdraw(&'a [LoanFtWithdraw<'a>]),
  LoanFtClaimRewards(&'a [LoanFtClaimRewards<'a>]),

  LoanWhitelistAdd(&'a [LoanWhitelistAdd<'a>]),
  LoanWhitelistRemove(&'a [LoanWhitelistRemove<'a>]),
  LoanWhitelistUpdatePrice(&'a [LoanWhitelistUpdatePrice<'a>]),

  LoanNft(&'a [LoanNft<'a>]),
  LoanNftPay(&'a [LoanNftPay<'a>]),
  LoanNftClaim(&'a [LoanNftClaim<'a>]),
  LoanNftClaimExpired(&'a [LoanNftClaimExpired<'a>]),
}

fn new_loan<'a>(version: &'static str, event_kind: NepLoanEventKind<'a>) -> NearEvent<'a> {
  NearEvent::NepLoan(NepLoanEvent { version, event_kind })
}

fn new_loan_v1(event_kind: NepLoanEventKind) -> NearEvent {
  new_loan("1.0.0", event_kind)
}

// base

#[derive(Serialize, Debug)]
#[serde(tag = "standard")]
#[must_use = "don't forget to `.emit()` this event"]
#[serde(rename_all = "snake_case")]
pub(crate) enum NearEvent<'a> {
    // Nep171(crate::non_fungible_token::events::Nep171Event<'a>),
    // Nep141(crate::base::Nep141Event<'a>),
  NepLoan(NepLoanEvent<'a>),
}

impl<'a> NearEvent<'a> {
    fn to_json_string(&self) -> String {
        // Events cannot fail to serialize so fine to panic on error
        #[allow(clippy::redundant_closure)]
        serde_json::to_string(self).ok().unwrap_or_else(|| env::abort())
    }

    fn to_json_event_string(&self) -> String {
        format!("EVENT_JSON:{}", self.to_json_string())
    }

    /// Logs the event to the host. This is required to ensure that the event is triggered
    /// and to consume the event.
    pub(crate) fn emit(self) {
        near_sdk::env::log_str(&self.to_json_event_string());
    }
}
