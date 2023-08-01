import assert from 'assert'
import {Runtime, ChainContext, Event, Result, Option} from './support'
import * as v100 from './v100'

export class OffchainComputingInfraImplBuildDeregisteredEvent {
    constructor(private readonly event: Event) {
        assert(this.event.name === 'OffchainComputingInfra.ImplBuildDeregistered')
    }

    /**
     * Remove worker's implementation permission successfully
     */
    get isV100(): boolean {
        return this.event.block._runtime.getEventTypeHash('OffchainComputingInfra.ImplBuildDeregistered') === 'ee1f58f7a32dedbbf0581d70188f18897f3ef35cb81eb50560663c7e0006d515'
    }

    /**
     * Remove worker's implementation permission successfully
     */
    get asV100(): {implId: number, implBuildVersion: number} {
        assert(this.isV100)
        return this.event.block._runtime.decodeJsonEvent(this.event)
    }
}

export class OffchainComputingInfraImplBuildRegisteredEvent {
    constructor(private readonly event: Event) {
        assert(this.event.name === 'OffchainComputingInfra.ImplBuildRegistered')
    }

    /**
     * Update worker's implementation permission successfully
     */
    get isV100(): boolean {
        return this.event.block._runtime.getEventTypeHash('OffchainComputingInfra.ImplBuildRegistered') === 'e702a7c67fd43ee1a9344f6f419dbb8f5211c58a4fc37aa70488d08acdf695e5'
    }

    /**
     * Update worker's implementation permission successfully
     */
    get asV100(): {implId: number, implBuildVersion: number, magicBytes: (Uint8Array | undefined)} {
        assert(this.isV100)
        return this.event.block._runtime.decodeJsonEvent(this.event)
    }
}

export class OffchainComputingInfraImplBuildStatusUpdatedEvent {
    constructor(private readonly event: Event) {
        assert(this.event.name === 'OffchainComputingInfra.ImplBuildStatusUpdated')
    }

    get isV100(): boolean {
        return this.event.block._runtime.getEventTypeHash('OffchainComputingInfra.ImplBuildStatusUpdated') === '991c4e673e2b47723a8d672e05c9f8cf23cbfa1b63ac37797d0ff43de6877337'
    }

    get asV100(): {implId: number, implBuildVersion: number, status: v100.ImplBuildStatus} {
        assert(this.isV100)
        return this.event.block._runtime.decodeJsonEvent(this.event)
    }
}

export class OffchainComputingInfraImplDeploymentScopeUpdatedEvent {
    constructor(private readonly event: Event) {
        assert(this.event.name === 'OffchainComputingInfra.ImplDeploymentScopeUpdated')
    }

    get isV100(): boolean {
        return this.event.block._runtime.getEventTypeHash('OffchainComputingInfra.ImplDeploymentScopeUpdated') === 'a8d51f484c5707408d8dd5e46ac0c7a8782f1ee389019a58156124a38fb90d8f'
    }

    get asV100(): {implId: number, scope: v100.ApplicableScope} {
        assert(this.isV100)
        return this.event.block._runtime.decodeJsonEvent(this.event)
    }
}

export class OffchainComputingInfraImplDeregisteredEvent {
    constructor(private readonly event: Event) {
        assert(this.event.name === 'OffchainComputingInfra.ImplDeregistered')
    }

    get isV100(): boolean {
        return this.event.block._runtime.getEventTypeHash('OffchainComputingInfra.ImplDeregistered') === '243c45b1eb352aca71bba131e47d5eeda77b36576c7265093d0ba2a43295cdf0'
    }

    get asV100(): {implId: number} {
        assert(this.isV100)
        return this.event.block._runtime.decodeJsonEvent(this.event)
    }
}

export class OffchainComputingInfraImplMetadataRemovedEvent {
    constructor(private readonly event: Event) {
        assert(this.event.name === 'OffchainComputingInfra.ImplMetadataRemoved')
    }

    get isV100(): boolean {
        return this.event.block._runtime.getEventTypeHash('OffchainComputingInfra.ImplMetadataRemoved') === '243c45b1eb352aca71bba131e47d5eeda77b36576c7265093d0ba2a43295cdf0'
    }

    get asV100(): {implId: number} {
        assert(this.isV100)
        return this.event.block._runtime.decodeJsonEvent(this.event)
    }
}

export class OffchainComputingInfraImplMetadataUpdatedEvent {
    constructor(private readonly event: Event) {
        assert(this.event.name === 'OffchainComputingInfra.ImplMetadataUpdated')
    }

    get isV100(): boolean {
        return this.event.block._runtime.getEventTypeHash('OffchainComputingInfra.ImplMetadataUpdated') === 'a8b2895eef238f3190b37e1c01543c35b24426912864c569b1f90d9a44dada71'
    }

    get asV100(): {implId: number, metadata: Uint8Array} {
        assert(this.isV100)
        return this.event.block._runtime.decodeJsonEvent(this.event)
    }
}

export class OffchainComputingInfraImplRegisteredEvent {
    constructor(private readonly event: Event) {
        assert(this.event.name === 'OffchainComputingInfra.ImplRegistered')
    }

    get isV100(): boolean {
        return this.event.block._runtime.getEventTypeHash('OffchainComputingInfra.ImplRegistered') === '75000c65c0d1eecfcc869ed0d53b023ed8b7970875387cd7ce5ca4ece26d9bd3'
    }

    get asV100(): {implId: number, owner: Uint8Array, attestationMethod: v100.AttestationMethod, deploymentScope: v100.ApplicableScope} {
        assert(this.isV100)
        return this.event.block._runtime.decodeJsonEvent(this.event)
    }
}

export class OffchainComputingInfraWorkerAttestationRefreshedEvent {
    constructor(private readonly event: Event) {
        assert(this.event.name === 'OffchainComputingInfra.WorkerAttestationRefreshed')
    }

    /**
     * The worker refresh its attestation successfully
     */
    get isV100(): boolean {
        return this.event.block._runtime.getEventTypeHash('OffchainComputingInfra.WorkerAttestationRefreshed') === '11311569c0b5d8d9a141443d6585b6f61dbfbe570c7b44dbb38dec80cbd91856'
    }

    /**
     * The worker refresh its attestation successfully
     */
    get asV100(): {worker: Uint8Array, expiresAt: (bigint | undefined)} {
        assert(this.isV100)
        return this.event.block._runtime.decodeJsonEvent(this.event)
    }
}

export class OffchainComputingInfraWorkerDeregisteredEvent {
    constructor(private readonly event: Event) {
        assert(this.event.name === 'OffchainComputingInfra.WorkerDeregistered')
    }

    /**
     * The worker registered successfully
     */
    get isV100(): boolean {
        return this.event.block._runtime.getEventTypeHash('OffchainComputingInfra.WorkerDeregistered') === '89afc0a50a4c6c7a30b8f0f59fabb0595938157214c865a0e46fcb20cd60fef4'
    }

    /**
     * The worker registered successfully
     */
    get asV100(): {worker: Uint8Array, force: boolean} {
        assert(this.isV100)
        return this.event.block._runtime.decodeJsonEvent(this.event)
    }
}

export class OffchainComputingInfraWorkerHeartbeatReceivedEvent {
    constructor(private readonly event: Event) {
        assert(this.event.name === 'OffchainComputingInfra.WorkerHeartbeatReceived')
    }

    /**
     * The worker send heartbeat successfully
     */
    get isV100(): boolean {
        return this.event.block._runtime.getEventTypeHash('OffchainComputingInfra.WorkerHeartbeatReceived') === '9821db54279b938f80c0331e3b1a0e59c27d3a5404e3ff37153b9e0337713e96'
    }

    /**
     * The worker send heartbeat successfully
     */
    get asV100(): {worker: Uint8Array, next: number, uptime: bigint} {
        assert(this.isV100)
        return this.event.block._runtime.decodeJsonEvent(this.event)
    }
}

export class OffchainComputingInfraWorkerOfflineEvent {
    constructor(private readonly event: Event) {
        assert(this.event.name === 'OffchainComputingInfra.WorkerOffline')
    }

    /**
     * The worker is offline
     */
    get isV100(): boolean {
        return this.event.block._runtime.getEventTypeHash('OffchainComputingInfra.WorkerOffline') === '706d1637d5f1fb899e7a3e835222f70500beae35e8007feefe5671bde57b1b0e'
    }

    /**
     * The worker is offline
     */
    get asV100(): {worker: Uint8Array, reason: v100.OfflineReason} {
        assert(this.isV100)
        return this.event.block._runtime.decodeJsonEvent(this.event)
    }
}

export class OffchainComputingInfraWorkerOnlineEvent {
    constructor(private readonly event: Event) {
        assert(this.event.name === 'OffchainComputingInfra.WorkerOnline')
    }

    /**
     * The worker is online
     */
    get isV100(): boolean {
        return this.event.block._runtime.getEventTypeHash('OffchainComputingInfra.WorkerOnline') === 'aaf3d9ba5d2b758f255a0585bcf74f1a8760c5debb4946ed2bf74fdc7a0262f3'
    }

    /**
     * The worker is online
     */
    get asV100(): {worker: Uint8Array, implSpecVersion: number, implBuildVersion: number, attestationMethod: v100.AttestationMethod, attestationExpiresAt: (bigint | undefined), nextHeartbeat: number} {
        assert(this.isV100)
        return this.event.block._runtime.decodeJsonEvent(this.event)
    }
}

export class OffchainComputingInfraWorkerRegisteredEvent {
    constructor(private readonly event: Event) {
        assert(this.event.name === 'OffchainComputingInfra.WorkerRegistered')
    }

    /**
     * The worker registered successfully
     */
    get isV100(): boolean {
        return this.event.block._runtime.getEventTypeHash('OffchainComputingInfra.WorkerRegistered') === '827feabbaece11c6815c2990c6cee79b02f149ad9405967a3410ef1fe8258576'
    }

    /**
     * The worker registered successfully
     */
    get asV100(): {worker: Uint8Array, owner: Uint8Array, implId: number} {
        assert(this.isV100)
        return this.event.block._runtime.decodeJsonEvent(this.event)
    }
}

export class OffchainComputingInfraWorkerRequestingOfflineEvent {
    constructor(private readonly event: Event) {
        assert(this.event.name === 'OffchainComputingInfra.WorkerRequestingOffline')
    }

    /**
     * The worker is requesting offline
     */
    get isV100(): boolean {
        return this.event.block._runtime.getEventTypeHash('OffchainComputingInfra.WorkerRequestingOffline') === '92a756e29fcd5c189000de949169e0c1ab6089e641fedb90b510a59a4b108e28'
    }

    /**
     * The worker is requesting offline
     */
    get asV100(): {worker: Uint8Array} {
        assert(this.isV100)
        return this.event.block._runtime.decodeJsonEvent(this.event)
    }
}

export class OffchainComputingInfraWorkerUnresponsiveEvent {
    constructor(private readonly event: Event) {
        assert(this.event.name === 'OffchainComputingInfra.WorkerUnresponsive')
    }

    get isV100(): boolean {
        return this.event.block._runtime.getEventTypeHash('OffchainComputingInfra.WorkerUnresponsive') === '92a756e29fcd5c189000de949169e0c1ab6089e641fedb90b510a59a4b108e28'
    }

    get asV100(): {worker: Uint8Array} {
        assert(this.isV100)
        return this.event.block._runtime.decodeJsonEvent(this.event)
    }
}

export class OffchainComputingPoolJobAssignedEvent {
    constructor(private readonly event: Event) {
        assert(this.event.name === 'OffchainComputingPool.JobAssigned')
    }

    get isV100(): boolean {
        return this.event.block._runtime.getEventTypeHash('OffchainComputingPool.JobAssigned') === 'afa6ec0d6dcdc2fa03b5f4d02a08bfd3adf490976f1f5080cf7d783c88c59315'
    }

    get asV100(): {poolId: number, jobId: number, assignee: Uint8Array} {
        assert(this.isV100)
        return this.event.block._runtime.decodeJsonEvent(this.event)
    }
}

export class OffchainComputingPoolJobCreatedEvent {
    constructor(private readonly event: Event) {
        assert(this.event.name === 'OffchainComputingPool.JobCreated')
    }

    get isV100(): boolean {
        return this.event.block._runtime.getEventTypeHash('OffchainComputingPool.JobCreated') === '2ca1a72fb554128c8618ed4dea31ea14b5f07bbc5b7ca888ed8a5bf0244f67cd'
    }

    get asV100(): {poolId: number, jobId: number, uniqueTrackId: (number | undefined), policyId: number, depositor: Uint8Array, beneficiary: Uint8Array, implSpecVersion: number, input: (Uint8Array | undefined), expiresIn: bigint} {
        assert(this.isV100)
        return this.event.block._runtime.decodeJsonEvent(this.event)
    }
}

export class OffchainComputingPoolJobDestroyedEvent {
    constructor(private readonly event: Event) {
        assert(this.event.name === 'OffchainComputingPool.JobDestroyed')
    }

    get isV100(): boolean {
        return this.event.block._runtime.getEventTypeHash('OffchainComputingPool.JobDestroyed') === '97c1f6f2901e643a2159e7f69a3c2fdda583ca933f60779b0e603156dd5296df'
    }

    get asV100(): {poolId: number, jobId: number, uniqueTrackId: (number | undefined), destroyer: Uint8Array, force: boolean} {
        assert(this.isV100)
        return this.event.block._runtime.decodeJsonEvent(this.event)
    }
}

export class OffchainComputingPoolJobPolicyCreatedEvent {
    constructor(private readonly event: Event) {
        assert(this.event.name === 'OffchainComputingPool.JobPolicyCreated')
    }

    get isV100(): boolean {
        return this.event.block._runtime.getEventTypeHash('OffchainComputingPool.JobPolicyCreated') === '2fdcfa465bf69e7c0acd0c667d4e33944626eb86e207ccae5af65ccf1e69dbf8'
    }

    get asV100(): {poolId: number, policyId: number, applicableScope: v100.ApplicableScope, startBlock: (number | undefined), endBlock: (number | undefined)} {
        assert(this.isV100)
        return this.event.block._runtime.decodeJsonEvent(this.event)
    }
}

export class OffchainComputingPoolJobPolicyDestroyedEvent {
    constructor(private readonly event: Event) {
        assert(this.event.name === 'OffchainComputingPool.JobPolicyDestroyed')
    }

    get isV100(): boolean {
        return this.event.block._runtime.getEventTypeHash('OffchainComputingPool.JobPolicyDestroyed') === '0a745627569b6e48894cab5a1744a734efdb1d3dfb30516a25ee14ebc11efd29'
    }

    get asV100(): {poolId: number, policyId: number} {
        assert(this.isV100)
        return this.event.block._runtime.decodeJsonEvent(this.event)
    }
}

export class OffchainComputingPoolJobPolicyEnablementUpdatedEvent {
    constructor(private readonly event: Event) {
        assert(this.event.name === 'OffchainComputingPool.JobPolicyEnablementUpdated')
    }

    get isV100(): boolean {
        return this.event.block._runtime.getEventTypeHash('OffchainComputingPool.JobPolicyEnablementUpdated') === '14b8d3cdf7df15a6e7d0ac211564ea5a4c966b86fcdd995eac3a987c41ca6146'
    }

    get asV100(): {poolId: number, policyId: number, enabled: boolean} {
        assert(this.isV100)
        return this.event.block._runtime.decodeJsonEvent(this.event)
    }
}

export class OffchainComputingPoolJobReleasedEvent {
    constructor(private readonly event: Event) {
        assert(this.event.name === 'OffchainComputingPool.JobReleased')
    }

    get isV100(): boolean {
        return this.event.block._runtime.getEventTypeHash('OffchainComputingPool.JobReleased') === 'b0297d16a829746a9da69425d229c5371423f3c3590a167521b432486abddac5'
    }

    get asV100(): {poolId: number, jobId: number} {
        assert(this.isV100)
        return this.event.block._runtime.decodeJsonEvent(this.event)
    }
}

export class OffchainComputingPoolJobResultUpdatedEvent {
    constructor(private readonly event: Event) {
        assert(this.event.name === 'OffchainComputingPool.JobResultUpdated')
    }

    get isV100(): boolean {
        return this.event.block._runtime.getEventTypeHash('OffchainComputingPool.JobResultUpdated') === '46bfc8318d1eec0722fd53179348b372fa85cd942c03333d6014048410f7f735'
    }

    get asV100(): {poolId: number, jobId: number, result: v100.JobResult, output: (Uint8Array | undefined), proof: (Uint8Array | undefined)} {
        assert(this.isV100)
        return this.event.block._runtime.decodeJsonEvent(this.event)
    }
}

export class OffchainComputingPoolJobStatusUpdatedEvent {
    constructor(private readonly event: Event) {
        assert(this.event.name === 'OffchainComputingPool.JobStatusUpdated')
    }

    get isV100(): boolean {
        return this.event.block._runtime.getEventTypeHash('OffchainComputingPool.JobStatusUpdated') === '87918cd4ffa1fe11c78f43852ab3c669c246a43be50e45bc4d580fc6543fb39a'
    }

    get asV100(): {poolId: number, jobId: number, status: v100.JobStatus} {
        assert(this.isV100)
        return this.event.block._runtime.decodeJsonEvent(this.event)
    }
}

export class OffchainComputingPoolPoolCreatedEvent {
    constructor(private readonly event: Event) {
        assert(this.event.name === 'OffchainComputingPool.PoolCreated')
    }

    get isV100(): boolean {
        return this.event.block._runtime.getEventTypeHash('OffchainComputingPool.PoolCreated') === '9ee7b2b124899a27c7c1adb59ceedd04ea0cb81f092b946f4bc808ebc17333be'
    }

    get asV100(): {owner: Uint8Array, poolId: number, implId: number, createJobEnabled: boolean, autoDestroyProcessedJobEnabled: boolean} {
        assert(this.isV100)
        return this.event.block._runtime.decodeJsonEvent(this.event)
    }
}

export class OffchainComputingPoolPoolDestroyedEvent {
    constructor(private readonly event: Event) {
        assert(this.event.name === 'OffchainComputingPool.PoolDestroyed')
    }

    get isV100(): boolean {
        return this.event.block._runtime.getEventTypeHash('OffchainComputingPool.PoolDestroyed') === 'a662258b1bdb045a915972ea29e9ec0b46cdd5598b0da37b0e70ac766e3735a0'
    }

    get asV100(): {poolId: number} {
        assert(this.isV100)
        return this.event.block._runtime.decodeJsonEvent(this.event)
    }
}

export class OffchainComputingPoolPoolMetadataRemovedEvent {
    constructor(private readonly event: Event) {
        assert(this.event.name === 'OffchainComputingPool.PoolMetadataRemoved')
    }

    get isV100(): boolean {
        return this.event.block._runtime.getEventTypeHash('OffchainComputingPool.PoolMetadataRemoved') === 'a662258b1bdb045a915972ea29e9ec0b46cdd5598b0da37b0e70ac766e3735a0'
    }

    get asV100(): {poolId: number} {
        assert(this.isV100)
        return this.event.block._runtime.decodeJsonEvent(this.event)
    }
}

export class OffchainComputingPoolPoolMetadataUpdatedEvent {
    constructor(private readonly event: Event) {
        assert(this.event.name === 'OffchainComputingPool.PoolMetadataUpdated')
    }

    get isV100(): boolean {
        return this.event.block._runtime.getEventTypeHash('OffchainComputingPool.PoolMetadataUpdated') === '027a45601396dd984f964705eb7eabc42207aedba700c7316c5f0201a21bc953'
    }

    get asV100(): {poolId: number, metadata: Uint8Array} {
        assert(this.isV100)
        return this.event.block._runtime.decodeJsonEvent(this.event)
    }
}

export class OffchainComputingPoolPoolSettingsUpdatedEvent {
    constructor(private readonly event: Event) {
        assert(this.event.name === 'OffchainComputingPool.PoolSettingsUpdated')
    }

    get isV100(): boolean {
        return this.event.block._runtime.getEventTypeHash('OffchainComputingPool.PoolSettingsUpdated') === '41628b04b807369b6a8a2beaedb67a8dcdb05c46a6285c696f61045ca76c7343'
    }

    get asV100(): {poolId: number, minImplSpecVersion: number, maxImplSpecVersion: number, createJobEnabled: boolean, autoDestroyProcessedJobEnabled: boolean} {
        assert(this.isV100)
        return this.event.block._runtime.decodeJsonEvent(this.event)
    }
}

export class OffchainComputingPoolWorkerProvisionedEvent {
    constructor(private readonly event: Event) {
        assert(this.event.name === 'OffchainComputingPool.WorkerProvisioned')
    }

    get isV100(): boolean {
        return this.event.block._runtime.getEventTypeHash('OffchainComputingPool.WorkerProvisioned') === '797c331fcd879fe634a85213860a97ecd0575adecd2689f258b95e6c04d3cdf3'
    }

    get asV100(): {poolId: number, worker: Uint8Array} {
        assert(this.isV100)
        return this.event.block._runtime.decodeJsonEvent(this.event)
    }
}

export class OffchainComputingPoolWorkerRevokedEvent {
    constructor(private readonly event: Event) {
        assert(this.event.name === 'OffchainComputingPool.WorkerRevoked')
    }

    get isV100(): boolean {
        return this.event.block._runtime.getEventTypeHash('OffchainComputingPool.WorkerRevoked') === '797c331fcd879fe634a85213860a97ecd0575adecd2689f258b95e6c04d3cdf3'
    }

    get asV100(): {poolId: number, worker: Uint8Array} {
        assert(this.isV100)
        return this.event.block._runtime.decodeJsonEvent(this.event)
    }
}

export class OffchainComputingPoolWorkerSubscribedEvent {
    constructor(private readonly event: Event) {
        assert(this.event.name === 'OffchainComputingPool.WorkerSubscribed')
    }

    get isV100(): boolean {
        return this.event.block._runtime.getEventTypeHash('OffchainComputingPool.WorkerSubscribed') === '2bf1f63dd355996120e05c07b3507c93d7b4611ab4df91424d8bc633e8293fed'
    }

    get asV100(): {worker: Uint8Array, poolId: number} {
        assert(this.isV100)
        return this.event.block._runtime.decodeJsonEvent(this.event)
    }
}

export class OffchainComputingPoolWorkerUnsubscribedEvent {
    constructor(private readonly event: Event) {
        assert(this.event.name === 'OffchainComputingPool.WorkerUnsubscribed')
    }

    get isV100(): boolean {
        return this.event.block._runtime.getEventTypeHash('OffchainComputingPool.WorkerUnsubscribed') === '2bf1f63dd355996120e05c07b3507c93d7b4611ab4df91424d8bc633e8293fed'
    }

    get asV100(): {worker: Uint8Array, poolId: number} {
        assert(this.isV100)
        return this.event.block._runtime.decodeJsonEvent(this.event)
    }
}
