use anchor_lang::prelude::*;

#[error_code]
pub enum EscrowError {
    #[msg("The current stage is not valid for an exchange or cancellation.")]
    InvalidStage,
    #[msg("The available funds are insufficient for this operation.")]
    InsufficientFunds,
    #[msg("The mint account specified for the trade is invalid.")]
    InvalidMint,
    #[msg("A required mint for the trade is missing.")]
    MissingMint,
    #[msg("The trade type is invalid, possibly due to missing mint addresses.")]
    InvalidTradeType,
    #[msg("Invalid mint association between the provided token accounts.")]
    InvalidAccount,
    #[msg("Duplicate mint accounts are not allowed for this operation.")]
    DuplicateMint,
    #[msg("The account does not have a valid owner.")]
    InvalidOwner,
    #[msg("The specified partner is not valid for this trade.")]
    InvalidPartner,
    #[msg("Both trade value and receive value must be greater than zero.")]
    ZeroValue,
    #[msg("Instruction data is missing required parameters.")]
    MissingParams,
}
