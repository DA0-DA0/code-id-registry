use cosmwasm_std::StdError;
use cw_utils::PaymentError;
use thiserror::Error;

#[derive(Error, Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    Payment(#[from] PaymentError),

    #[error("Unauthorized; only admin may register or unregister code ID")]
    UnauthorizedRegistration {},

    #[error("Unauthorized; only admin may update admin")]
    UnauthorizedUpdateAdmin {},

    #[error("Incorrect payment amount")]
    IncorrectPaymentAmount {},

    #[error("Contract not found")]
    NotFound {},

    #[error("Code ID {0} has already been registered on chain {1}")]
    CodeIDAlreadyRegistered(u64, String),

    #[error("Invalid CW20, this address is not a CW20")]
    InvalidCw20 {},

    #[error("This CW20's address does not match the configured CW20 payment address")]
    UnrecognizedCw20 {},

    #[error("This token's denom does not match the configured token's denom")]
    UnrecognizedNativeToken {},

    #[error("Invalid payment")]
    InvalidPayment {},
}
