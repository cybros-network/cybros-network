import assert from 'assert'
import {Runtime, ChainContext, EventContext, Event, Result, Option} from './support'
import * as v100 from './v100'

export class OffchainComputingJobAssignedEvent {
    private readonly runtime: Runtime
    private readonly event: Event

    constructor(ctx: EventContext)
    constructor(ctx: ChainContext, event: Event)
    constructor(ctx: EventContext, event?: Event) {
        this.runtime = ctx._chain.runtime
        this.event = event || ctx.event
        assert(this.event.name === 'OffchainComputing.JobAssigned')
    }

    get isV100(): boolean {
        return this.runtime.getEventTypeHash('OffchainComputing.JobAssigned') === 'afa6ec0d6dcdc2fa03b5f4d02a08bfd3adf490976f1f5080cf7d783c88c59315'
    }

    get asV100(): {poolId: number, jobId: number, assignee: Uint8Array} {
        assert(this.isV100)
        return this.runtime.decodeJsonEvent(this.event)
    }
}

export class OffchainComputingJobCreatedEvent {
    private readonly runtime: Runtime
    private readonly event: Event

    constructor(ctx: EventContext)
    constructor(ctx: ChainContext, event: Event)
    constructor(ctx: EventContext, event?: Event) {
        this.runtime = ctx._chain.runtime
        this.event = event || ctx.event
        assert(this.event.name === 'OffchainComputing.JobCreated')
    }

    get isV100(): boolean {
        return this.runtime.getEventTypeHash('OffchainComputing.JobCreated') === '2ca1a72fb554128c8618ed4dea31ea14b5f07bbc5b7ca888ed8a5bf0244f67cd'
    }

    get asV100(): {poolId: number, jobId: number, uniqueTrackId: (number | undefined), policyId: number, depositor: Uint8Array, beneficiary: Uint8Array, implSpecVersion: number, input: (Uint8Array | undefined), expiresIn: bigint} {
        assert(this.isV100)
        return this.runtime.decodeJsonEvent(this.event)
    }
}

export class OffchainComputingJobDestroyedEvent {
    private readonly runtime: Runtime
    private readonly event: Event

    constructor(ctx: EventContext)
    constructor(ctx: ChainContext, event: Event)
    constructor(ctx: EventContext, event?: Event) {
        this.runtime = ctx._chain.runtime
        this.event = event || ctx.event
        assert(this.event.name === 'OffchainComputing.JobDestroyed')
    }

    get isV100(): boolean {
        return this.runtime.getEventTypeHash('OffchainComputing.JobDestroyed') === '97c1f6f2901e643a2159e7f69a3c2fdda583ca933f60779b0e603156dd5296df'
    }

    get asV100(): {poolId: number, jobId: number, uniqueTrackId: (number | undefined), destroyer: Uint8Array, force: boolean} {
        assert(this.isV100)
        return this.runtime.decodeJsonEvent(this.event)
    }
}

export class OffchainComputingJobPolicyCreatedEvent {
    private readonly runtime: Runtime
    private readonly event: Event

    constructor(ctx: EventContext)
    constructor(ctx: ChainContext, event: Event)
    constructor(ctx: EventContext, event?: Event) {
        this.runtime = ctx._chain.runtime
        this.event = event || ctx.event
        assert(this.event.name === 'OffchainComputing.JobPolicyCreated')
    }

    get isV100(): boolean {
        return this.runtime.getEventTypeHash('OffchainComputing.JobPolicyCreated') === '2fdcfa465bf69e7c0acd0c667d4e33944626eb86e207ccae5af65ccf1e69dbf8'
    }

    get asV100(): {poolId: number, policyId: number, applicableScope: v100.ApplicableScope, startBlock: (number | undefined), endBlock: (number | undefined)} {
        assert(this.isV100)
        return this.runtime.decodeJsonEvent(this.event)
    }
}

export class OffchainComputingJobPolicyDestroyedEvent {
    private readonly runtime: Runtime
    private readonly event: Event

    constructor(ctx: EventContext)
    constructor(ctx: ChainContext, event: Event)
    constructor(ctx: EventContext, event?: Event) {
        this.runtime = ctx._chain.runtime
        this.event = event || ctx.event
        assert(this.event.name === 'OffchainComputing.JobPolicyDestroyed')
    }

    get isV100(): boolean {
        return this.runtime.getEventTypeHash('OffchainComputing.JobPolicyDestroyed') === '0a745627569b6e48894cab5a1744a734efdb1d3dfb30516a25ee14ebc11efd29'
    }

    get asV100(): {poolId: number, policyId: number} {
        assert(this.isV100)
        return this.runtime.decodeJsonEvent(this.event)
    }
}

export class OffchainComputingJobPolicyEnablementUpdatedEvent {
    private readonly runtime: Runtime
    private readonly event: Event

    constructor(ctx: EventContext)
    constructor(ctx: ChainContext, event: Event)
    constructor(ctx: EventContext, event?: Event) {
        this.runtime = ctx._chain.runtime
        this.event = event || ctx.event
        assert(this.event.name === 'OffchainComputing.JobPolicyEnablementUpdated')
    }

    get isV100(): boolean {
        return this.runtime.getEventTypeHash('OffchainComputing.JobPolicyEnablementUpdated') === '14b8d3cdf7df15a6e7d0ac211564ea5a4c966b86fcdd995eac3a987c41ca6146'
    }

    get asV100(): {poolId: number, policyId: number, enabled: boolean} {
        assert(this.isV100)
        return this.runtime.decodeJsonEvent(this.event)
    }
}

export class OffchainComputingJobReleasedEvent {
    private readonly runtime: Runtime
    private readonly event: Event

    constructor(ctx: EventContext)
    constructor(ctx: ChainContext, event: Event)
    constructor(ctx: EventContext, event?: Event) {
        this.runtime = ctx._chain.runtime
        this.event = event || ctx.event
        assert(this.event.name === 'OffchainComputing.JobReleased')
    }

    get isV100(): boolean {
        return this.runtime.getEventTypeHash('OffchainComputing.JobReleased') === 'b0297d16a829746a9da69425d229c5371423f3c3590a167521b432486abddac5'
    }

    get asV100(): {poolId: number, jobId: number} {
        assert(this.isV100)
        return this.runtime.decodeJsonEvent(this.event)
    }
}

export class OffchainComputingJobResultUpdatedEvent {
    private readonly runtime: Runtime
    private readonly event: Event

    constructor(ctx: EventContext)
    constructor(ctx: ChainContext, event: Event)
    constructor(ctx: EventContext, event?: Event) {
        this.runtime = ctx._chain.runtime
        this.event = event || ctx.event
        assert(this.event.name === 'OffchainComputing.JobResultUpdated')
    }

    get isV100(): boolean {
        return this.runtime.getEventTypeHash('OffchainComputing.JobResultUpdated') === '46bfc8318d1eec0722fd53179348b372fa85cd942c03333d6014048410f7f735'
    }

    get asV100(): {poolId: number, jobId: number, result: v100.JobResult, output: (Uint8Array | undefined), proof: (Uint8Array | undefined)} {
        assert(this.isV100)
        return this.runtime.decodeJsonEvent(this.event)
    }
}

export class OffchainComputingJobStatusUpdatedEvent {
    private readonly runtime: Runtime
    private readonly event: Event

    constructor(ctx: EventContext)
    constructor(ctx: ChainContext, event: Event)
    constructor(ctx: EventContext, event?: Event) {
        this.runtime = ctx._chain.runtime
        this.event = event || ctx.event
        assert(this.event.name === 'OffchainComputing.JobStatusUpdated')
    }

    get isV100(): boolean {
        return this.runtime.getEventTypeHash('OffchainComputing.JobStatusUpdated') === '87918cd4ffa1fe11c78f43852ab3c669c246a43be50e45bc4d580fc6543fb39a'
    }

    get asV100(): {poolId: number, jobId: number, status: v100.JobStatus} {
        assert(this.isV100)
        return this.runtime.decodeJsonEvent(this.event)
    }
}

export class OffchainComputingPoolCreatedEvent {
    private readonly runtime: Runtime
    private readonly event: Event

    constructor(ctx: EventContext)
    constructor(ctx: ChainContext, event: Event)
    constructor(ctx: EventContext, event?: Event) {
        this.runtime = ctx._chain.runtime
        this.event = event || ctx.event
        assert(this.event.name === 'OffchainComputing.PoolCreated')
    }

    get isV100(): boolean {
        return this.runtime.getEventTypeHash('OffchainComputing.PoolCreated') === '9ee7b2b124899a27c7c1adb59ceedd04ea0cb81f092b946f4bc808ebc17333be'
    }

    get asV100(): {owner: Uint8Array, poolId: number, implId: number, createJobEnabled: boolean, autoDestroyProcessedJobEnabled: boolean} {
        assert(this.isV100)
        return this.runtime.decodeJsonEvent(this.event)
    }
}

export class OffchainComputingPoolDestroyedEvent {
    private readonly runtime: Runtime
    private readonly event: Event

    constructor(ctx: EventContext)
    constructor(ctx: ChainContext, event: Event)
    constructor(ctx: EventContext, event?: Event) {
        this.runtime = ctx._chain.runtime
        this.event = event || ctx.event
        assert(this.event.name === 'OffchainComputing.PoolDestroyed')
    }

    get isV100(): boolean {
        return this.runtime.getEventTypeHash('OffchainComputing.PoolDestroyed') === 'a662258b1bdb045a915972ea29e9ec0b46cdd5598b0da37b0e70ac766e3735a0'
    }

    get asV100(): {poolId: number} {
        assert(this.isV100)
        return this.runtime.decodeJsonEvent(this.event)
    }
}

export class OffchainComputingPoolMetadataRemovedEvent {
    private readonly runtime: Runtime
    private readonly event: Event

    constructor(ctx: EventContext)
    constructor(ctx: ChainContext, event: Event)
    constructor(ctx: EventContext, event?: Event) {
        this.runtime = ctx._chain.runtime
        this.event = event || ctx.event
        assert(this.event.name === 'OffchainComputing.PoolMetadataRemoved')
    }

    get isV100(): boolean {
        return this.runtime.getEventTypeHash('OffchainComputing.PoolMetadataRemoved') === 'a662258b1bdb045a915972ea29e9ec0b46cdd5598b0da37b0e70ac766e3735a0'
    }

    get asV100(): {poolId: number} {
        assert(this.isV100)
        return this.runtime.decodeJsonEvent(this.event)
    }
}

export class OffchainComputingPoolMetadataUpdatedEvent {
    private readonly runtime: Runtime
    private readonly event: Event

    constructor(ctx: EventContext)
    constructor(ctx: ChainContext, event: Event)
    constructor(ctx: EventContext, event?: Event) {
        this.runtime = ctx._chain.runtime
        this.event = event || ctx.event
        assert(this.event.name === 'OffchainComputing.PoolMetadataUpdated')
    }

    get isV100(): boolean {
        return this.runtime.getEventTypeHash('OffchainComputing.PoolMetadataUpdated') === '027a45601396dd984f964705eb7eabc42207aedba700c7316c5f0201a21bc953'
    }

    get asV100(): {poolId: number, metadata: Uint8Array} {
        assert(this.isV100)
        return this.runtime.decodeJsonEvent(this.event)
    }
}

export class OffchainComputingPoolSettingsUpdatedEvent {
    private readonly runtime: Runtime
    private readonly event: Event

    constructor(ctx: EventContext)
    constructor(ctx: ChainContext, event: Event)
    constructor(ctx: EventContext, event?: Event) {
        this.runtime = ctx._chain.runtime
        this.event = event || ctx.event
        assert(this.event.name === 'OffchainComputing.PoolSettingsUpdated')
    }

    get isV100(): boolean {
        return this.runtime.getEventTypeHash('OffchainComputing.PoolSettingsUpdated') === '41628b04b807369b6a8a2beaedb67a8dcdb05c46a6285c696f61045ca76c7343'
    }

    get asV100(): {poolId: number, minImplSpecVersion: number, maxImplSpecVersion: number, createJobEnabled: boolean, autoDestroyProcessedJobEnabled: boolean} {
        assert(this.isV100)
        return this.runtime.decodeJsonEvent(this.event)
    }
}

export class OffchainComputingWorkerProvisionedEvent {
    private readonly runtime: Runtime
    private readonly event: Event

    constructor(ctx: EventContext)
    constructor(ctx: ChainContext, event: Event)
    constructor(ctx: EventContext, event?: Event) {
        this.runtime = ctx._chain.runtime
        this.event = event || ctx.event
        assert(this.event.name === 'OffchainComputing.WorkerProvisioned')
    }

    get isV100(): boolean {
        return this.runtime.getEventTypeHash('OffchainComputing.WorkerProvisioned') === '797c331fcd879fe634a85213860a97ecd0575adecd2689f258b95e6c04d3cdf3'
    }

    get asV100(): {poolId: number, worker: Uint8Array} {
        assert(this.isV100)
        return this.runtime.decodeJsonEvent(this.event)
    }
}

export class OffchainComputingWorkerRevokedEvent {
    private readonly runtime: Runtime
    private readonly event: Event

    constructor(ctx: EventContext)
    constructor(ctx: ChainContext, event: Event)
    constructor(ctx: EventContext, event?: Event) {
        this.runtime = ctx._chain.runtime
        this.event = event || ctx.event
        assert(this.event.name === 'OffchainComputing.WorkerRevoked')
    }

    get isV100(): boolean {
        return this.runtime.getEventTypeHash('OffchainComputing.WorkerRevoked') === '797c331fcd879fe634a85213860a97ecd0575adecd2689f258b95e6c04d3cdf3'
    }

    get asV100(): {poolId: number, worker: Uint8Array} {
        assert(this.isV100)
        return this.runtime.decodeJsonEvent(this.event)
    }
}

export class OffchainComputingWorkerSubscribedEvent {
    private readonly runtime: Runtime
    private readonly event: Event

    constructor(ctx: EventContext)
    constructor(ctx: ChainContext, event: Event)
    constructor(ctx: EventContext, event?: Event) {
        this.runtime = ctx._chain.runtime
        this.event = event || ctx.event
        assert(this.event.name === 'OffchainComputing.WorkerSubscribed')
    }

    get isV100(): boolean {
        return this.runtime.getEventTypeHash('OffchainComputing.WorkerSubscribed') === '2bf1f63dd355996120e05c07b3507c93d7b4611ab4df91424d8bc633e8293fed'
    }

    get asV100(): {worker: Uint8Array, poolId: number} {
        assert(this.isV100)
        return this.runtime.decodeJsonEvent(this.event)
    }
}

export class OffchainComputingWorkerUnsubscribedEvent {
    private readonly runtime: Runtime
    private readonly event: Event

    constructor(ctx: EventContext)
    constructor(ctx: ChainContext, event: Event)
    constructor(ctx: EventContext, event?: Event) {
        this.runtime = ctx._chain.runtime
        this.event = event || ctx.event
        assert(this.event.name === 'OffchainComputing.WorkerUnsubscribed')
    }

    get isV100(): boolean {
        return this.runtime.getEventTypeHash('OffchainComputing.WorkerUnsubscribed') === '2bf1f63dd355996120e05c07b3507c93d7b4611ab4df91424d8bc633e8293fed'
    }

    get asV100(): {worker: Uint8Array, poolId: number} {
        assert(this.isV100)
        return this.runtime.decodeJsonEvent(this.event)
    }
}

export class OffchainComputingWorkersImplBuildDeregisteredEvent {
    private readonly runtime: Runtime
    private readonly event: Event

    constructor(ctx: EventContext)
    constructor(ctx: ChainContext, event: Event)
    constructor(ctx: EventContext, event?: Event) {
        this.runtime = ctx._chain.runtime
        this.event = event || ctx.event
        assert(this.event.name === 'OffchainComputingWorkers.ImplBuildDeregistered')
    }

    /**
     * Remove worker's implementation permission successfully
     */
    get isV100(): boolean {
        return this.runtime.getEventTypeHash('OffchainComputingWorkers.ImplBuildDeregistered') === 'ee1f58f7a32dedbbf0581d70188f18897f3ef35cb81eb50560663c7e0006d515'
    }

    /**
     * Remove worker's implementation permission successfully
     */
    get asV100(): {implId: number, implBuildVersion: number} {
        assert(this.isV100)
        return this.runtime.decodeJsonEvent(this.event)
    }
}

export class OffchainComputingWorkersImplBuildRegisteredEvent {
    private readonly runtime: Runtime
    private readonly event: Event

    constructor(ctx: EventContext)
    constructor(ctx: ChainContext, event: Event)
    constructor(ctx: EventContext, event?: Event) {
        this.runtime = ctx._chain.runtime
        this.event = event || ctx.event
        assert(this.event.name === 'OffchainComputingWorkers.ImplBuildRegistered')
    }

    /**
     * Update worker's implementation permission successfully
     */
    get isV100(): boolean {
        return this.runtime.getEventTypeHash('OffchainComputingWorkers.ImplBuildRegistered') === 'e702a7c67fd43ee1a9344f6f419dbb8f5211c58a4fc37aa70488d08acdf695e5'
    }

    /**
     * Update worker's implementation permission successfully
     */
    get asV100(): {implId: number, implBuildVersion: number, magicBytes: (Uint8Array | undefined)} {
        assert(this.isV100)
        return this.runtime.decodeJsonEvent(this.event)
    }
}

export class OffchainComputingWorkersImplBuildStatusUpdatedEvent {
    private readonly runtime: Runtime
    private readonly event: Event

    constructor(ctx: EventContext)
    constructor(ctx: ChainContext, event: Event)
    constructor(ctx: EventContext, event?: Event) {
        this.runtime = ctx._chain.runtime
        this.event = event || ctx.event
        assert(this.event.name === 'OffchainComputingWorkers.ImplBuildStatusUpdated')
    }

    get isV100(): boolean {
        return this.runtime.getEventTypeHash('OffchainComputingWorkers.ImplBuildStatusUpdated') === '991c4e673e2b47723a8d672e05c9f8cf23cbfa1b63ac37797d0ff43de6877337'
    }

    get asV100(): {implId: number, implBuildVersion: number, status: v100.ImplBuildStatus} {
        assert(this.isV100)
        return this.runtime.decodeJsonEvent(this.event)
    }
}

export class OffchainComputingWorkersImplDeploymentScopeUpdatedEvent {
    private readonly runtime: Runtime
    private readonly event: Event

    constructor(ctx: EventContext)
    constructor(ctx: ChainContext, event: Event)
    constructor(ctx: EventContext, event?: Event) {
        this.runtime = ctx._chain.runtime
        this.event = event || ctx.event
        assert(this.event.name === 'OffchainComputingWorkers.ImplDeploymentScopeUpdated')
    }

    get isV100(): boolean {
        return this.runtime.getEventTypeHash('OffchainComputingWorkers.ImplDeploymentScopeUpdated') === 'a8d51f484c5707408d8dd5e46ac0c7a8782f1ee389019a58156124a38fb90d8f'
    }

    get asV100(): {implId: number, scope: v100.ApplicableScope} {
        assert(this.isV100)
        return this.runtime.decodeJsonEvent(this.event)
    }
}

export class OffchainComputingWorkersImplDeregisteredEvent {
    private readonly runtime: Runtime
    private readonly event: Event

    constructor(ctx: EventContext)
    constructor(ctx: ChainContext, event: Event)
    constructor(ctx: EventContext, event?: Event) {
        this.runtime = ctx._chain.runtime
        this.event = event || ctx.event
        assert(this.event.name === 'OffchainComputingWorkers.ImplDeregistered')
    }

    get isV100(): boolean {
        return this.runtime.getEventTypeHash('OffchainComputingWorkers.ImplDeregistered') === '243c45b1eb352aca71bba131e47d5eeda77b36576c7265093d0ba2a43295cdf0'
    }

    get asV100(): {implId: number} {
        assert(this.isV100)
        return this.runtime.decodeJsonEvent(this.event)
    }
}

export class OffchainComputingWorkersImplMetadataRemovedEvent {
    private readonly runtime: Runtime
    private readonly event: Event

    constructor(ctx: EventContext)
    constructor(ctx: ChainContext, event: Event)
    constructor(ctx: EventContext, event?: Event) {
        this.runtime = ctx._chain.runtime
        this.event = event || ctx.event
        assert(this.event.name === 'OffchainComputingWorkers.ImplMetadataRemoved')
    }

    get isV100(): boolean {
        return this.runtime.getEventTypeHash('OffchainComputingWorkers.ImplMetadataRemoved') === '243c45b1eb352aca71bba131e47d5eeda77b36576c7265093d0ba2a43295cdf0'
    }

    get asV100(): {implId: number} {
        assert(this.isV100)
        return this.runtime.decodeJsonEvent(this.event)
    }
}

export class OffchainComputingWorkersImplMetadataUpdatedEvent {
    private readonly runtime: Runtime
    private readonly event: Event

    constructor(ctx: EventContext)
    constructor(ctx: ChainContext, event: Event)
    constructor(ctx: EventContext, event?: Event) {
        this.runtime = ctx._chain.runtime
        this.event = event || ctx.event
        assert(this.event.name === 'OffchainComputingWorkers.ImplMetadataUpdated')
    }

    get isV100(): boolean {
        return this.runtime.getEventTypeHash('OffchainComputingWorkers.ImplMetadataUpdated') === 'a8b2895eef238f3190b37e1c01543c35b24426912864c569b1f90d9a44dada71'
    }

    get asV100(): {implId: number, metadata: Uint8Array} {
        assert(this.isV100)
        return this.runtime.decodeJsonEvent(this.event)
    }
}

export class OffchainComputingWorkersImplRegisteredEvent {
    private readonly runtime: Runtime
    private readonly event: Event

    constructor(ctx: EventContext)
    constructor(ctx: ChainContext, event: Event)
    constructor(ctx: EventContext, event?: Event) {
        this.runtime = ctx._chain.runtime
        this.event = event || ctx.event
        assert(this.event.name === 'OffchainComputingWorkers.ImplRegistered')
    }

    get isV100(): boolean {
        return this.runtime.getEventTypeHash('OffchainComputingWorkers.ImplRegistered') === '75000c65c0d1eecfcc869ed0d53b023ed8b7970875387cd7ce5ca4ece26d9bd3'
    }

    get asV100(): {implId: number, owner: Uint8Array, attestationMethod: v100.AttestationMethod, deploymentScope: v100.ApplicableScope} {
        assert(this.isV100)
        return this.runtime.decodeJsonEvent(this.event)
    }
}

export class OffchainComputingWorkersWorkerAttestationRefreshedEvent {
    private readonly runtime: Runtime
    private readonly event: Event

    constructor(ctx: EventContext)
    constructor(ctx: ChainContext, event: Event)
    constructor(ctx: EventContext, event?: Event) {
        this.runtime = ctx._chain.runtime
        this.event = event || ctx.event
        assert(this.event.name === 'OffchainComputingWorkers.WorkerAttestationRefreshed')
    }

    /**
     * The worker refresh its attestation successfully
     */
    get isV100(): boolean {
        return this.runtime.getEventTypeHash('OffchainComputingWorkers.WorkerAttestationRefreshed') === '11311569c0b5d8d9a141443d6585b6f61dbfbe570c7b44dbb38dec80cbd91856'
    }

    /**
     * The worker refresh its attestation successfully
     */
    get asV100(): {worker: Uint8Array, expiresAt: (bigint | undefined)} {
        assert(this.isV100)
        return this.runtime.decodeJsonEvent(this.event)
    }
}

export class OffchainComputingWorkersWorkerDeregisteredEvent {
    private readonly runtime: Runtime
    private readonly event: Event

    constructor(ctx: EventContext)
    constructor(ctx: ChainContext, event: Event)
    constructor(ctx: EventContext, event?: Event) {
        this.runtime = ctx._chain.runtime
        this.event = event || ctx.event
        assert(this.event.name === 'OffchainComputingWorkers.WorkerDeregistered')
    }

    /**
     * The worker registered successfully
     */
    get isV100(): boolean {
        return this.runtime.getEventTypeHash('OffchainComputingWorkers.WorkerDeregistered') === '89afc0a50a4c6c7a30b8f0f59fabb0595938157214c865a0e46fcb20cd60fef4'
    }

    /**
     * The worker registered successfully
     */
    get asV100(): {worker: Uint8Array, force: boolean} {
        assert(this.isV100)
        return this.runtime.decodeJsonEvent(this.event)
    }
}

export class OffchainComputingWorkersWorkerHeartbeatReceivedEvent {
    private readonly runtime: Runtime
    private readonly event: Event

    constructor(ctx: EventContext)
    constructor(ctx: ChainContext, event: Event)
    constructor(ctx: EventContext, event?: Event) {
        this.runtime = ctx._chain.runtime
        this.event = event || ctx.event
        assert(this.event.name === 'OffchainComputingWorkers.WorkerHeartbeatReceived')
    }

    /**
     * The worker send heartbeat successfully
     */
    get isV100(): boolean {
        return this.runtime.getEventTypeHash('OffchainComputingWorkers.WorkerHeartbeatReceived') === '9821db54279b938f80c0331e3b1a0e59c27d3a5404e3ff37153b9e0337713e96'
    }

    /**
     * The worker send heartbeat successfully
     */
    get asV100(): {worker: Uint8Array, next: number, uptime: bigint} {
        assert(this.isV100)
        return this.runtime.decodeJsonEvent(this.event)
    }
}

export class OffchainComputingWorkersWorkerOfflineEvent {
    private readonly runtime: Runtime
    private readonly event: Event

    constructor(ctx: EventContext)
    constructor(ctx: ChainContext, event: Event)
    constructor(ctx: EventContext, event?: Event) {
        this.runtime = ctx._chain.runtime
        this.event = event || ctx.event
        assert(this.event.name === 'OffchainComputingWorkers.WorkerOffline')
    }

    /**
     * The worker is offline
     */
    get isV100(): boolean {
        return this.runtime.getEventTypeHash('OffchainComputingWorkers.WorkerOffline') === '706d1637d5f1fb899e7a3e835222f70500beae35e8007feefe5671bde57b1b0e'
    }

    /**
     * The worker is offline
     */
    get asV100(): {worker: Uint8Array, reason: v100.OfflineReason} {
        assert(this.isV100)
        return this.runtime.decodeJsonEvent(this.event)
    }
}

export class OffchainComputingWorkersWorkerOnlineEvent {
    private readonly runtime: Runtime
    private readonly event: Event

    constructor(ctx: EventContext)
    constructor(ctx: ChainContext, event: Event)
    constructor(ctx: EventContext, event?: Event) {
        this.runtime = ctx._chain.runtime
        this.event = event || ctx.event
        assert(this.event.name === 'OffchainComputingWorkers.WorkerOnline')
    }

    /**
     * The worker is online
     */
    get isV100(): boolean {
        return this.runtime.getEventTypeHash('OffchainComputingWorkers.WorkerOnline') === 'aaf3d9ba5d2b758f255a0585bcf74f1a8760c5debb4946ed2bf74fdc7a0262f3'
    }

    /**
     * The worker is online
     */
    get asV100(): {worker: Uint8Array, implSpecVersion: number, implBuildVersion: number, attestationMethod: v100.AttestationMethod, attestationExpiresAt: (bigint | undefined), nextHeartbeat: number} {
        assert(this.isV100)
        return this.runtime.decodeJsonEvent(this.event)
    }
}

export class OffchainComputingWorkersWorkerRegisteredEvent {
    private readonly runtime: Runtime
    private readonly event: Event

    constructor(ctx: EventContext)
    constructor(ctx: ChainContext, event: Event)
    constructor(ctx: EventContext, event?: Event) {
        this.runtime = ctx._chain.runtime
        this.event = event || ctx.event
        assert(this.event.name === 'OffchainComputingWorkers.WorkerRegistered')
    }

    /**
     * The worker registered successfully
     */
    get isV100(): boolean {
        return this.runtime.getEventTypeHash('OffchainComputingWorkers.WorkerRegistered') === '827feabbaece11c6815c2990c6cee79b02f149ad9405967a3410ef1fe8258576'
    }

    /**
     * The worker registered successfully
     */
    get asV100(): {worker: Uint8Array, owner: Uint8Array, implId: number} {
        assert(this.isV100)
        return this.runtime.decodeJsonEvent(this.event)
    }
}

export class OffchainComputingWorkersWorkerRequestingOfflineEvent {
    private readonly runtime: Runtime
    private readonly event: Event

    constructor(ctx: EventContext)
    constructor(ctx: ChainContext, event: Event)
    constructor(ctx: EventContext, event?: Event) {
        this.runtime = ctx._chain.runtime
        this.event = event || ctx.event
        assert(this.event.name === 'OffchainComputingWorkers.WorkerRequestingOffline')
    }

    /**
     * The worker is requesting offline
     */
    get isV100(): boolean {
        return this.runtime.getEventTypeHash('OffchainComputingWorkers.WorkerRequestingOffline') === '92a756e29fcd5c189000de949169e0c1ab6089e641fedb90b510a59a4b108e28'
    }

    /**
     * The worker is requesting offline
     */
    get asV100(): {worker: Uint8Array} {
        assert(this.isV100)
        return this.runtime.decodeJsonEvent(this.event)
    }
}

export class OffchainComputingWorkersWorkerUnresponsiveEvent {
    private readonly runtime: Runtime
    private readonly event: Event

    constructor(ctx: EventContext)
    constructor(ctx: ChainContext, event: Event)
    constructor(ctx: EventContext, event?: Event) {
        this.runtime = ctx._chain.runtime
        this.event = event || ctx.event
        assert(this.event.name === 'OffchainComputingWorkers.WorkerUnresponsive')
    }

    get isV100(): boolean {
        return this.runtime.getEventTypeHash('OffchainComputingWorkers.WorkerUnresponsive') === '92a756e29fcd5c189000de949169e0c1ab6089e641fedb90b510a59a4b108e28'
    }

    get asV100(): {worker: Uint8Array} {
        assert(this.isV100)
        return this.runtime.decodeJsonEvent(this.event)
    }
}
