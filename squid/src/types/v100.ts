import type {Result, Option} from './support'

export type ImplBuildStatus = ImplBuildStatus_Released | ImplBuildStatus_Deprecated | ImplBuildStatus_Retired

export interface ImplBuildStatus_Released {
    __kind: 'Released'
}

export interface ImplBuildStatus_Deprecated {
    __kind: 'Deprecated'
}

export interface ImplBuildStatus_Retired {
    __kind: 'Retired'
}

export type ApplicableScope = ApplicableScope_Owner | ApplicableScope_Public

export interface ApplicableScope_Owner {
    __kind: 'Owner'
}

export interface ApplicableScope_Public {
    __kind: 'Public'
}

export type AttestationMethod = AttestationMethod_OptOut

export interface AttestationMethod_OptOut {
    __kind: 'OptOut'
}

export type OfflineReason = OfflineReason_Graceful | OfflineReason_Forced | OfflineReason_Unresponsive | OfflineReason_AttestationExpired | OfflineReason_ImplBuildRetired | OfflineReason_InsufficientDepositFunds | OfflineReason_Other

export interface OfflineReason_Graceful {
    __kind: 'Graceful'
}

export interface OfflineReason_Forced {
    __kind: 'Forced'
}

export interface OfflineReason_Unresponsive {
    __kind: 'Unresponsive'
}

export interface OfflineReason_AttestationExpired {
    __kind: 'AttestationExpired'
}

export interface OfflineReason_ImplBuildRetired {
    __kind: 'ImplBuildRetired'
}

export interface OfflineReason_InsufficientDepositFunds {
    __kind: 'InsufficientDepositFunds'
}

export interface OfflineReason_Other {
    __kind: 'Other'
}

export type JobResult = JobResult_Success | JobResult_Fail | JobResult_Error | JobResult_Panic

export interface JobResult_Success {
    __kind: 'Success'
}

export interface JobResult_Fail {
    __kind: 'Fail'
}

export interface JobResult_Error {
    __kind: 'Error'
}

export interface JobResult_Panic {
    __kind: 'Panic'
}

export type JobStatus = JobStatus_Pending | JobStatus_Processing | JobStatus_Processed | JobStatus_Discarded

export interface JobStatus_Pending {
    __kind: 'Pending'
}

export interface JobStatus_Processing {
    __kind: 'Processing'
}

export interface JobStatus_Processed {
    __kind: 'Processed'
}

export interface JobStatus_Discarded {
    __kind: 'Discarded'
}
