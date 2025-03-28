use anchor_lang::prelude::*;

pub use TakesFunError::*;

#[error_code]
pub enum TakesFunError {
    #[msg("ValueTooSmall")]
    ValueTooSmall,

    #[msg("ValueTooLarge")]
    ValueTooLarge,

    #[msg("ValueInvalid")]
    ValueInvalid,

    #[msg("IncorrectConfigAccount")]
    IncorrectConfigAccount,

    #[msg("IncorrectAuthority")]
    IncorrectAuthority,

    #[msg("Insufficient Real Sol Reserves")]
    InsufficientRealSolReserves,

    #[msg("Invalid Expected Real Sol Reserves")]
    InvalidExpectedRealSolReserves,

    #[msg("Overflow or underflow occured")]
    OverflowOrUnderflowOccurred,

    #[msg("Amount is invalid")]
    InvalidAmount,

    #[msg("Incorrect team wallet address")]
    IncorrectTeamWallet,

    #[msg("Curve is not completed")]
    CurveNotCompleted,

    #[msg("Can not swap after the market is completed")]
    MarketAlreadyCompleted,

    #[msg("Mint authority should be revoked")]
    MintAuthorityEnabled,

    #[msg("Freeze authority should be revoked")]
    FreezeAuthorityEnabled,

    #[msg("Return amount is too small compared to the minimum received amount")]
    ReturnAmountTooSmall,

    #[msg("AMM is already exist")]
    AmmAlreadyExists,

    #[msg("Global Not Initialized")]
    NotInitialized,

    #[msg("Invalid Global Authority")]
    InvalidGlobalAuthority,

    #[msg("This creator is not in whitelist")]
    NotWhiteList,

    #[msg("IncorrectLaunchPhase")]
    IncorrectLaunchPhase,

    #[msg("Not enough tokens to complete the sell order.")]
    InsufficientTokens,

    #[msg("Not enough SOL received to be valid.")]
    InsufficientSol,

    #[msg("Sell Failed")]
    SellFailed,

    #[msg("Buy Failed")]
    BuyFailed,

    #[msg("This token is not a bonding curve token")]
    NotBondingCurveMint,

    #[msg("Not quote mint")]
    NotSOL,

    #[msg("Invalid Migration Authority")]
    InvalidMigrationAuthority,

    #[msg("Bonding curve is not completed")]
    NotCompleted,

    #[msg("Invalid Meteora Program")]
    InvalidMeteoraProgram,

    #[msg("Arithmetic Error")]
    ArithmeticError,

    #[msg("Slippage Exceeded")]
    SlippageExceeded,

    #[msg("buy token amount invalid")]
    BuyTokenAmountInvalid,

    #[msg("incorrect yes token")]
    IncorrectYesToken,

    #[msg("incorrect no token")]
    IncorrectNoToken,
}
