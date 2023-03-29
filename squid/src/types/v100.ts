import type {Result, Option} from './support'

export interface CreatingTaskPolicy {
    permission: CreatingTaskPermission
    startBlock: (number | undefined)
    endBlock: (number | undefined)
}

export type TaskResult = TaskResult_Success | TaskResult_Failed | TaskResult_Errored

export interface TaskResult_Success {
    __kind: 'Success'
}

export interface TaskResult_Failed {
    __kind: 'Failed'
}

export interface TaskResult_Errored {
    __kind: 'Errored'
}

export type TaskStatus = TaskStatus_Pending | TaskStatus_Processing | TaskStatus_Processed

export interface TaskStatus_Pending {
    __kind: 'Pending'
}

export interface TaskStatus_Processing {
    __kind: 'Processing'
}

export interface TaskStatus_Processed {
    __kind: 'Processed'
}

export type OfflineReason = OfflineReason_Graceful | OfflineReason_Forced | OfflineReason_Unresponsive | OfflineReason_AttestationExpired | OfflineReason_WorkerImplBlocked | OfflineReason_InsufficientReservedFunds | OfflineReason_Other

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

export interface OfflineReason_WorkerImplBlocked {
    __kind: 'WorkerImplBlocked'
}

export interface OfflineReason_InsufficientReservedFunds {
    __kind: 'InsufficientReservedFunds'
}

export interface OfflineReason_Other {
    __kind: 'Other'
    value: (Uint8Array | undefined)
}

export type AttestationMethod = AttestationMethod_NonTEE | AttestationMethod_Root

export interface AttestationMethod_NonTEE {
    __kind: 'NonTEE'
}

export interface AttestationMethod_Root {
    __kind: 'Root'
}

export type CreatingTaskPermission = CreatingTaskPermission_Owner | CreatingTaskPermission_Public

export interface CreatingTaskPermission_Owner {
    __kind: 'Owner'
}

export interface CreatingTaskPermission_Public {
    __kind: 'Public'
}
