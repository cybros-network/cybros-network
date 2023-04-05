import { type Context } from "../processor"
import {
    OffchainComputingPoolCreatedEvent as PoolCreatedEvent,
    OffchainComputingPoolDestroyedEvent as PoolDestroyedEvent,
    OffchainComputingPoolMetadataUpdatedEvent as PoolMetadataUpdatedEvent,
    OffchainComputingPoolMetadataRemovedEvent as PoolMetadataRemovedEvent,
    OffchainComputingPoolCreatingTaskAbilityEnabledEvent as PoolCreatingTaskAbilityEnabledEvent,
    OffchainComputingPoolCreatingTaskAbilityDisabledEvent as PoolCreatingTaskAbilityDisabledEvent,
    OffchainComputingWorkerAddedEvent as WorkerAddedEvent,
    OffchainComputingWorkerRemovedEvent as WorkerRemovedEvent,
} from "../types/events"

// interface PoolChanges {
//     readonly id: string
//     owner?: string
//     status?: WorkerStatus
//     implName?: string
//     implVersion?: number
//     attestationMethod?: AttestationMethod
//     lastAttestedAt?: Date
//     lastHeartbeatReceivedAt?: Date
//     offlineAt?: Date
//     offlineReason?: OfflineReason
//
//     deregistered: boolean
//     lastUpdatedBlockNumber: number
// }
