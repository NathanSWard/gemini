use serde::Deserialize;

pub const fn error_code_reason(ec: u16) -> &'static str {
    match ec {
        200 => "Request was successful",
        300..=399 => "API entry point has moved, see Location: header. Most likely an http: to https: redirect.",
        400 => "Auction not open or paused, ineligible timing, market not open, or the request was malformed; in the case of a private API request, missing or malformed Gemini private API authentication headers",
        403 => "The API key is missing the role necessary to access this private API endpoint",
        404 => "Unknown API entry point or Order not found",
        406 => "Insufficient Funds",
        429 => "Rate Limiting was applied",
        500 => "The server encountered an error",
        502 => "Technical issues are preventing the request from being satisfied",
        503 => "The exchange is down for maintenance",
        _ => "unknown",
    }
}

crate::string_field_impl!(ErrorResult, "error");

#[derive(Deserialize, Clone, Debug)]
pub struct Error {
    result: ErrorResult,
    reason: ErrorReason,
    message: String,
}

#[derive(Deserialize, Clone, Debug)]
pub enum ErrorReason {
    AuctionNotOpen, //Failed to place an auction-only order because there is no current auction open for this symbol
    ClientOrderIdTooLong, // 	The Client Order ID must be under 100 characters
    ClientOrderIdMustBeString, // 	The Client Order ID must be a string
    ConflictingOptions, // 	New orders using a combination of order execution options are not supported
    ConflictingAccountName, // 	The specified name is already in use within the master group
    EndpointMismatch, // 	The request was submitted to an endpoint different than the one in the payload
    EndpointNotFound, // 	No endpoint was specified
    GTSTradeIDMustBeString, // 	The Clearing ID must be a string
    IneligibleTiming, // 	Failed to place an auction order for the current auction on this symbol because the timing is not eligible: new orders may only be placed before the auction begins.
    InsufficientFunds, // 	The order was rejected because of insufficient funds
    InvalidJson,      // 	The JSON provided is invalid
    InvalidNonce, // 	The nonce was not greater than the previously used nonce, or was not present
    InvalidOrderType, // 	An unknown order type was provided
    InvalidParameterValue,
    InvalidPrice,               // 	For new orders, the price was invalid
    InvalidStopPrice,           // 	For new stop limit orders, the price was invalid
    InvalidStopPriceSell, // 	For new stop limit sell orders, the "stop_price" price was lower than the "sell" price
    InvalidStopPriceBuy, // 	For new stop limit buy orders, the "stop_price" price was greater than the "buy" price
    InvalidStopPriceRatio, // 	For new stop limit orders, the "buy" or "sell" price was not within 50% of the "stop_price"
    InvalidQuantity,       // 	A negative or otherwise invalid quantity was specified
    InvalidSide,           //	For new orders, and invalid side was specified
    InvalidSignature,      // 	The signature did not match the expected signature
    InvalidSymbol,         // 	An invalid symbol was specified
    InvalidTimestampInPayload, // 	The JSON payload contained a timestamp parameter with an unsupported value.
    InvalidAccountName, // 	The specified name did not match any accounts within the master group
    InvalidAccountType, // 	The specified type did not match exchange or custody
    InvalidFundTransfer, // 	The fund transfer was not successful
    Maintenance,        // 	The system is down for maintenance
    MarketNotOpen,      // 	The order was rejected because the market is not accepting new orders
    MissingAccountName, // 	A required account name was not specified in a field requiring one
    MissingAccounts,    // 	A required account field was not specified
    MissingApikeyHeader, // 	The X-GEMINI-APIKEY header was missing
    MissingOrderField,  // 	A required order_id field was not specified
    MissingRole, // 	The API key used to access this endpoint does not have the required role assigned to it
    MissingPayloadHeader, // 	The X-GEMINI-PAYLOAD header was missing
    MissingPayloadKey, // 	The payload is missing a required key
    MissingSignatureHeader, // 	The X-GEMINI-SIGNATURE header was missing
    MissingName, // 	A required name field was not specified
    MissingNonce, // 	A nonce was not provided in the payload. See Private API Invocation for more detail.
    MoreThanOneAccount, // 	More than one account was specified on an API that only accepts a single account
    AccountsOnGroupOnlyApi, // 	The account field was specified on a non-master API key
    AccountLimitExceeded, // 	The account field specified more than the maximum supported accounts for that API
    NoAccountOfTypeRequired, // 	The account field specified multiple accounts and some were not of the required account type
    AccountNotOfTypeRequired, // 	The account specified in the account field was not of the required account type
    NotGroupApiCompatible,    // 	A master API key was used to invoke an account only API
    ExceededMaxAccountsInGroup, // 	An account could not be created as the master group already has the maximum number of allowed accounts in it
    NoSSL,                      // 	You must use HTTPS to access the API
    OptionsMustBeArray,         // 	The options parameter must be an array.
    OrderNotFound,              // 	The order specified was not found
    RateLimit,                  // 	Requests were made too frequently. See Rate Limits below.
    System,                     // 	We are experiencing technical issues
    UnsupportedOption,          // 	This order execution option is not supported.
    HasNotAgreedToCustodyTerms, // 	The Group has not yet agreed to the Custody terms and conditions. Please visit https://exchange.gemini.com/custody to read the terms and conditions of custody accounts.
    BadAccountType, // 	The type parameter must contain a string of either exchange or custody.
}
