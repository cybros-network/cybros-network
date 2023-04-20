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

export type ImplBuildStatus = ImplBuildStatus_Released | ImplBuildStatus_Deprecated | ImplBuildStatus_Blocked

export interface ImplBuildStatus_Released {
    __kind: 'Released'
}

export interface ImplBuildStatus_Deprecated {
    __kind: 'Deprecated'
}

export interface ImplBuildStatus_Blocked {
    __kind: 'Blocked'
}

export type ImplDeploymentPermission = ImplDeploymentPermission_Owner | ImplDeploymentPermission_Public

export interface ImplDeploymentPermission_Owner {
    __kind: 'Owner'
}

export interface ImplDeploymentPermission_Public {
    __kind: 'Public'
}

export type AttestationMethod = AttestationMethod_OptOut

export interface AttestationMethod_OptOut {
    __kind: 'OptOut'
}

export type OfflineReason = OfflineReason_Graceful | OfflineReason_Forced | OfflineReason_Unresponsive | OfflineReason_AttestationExpired | OfflineReason_ImplBlocked | OfflineReason_InsufficientDepositFunds | OfflineReason_Other

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

export interface OfflineReason_ImplBlocked {
    __kind: 'ImplBlocked'
}

export interface OfflineReason_InsufficientDepositFunds {
    __kind: 'InsufficientDepositFunds'
}

export interface OfflineReason_Other {
    __kind: 'Other'
}

export type CreatingTaskPermission = CreatingTaskPermission_Owner | CreatingTaskPermission_Public

export interface CreatingTaskPermission_Owner {
    __kind: 'Owner'
}

export interface CreatingTaskPermission_Public {
    __kind: 'Public'
}
