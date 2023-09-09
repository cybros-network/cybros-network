import {EventType, sts} from './support'
import * as v100 from './v100'

/**
 * Remove worker's implementation permission successfully
 */
export const OffchainComputingInfraImplBuildDeregisteredEventV100 = new EventType(
    sts.struct({
        implId: sts.number(),
        implBuildVersion: sts.number(),
    })
)

/**
 * Update worker's implementation permission successfully
 */
export const OffchainComputingInfraImplBuildRegisteredEventV100 = new EventType(
    sts.struct({
        implId: sts.number(),
        implBuildVersion: sts.number(),
        magicBytes: sts.option(() => sts.bytes()),
    })
)

export const OffchainComputingInfraImplBuildStatusUpdatedEventV100 = new EventType(
    sts.struct({
        implId: sts.number(),
        implBuildVersion: sts.number(),
        status: v100.ImplBuildStatus,
    })
)

export const OffchainComputingInfraImplDeploymentScopeUpdatedEventV100 = new EventType(
    sts.struct({
        implId: sts.number(),
        scope: v100.ApplicableScope,
    })
)

export const OffchainComputingInfraImplDeregisteredEventV100 = new EventType(
    sts.struct({
        implId: sts.number(),
    })
)

export const OffchainComputingInfraImplMetadataRemovedEventV100 = new EventType(
    sts.struct({
        implId: sts.number(),
    })
)

export const OffchainComputingInfraImplMetadataUpdatedEventV100 = new EventType(
    sts.struct({
        implId: sts.number(),
        metadata: sts.bytes(),
    })
)

export const OffchainComputingInfraImplRegisteredEventV100 = new EventType(
    sts.struct({
        implId: sts.number(),
        owner: sts.bytes(),
        attestationMethod: v100.AttestationMethod,
        deploymentScope: v100.ApplicableScope,
    })
)

/**
 * The worker refresh its attestation successfully
 */
export const OffchainComputingInfraWorkerAttestationRefreshedEventV100 = new EventType(
    sts.struct({
        worker: sts.bytes(),
        expiresAt: sts.option(() => sts.bigint()),
    })
)

/**
 * The worker registered successfully
 */
export const OffchainComputingInfraWorkerDeregisteredEventV100 = new EventType(
    sts.struct({
        worker: sts.bytes(),
        force: sts.boolean(),
    })
)

/**
 * The worker send heartbeat successfully
 */
export const OffchainComputingInfraWorkerHeartbeatReceivedEventV100 = new EventType(
    sts.struct({
        worker: sts.bytes(),
        next: sts.number(),
        uptime: sts.bigint(),
    })
)

/**
 * The worker is offline
 */
export const OffchainComputingInfraWorkerOfflineEventV100 = new EventType(
    sts.struct({
        worker: sts.bytes(),
        reason: v100.OfflineReason,
    })
)

/**
 * The worker is online
 */
export const OffchainComputingInfraWorkerOnlineEventV100 = new EventType(
    sts.struct({
        worker: sts.bytes(),
        implSpecVersion: sts.number(),
        implBuildVersion: sts.number(),
        attestationMethod: v100.AttestationMethod,
        attestationExpiresAt: sts.option(() => sts.bigint()),
        nextHeartbeat: sts.number(),
    })
)

/**
 * The worker registered successfully
 */
export const OffchainComputingInfraWorkerRegisteredEventV100 = new EventType(
    sts.struct({
        worker: sts.bytes(),
        owner: sts.bytes(),
        implId: sts.number(),
    })
)

/**
 * The worker is requesting offline
 */
export const OffchainComputingInfraWorkerRequestingOfflineEventV100 = new EventType(
    sts.struct({
        worker: sts.bytes(),
    })
)

export const OffchainComputingInfraWorkerUnresponsiveEventV100 = new EventType(
    sts.struct({
        worker: sts.bytes(),
    })
)

export const OffchainComputingPoolJobAssignedEventV100 = new EventType(
    sts.struct({
        poolId: sts.number(),
        jobId: sts.number(),
        assignee: sts.bytes(),
    })
)

export const OffchainComputingPoolJobCreatedEventV100 = new EventType(
    sts.struct({
        poolId: sts.number(),
        jobId: sts.number(),
        uniqueTrackId: sts.option(() => sts.number()),
        policyId: sts.number(),
        depositor: sts.bytes(),
        beneficiary: sts.bytes(),
        implSpecVersion: sts.number(),
        input: sts.option(() => sts.bytes()),
        expiresIn: sts.bigint(),
    })
)

export const OffchainComputingPoolJobDestroyedEventV100 = new EventType(
    sts.struct({
        poolId: sts.number(),
        jobId: sts.number(),
        uniqueTrackId: sts.option(() => sts.number()),
        destroyer: sts.bytes(),
        force: sts.boolean(),
    })
)

export const OffchainComputingPoolJobPolicyCreatedEventV100 = new EventType(
    sts.struct({
        poolId: sts.number(),
        policyId: sts.number(),
        applicableScope: v100.ApplicableScope,
        startBlock: sts.option(() => sts.number()),
        endBlock: sts.option(() => sts.number()),
    })
)

export const OffchainComputingPoolJobPolicyDestroyedEventV100 = new EventType(
    sts.struct({
        poolId: sts.number(),
        policyId: sts.number(),
    })
)

export const OffchainComputingPoolJobPolicyEnablementUpdatedEventV100 = new EventType(
    sts.struct({
        poolId: sts.number(),
        policyId: sts.number(),
        enabled: sts.boolean(),
    })
)

export const OffchainComputingPoolJobReleasedEventV100 = new EventType(
    sts.struct({
        poolId: sts.number(),
        jobId: sts.number(),
    })
)

export const OffchainComputingPoolJobResultUpdatedEventV100 = new EventType(
    sts.struct({
        poolId: sts.number(),
        jobId: sts.number(),
        result: v100.JobResult,
        output: sts.option(() => sts.bytes()),
        proof: sts.option(() => sts.bytes()),
    })
)

export const OffchainComputingPoolJobStatusUpdatedEventV100 = new EventType(
    sts.struct({
        poolId: sts.number(),
        jobId: sts.number(),
        status: v100.JobStatus,
    })
)

export const OffchainComputingPoolPoolCreatedEventV100 = new EventType(
    sts.struct({
        owner: sts.bytes(),
        poolId: sts.number(),
        implId: sts.number(),
        createJobEnabled: sts.boolean(),
        autoDestroyProcessedJobEnabled: sts.boolean(),
    })
)

export const OffchainComputingPoolPoolDestroyedEventV100 = new EventType(
    sts.struct({
        poolId: sts.number(),
    })
)

export const OffchainComputingPoolPoolMetadataRemovedEventV100 = new EventType(
    sts.struct({
        poolId: sts.number(),
    })
)

export const OffchainComputingPoolPoolMetadataUpdatedEventV100 = new EventType(
    sts.struct({
        poolId: sts.number(),
        metadata: sts.bytes(),
    })
)

export const OffchainComputingPoolPoolSettingsUpdatedEventV100 = new EventType(
    sts.struct({
        poolId: sts.number(),
        minImplSpecVersion: sts.number(),
        maxImplSpecVersion: sts.number(),
        createJobEnabled: sts.boolean(),
        autoDestroyProcessedJobEnabled: sts.boolean(),
    })
)

export const OffchainComputingPoolWorkerProvisionedEventV100 = new EventType(
    sts.struct({
        poolId: sts.number(),
        worker: sts.bytes(),
    })
)

export const OffchainComputingPoolWorkerRevokedEventV100 = new EventType(
    sts.struct({
        poolId: sts.number(),
        worker: sts.bytes(),
    })
)

export const OffchainComputingPoolWorkerSubscribedEventV100 = new EventType(
    sts.struct({
        worker: sts.bytes(),
        poolId: sts.number(),
    })
)

export const OffchainComputingPoolWorkerUnsubscribedEventV100 = new EventType(
    sts.struct({
        worker: sts.bytes(),
        poolId: sts.number(),
    })
)
