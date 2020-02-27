import * as externals from "./externals";

const SYSTEM_CONTRACT_ERROR_CODE_OFFSET: u32 = 65024;
const POS_ERROR_CODE_OFFSET: u32 = 65280;
const USER_ERROR_CODE_OFFSET: u32 = 65535;

export const enum ErrorCode {
    None = 1,
    MissingArgument = 2,
    InvalidArgument = 3,
    Deserialize = 4,
    Read = 5,
    ValueNotFound = 6,
    ContractNotFound = 7,
    GetKey = 8,
    UnexpectedKeyVariant = 9,
    UnexpectedContractRefVariant = 10,
    InvalidPurseName = 11,
    InvalidPurse = 12,
    UpgradeContractAtURef = 13,
    Transfer = 14,
    NoAccessRights = 15,
    CLTypeMismatch = 16,
    EarlyEndOfStream = 17,
    Formatting = 18,
    LeftOverBytes = 19,
    OutOfMemory = 20,
    MaxKeysLimit = 21,
    DuplicateKey = 22,
    PermissionDenied = 23,
    MissingKey = 24,
    ThresholdViolation = 25,
    KeyManagementThreshold = 26,
    DeploymentThreshold = 27,
    InsufficientTotalWeight = 28,
    InvalidSystemContract = 29,
    PurseNotCreated = 30,
    Unhandled = 31,
    BufferTooSmall = 32,
    HostBufferEmpty = 33,
    HostBufferFull = 34,
}

export const enum PosErrorCode {
    NotBonded = 0,
    TooManyEventsInQueue = 1,
    CannotUnbondLastValidator = 2,
    SpreadTooHigh = 3,
    MultipleRequests = 4,
    BondTooSmall = 5,
    BondTooLarge = 6,
    UnbondTooLarge = 7,
    BondTransferFailed = 8,
    UnbondTransferFailed = 9,
    MissingArgument = 10,
    InvalidArgument = 11,
    TimeWentBackwards = 12,
    StakesNotFound = 13,
    PaymentPurseNotFound = 14,
    PaymentPurseKeyUnexpectedType = 15,
    PaymentPurseBalanceNotFound = 16,
    BondingPurseNotFound = 17,
    BondingPurseKeyUnexpectedType = 18,
    RefundPurseKeyUnexpectedType = 19,
    RewardsPurseNotFound = 20,
    RewardsPurseKeyUnexpectedType = 21,
    QueueNotStoredAsByteArray = 22,
    QueueDeserializationFailed = 23,
    QueueDeserializationExtraBytes = 24,
    StakesKeyDeserializationFailed = 25,
    StakesDeserializationFailed = 26,
    SystemFunctionCalledByUserAccount = 27,
    InsufficientPaymentForAmountSpent = 28,
    FailedTransferToRewardsPurse = 29,
    FailedTransferToAccountPurse = 30,
    SetRefundPurseCalledOutsidePayment = 31,
}

export class Error{
    private errorCodeValue: u32;

    constructor(value: u32) {
        this.errorCodeValue = value;
    }

    static fromResult(result: u32): Error | null {
        if (result == 0) {
            // Ok
            return null;
        }
        return new Error(result);
    }

    static fromUserError(userErrorCodeValue: u16): Error {
        return new Error(USER_ERROR_CODE_OFFSET + 1 + userErrorCodeValue);
    }

    static fromErrorCode(errorCode: ErrorCode): Error {
        return new Error(<u32>errorCode);
    }

    static fromPosErrorCode(errorCode: PosErrorCode): Error {
        return new Error(<u32>errorCode + POS_ERROR_CODE_OFFSET);
    }

    value(): u32{
        return this.errorCodeValue;
    }

    isUserError(): bool{
        return this.errorCodeValue > USER_ERROR_CODE_OFFSET;
    }

    isSystemContractError(): bool{
        return this.errorCodeValue >= SYSTEM_CONTRACT_ERROR_CODE_OFFSET && this.errorCodeValue <= USER_ERROR_CODE_OFFSET;
    }

    revert(): void {
        externals.revert(this.errorCodeValue);
    }
}
