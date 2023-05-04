import assert from 'assert'
import {Chain, ChainContext, EventContext, Event, Result, Option} from './support'
import * as v100 from './v100'

export class OffchainComputingPoolCreatedEvent {
    private readonly _chain: Chain
    private readonly event: Event

    constructor(ctx: EventContext)
    constructor(ctx: ChainContext, event: Event)
    constructor(ctx: EventContext, event?: Event) {
        event = event || ctx.event
        assert(event.name === 'OffchainComputing.PoolCreated')
        this._chain = ctx._chain
        this.event = event
    }

    get isV100(): boolean {
        return this._chain.getEventHash('OffchainComputing.PoolCreated') === 'a9ac9e4043ff752859bf4ca3b9ad673f6296e4a2ae65a6e9c16866bb25bb6aaa'
    }

    get asV100(): {owner: Uint8Array, poolId: number, implId: number} {
        assert(this.isV100)
        return this._chain.decodeEvent(this.event)
    }
}

export class OffchainComputingPoolCreatingTaskAvailabilityUpdatedEvent {
    private readonly _chain: Chain
    private readonly event: Event

    constructor(ctx: EventContext)
    constructor(ctx: ChainContext, event: Event)
    constructor(ctx: EventContext, event?: Event) {
        event = event || ctx.event
        assert(event.name === 'OffchainComputing.PoolCreatingTaskAvailabilityUpdated')
        this._chain = ctx._chain
        this.event = event
    }

    get isV100(): boolean {
        return this._chain.getEventHash('OffchainComputing.PoolCreatingTaskAvailabilityUpdated') === 'e873f768ffe88b6663087f7f3610b1ad0d07087456214e4325e5498fab453ea5'
    }

    get asV100(): {poolId: number, availability: boolean} {
        assert(this.isV100)
        return this._chain.decodeEvent(this.event)
    }
}

export class OffchainComputingPoolDestroyedEvent {
    private readonly _chain: Chain
    private readonly event: Event

    constructor(ctx: EventContext)
    constructor(ctx: ChainContext, event: Event)
    constructor(ctx: EventContext, event?: Event) {
        event = event || ctx.event
        assert(event.name === 'OffchainComputing.PoolDestroyed')
        this._chain = ctx._chain
        this.event = event
    }

    get isV100(): boolean {
        return this._chain.getEventHash('OffchainComputing.PoolDestroyed') === 'a662258b1bdb045a915972ea29e9ec0b46cdd5598b0da37b0e70ac766e3735a0'
    }

    get asV100(): {poolId: number} {
        assert(this.isV100)
        return this._chain.decodeEvent(this.event)
    }
}

export class OffchainComputingPoolMetadataRemovedEvent {
    private readonly _chain: Chain
    private readonly event: Event

    constructor(ctx: EventContext)
    constructor(ctx: ChainContext, event: Event)
    constructor(ctx: EventContext, event?: Event) {
        event = event || ctx.event
        assert(event.name === 'OffchainComputing.PoolMetadataRemoved')
        this._chain = ctx._chain
        this.event = event
    }

    get isV100(): boolean {
        return this._chain.getEventHash('OffchainComputing.PoolMetadataRemoved') === 'a662258b1bdb045a915972ea29e9ec0b46cdd5598b0da37b0e70ac766e3735a0'
    }

    get asV100(): {poolId: number} {
        assert(this.isV100)
        return this._chain.decodeEvent(this.event)
    }
}

export class OffchainComputingPoolMetadataUpdatedEvent {
    private readonly _chain: Chain
    private readonly event: Event

    constructor(ctx: EventContext)
    constructor(ctx: ChainContext, event: Event)
    constructor(ctx: EventContext, event?: Event) {
        event = event || ctx.event
        assert(event.name === 'OffchainComputing.PoolMetadataUpdated')
        this._chain = ctx._chain
        this.event = event
    }

    get isV100(): boolean {
        return this._chain.getEventHash('OffchainComputing.PoolMetadataUpdated') === '027a45601396dd984f964705eb7eabc42207aedba700c7316c5f0201a21bc953'
    }

    get asV100(): {poolId: number, metadata: Uint8Array} {
        assert(this.isV100)
        return this._chain.decodeEvent(this.event)
    }
}

export class OffchainComputingTaskAssignedEvent {
    private readonly _chain: Chain
    private readonly event: Event

    constructor(ctx: EventContext)
    constructor(ctx: ChainContext, event: Event)
    constructor(ctx: EventContext, event?: Event) {
        event = event || ctx.event
        assert(event.name === 'OffchainComputing.TaskAssigned')
        this._chain = ctx._chain
        this.event = event
    }

    get isV100(): boolean {
        return this._chain.getEventHash('OffchainComputing.TaskAssigned') === '7e59e249e6907cc336a441cd3879c996f2b722d83af3ec95d9fe017ebd6ff67b'
    }

    get asV100(): {poolId: number, taskId: number, assignee: Uint8Array} {
        assert(this.isV100)
        return this._chain.decodeEvent(this.event)
    }
}

export class OffchainComputingTaskCreatedEvent {
    private readonly _chain: Chain
    private readonly event: Event

    constructor(ctx: EventContext)
    constructor(ctx: ChainContext, event: Event)
    constructor(ctx: EventContext, event?: Event) {
        event = event || ctx.event
        assert(event.name === 'OffchainComputing.TaskCreated')
        this._chain = ctx._chain
        this.event = event
    }

    get isV100(): boolean {
        return this._chain.getEventHash('OffchainComputing.TaskCreated') === '93d876ce0a7d0efd8e609ff81e6e41fe7803b746c42b793afa896fe7afa2f092'
    }

    get asV100(): {poolId: number, taskId: number, policyId: number, owner: Uint8Array, implSpecVersion: number, input: (Uint8Array | undefined), expiresIn: bigint} {
        assert(this.isV100)
        return this._chain.decodeEvent(this.event)
    }
}

export class OffchainComputingTaskDestroyedEvent {
    private readonly _chain: Chain
    private readonly event: Event

    constructor(ctx: EventContext)
    constructor(ctx: ChainContext, event: Event)
    constructor(ctx: EventContext, event?: Event) {
        event = event || ctx.event
        assert(event.name === 'OffchainComputing.TaskDestroyed')
        this._chain = ctx._chain
        this.event = event
    }

    get isV100(): boolean {
        return this._chain.getEventHash('OffchainComputing.TaskDestroyed') === '783cfd4d481195c995e6ebabb8223a8ea568b07b8279c8a0e660c5960a5c7cd8'
    }

    get asV100(): {poolId: number, taskId: number, destroyer: Uint8Array} {
        assert(this.isV100)
        return this._chain.decodeEvent(this.event)
    }
}

export class OffchainComputingTaskPolicyAvailabilityUpdatedEvent {
    private readonly _chain: Chain
    private readonly event: Event

    constructor(ctx: EventContext)
    constructor(ctx: ChainContext, event: Event)
    constructor(ctx: EventContext, event?: Event) {
        event = event || ctx.event
        assert(event.name === 'OffchainComputing.TaskPolicyAvailabilityUpdated')
        this._chain = ctx._chain
        this.event = event
    }

    get isV100(): boolean {
        return this._chain.getEventHash('OffchainComputing.TaskPolicyAvailabilityUpdated') === '898c4597e9122ea992f8c2888507b78530394fb594c42efbf45bc2728f464363'
    }

    get asV100(): {poolId: number, policyId: number, availability: boolean} {
        assert(this.isV100)
        return this._chain.decodeEvent(this.event)
    }
}

export class OffchainComputingTaskPolicyCreatedEvent {
    private readonly _chain: Chain
    private readonly event: Event

    constructor(ctx: EventContext)
    constructor(ctx: ChainContext, event: Event)
    constructor(ctx: EventContext, event?: Event) {
        event = event || ctx.event
        assert(event.name === 'OffchainComputing.TaskPolicyCreated')
        this._chain = ctx._chain
        this.event = event
    }

    get isV100(): boolean {
        return this._chain.getEventHash('OffchainComputing.TaskPolicyCreated') === 'cf77fa43433bd91a20f89e458b4aad071fb4ed3ea38be1146dd2462f24583b76'
    }

    get asV100(): {poolId: number, policyId: number, creatingTaskScope: v100.ApplicableScope, startBlock: (number | undefined), endBlock: (number | undefined)} {
        assert(this.isV100)
        return this._chain.decodeEvent(this.event)
    }
}

export class OffchainComputingTaskPolicyDestroyedEvent {
    private readonly _chain: Chain
    private readonly event: Event

    constructor(ctx: EventContext)
    constructor(ctx: ChainContext, event: Event)
    constructor(ctx: EventContext, event?: Event) {
        event = event || ctx.event
        assert(event.name === 'OffchainComputing.TaskPolicyDestroyed')
        this._chain = ctx._chain
        this.event = event
    }

    get isV100(): boolean {
        return this._chain.getEventHash('OffchainComputing.TaskPolicyDestroyed') === '0a745627569b6e48894cab5a1744a734efdb1d3dfb30516a25ee14ebc11efd29'
    }

    get asV100(): {poolId: number, policyId: number} {
        assert(this.isV100)
        return this._chain.decodeEvent(this.event)
    }
}

export class OffchainComputingTaskReleasedEvent {
    private readonly _chain: Chain
    private readonly event: Event

    constructor(ctx: EventContext)
    constructor(ctx: ChainContext, event: Event)
    constructor(ctx: EventContext, event?: Event) {
        event = event || ctx.event
        assert(event.name === 'OffchainComputing.TaskReleased')
        this._chain = ctx._chain
        this.event = event
    }

    get isV100(): boolean {
        return this._chain.getEventHash('OffchainComputing.TaskReleased') === 'd5c91082f6820aa9eaf82fb1d956566efe99689b6746c2c2f384bbb0515c3bea'
    }

    get asV100(): {poolId: number, taskId: number} {
        assert(this.isV100)
        return this._chain.decodeEvent(this.event)
    }
}

export class OffchainComputingTaskResultUpdatedEvent {
    private readonly _chain: Chain
    private readonly event: Event

    constructor(ctx: EventContext)
    constructor(ctx: ChainContext, event: Event)
    constructor(ctx: EventContext, event?: Event) {
        event = event || ctx.event
        assert(event.name === 'OffchainComputing.TaskResultUpdated')
        this._chain = ctx._chain
        this.event = event
    }

    get isV100(): boolean {
        return this._chain.getEventHash('OffchainComputing.TaskResultUpdated') === 'a9813d17fe7f3170522f7fd3ac94bc16918e1f11ce13300813980a8f593590ad'
    }

    get asV100(): {poolId: number, taskId: number, result: v100.TaskResult, output: (Uint8Array | undefined), proof: (Uint8Array | undefined)} {
        assert(this.isV100)
        return this._chain.decodeEvent(this.event)
    }
}

export class OffchainComputingTaskStatusUpdatedEvent {
    private readonly _chain: Chain
    private readonly event: Event

    constructor(ctx: EventContext)
    constructor(ctx: ChainContext, event: Event)
    constructor(ctx: EventContext, event?: Event) {
        event = event || ctx.event
        assert(event.name === 'OffchainComputing.TaskStatusUpdated')
        this._chain = ctx._chain
        this.event = event
    }

    get isV100(): boolean {
        return this._chain.getEventHash('OffchainComputing.TaskStatusUpdated') === '1cff08689f48bed99324800b5d74f3f579e0dd05dadb1d1ad4661f9ac02be73b'
    }

    get asV100(): {poolId: number, taskId: number, status: v100.TaskStatus} {
        assert(this.isV100)
        return this._chain.decodeEvent(this.event)
    }
}

export class OffchainComputingWorkerAuthorizedEvent {
    private readonly _chain: Chain
    private readonly event: Event

    constructor(ctx: EventContext)
    constructor(ctx: ChainContext, event: Event)
    constructor(ctx: EventContext, event?: Event) {
        event = event || ctx.event
        assert(event.name === 'OffchainComputing.WorkerAuthorized')
        this._chain = ctx._chain
        this.event = event
    }

    get isV100(): boolean {
        return this._chain.getEventHash('OffchainComputing.WorkerAuthorized') === '797c331fcd879fe634a85213860a97ecd0575adecd2689f258b95e6c04d3cdf3'
    }

    get asV100(): {poolId: number, worker: Uint8Array} {
        assert(this.isV100)
        return this._chain.decodeEvent(this.event)
    }
}

export class OffchainComputingWorkerRevokedEvent {
    private readonly _chain: Chain
    private readonly event: Event

    constructor(ctx: EventContext)
    constructor(ctx: ChainContext, event: Event)
    constructor(ctx: EventContext, event?: Event) {
        event = event || ctx.event
        assert(event.name === 'OffchainComputing.WorkerRevoked')
        this._chain = ctx._chain
        this.event = event
    }

    get isV100(): boolean {
        return this._chain.getEventHash('OffchainComputing.WorkerRevoked') === '797c331fcd879fe634a85213860a97ecd0575adecd2689f258b95e6c04d3cdf3'
    }

    get asV100(): {poolId: number, worker: Uint8Array} {
        assert(this.isV100)
        return this._chain.decodeEvent(this.event)
    }
}

export class OffchainComputingWorkerSubscribedEvent {
    private readonly _chain: Chain
    private readonly event: Event

    constructor(ctx: EventContext)
    constructor(ctx: ChainContext, event: Event)
    constructor(ctx: EventContext, event?: Event) {
        event = event || ctx.event
        assert(event.name === 'OffchainComputing.WorkerSubscribed')
        this._chain = ctx._chain
        this.event = event
    }

    get isV100(): boolean {
        return this._chain.getEventHash('OffchainComputing.WorkerSubscribed') === '2bf1f63dd355996120e05c07b3507c93d7b4611ab4df91424d8bc633e8293fed'
    }

    get asV100(): {worker: Uint8Array, poolId: number} {
        assert(this.isV100)
        return this._chain.decodeEvent(this.event)
    }
}

export class OffchainComputingWorkerUnsubscribedEvent {
    private readonly _chain: Chain
    private readonly event: Event

    constructor(ctx: EventContext)
    constructor(ctx: ChainContext, event: Event)
    constructor(ctx: EventContext, event?: Event) {
        event = event || ctx.event
        assert(event.name === 'OffchainComputing.WorkerUnsubscribed')
        this._chain = ctx._chain
        this.event = event
    }

    get isV100(): boolean {
        return this._chain.getEventHash('OffchainComputing.WorkerUnsubscribed') === '2bf1f63dd355996120e05c07b3507c93d7b4611ab4df91424d8bc633e8293fed'
    }

    get asV100(): {worker: Uint8Array, poolId: number} {
        assert(this.isV100)
        return this._chain.decodeEvent(this.event)
    }
}

export class OffchainComputingWorkersImplBuildDeregisteredEvent {
    private readonly _chain: Chain
    private readonly event: Event

    constructor(ctx: EventContext)
    constructor(ctx: ChainContext, event: Event)
    constructor(ctx: EventContext, event?: Event) {
        event = event || ctx.event
        assert(event.name === 'OffchainComputingWorkers.ImplBuildDeregistered')
        this._chain = ctx._chain
        this.event = event
    }

    /**
     * Remove worker's implementation permission successfully
     */
    get isV100(): boolean {
        return this._chain.getEventHash('OffchainComputingWorkers.ImplBuildDeregistered') === 'ee1f58f7a32dedbbf0581d70188f18897f3ef35cb81eb50560663c7e0006d515'
    }

    /**
     * Remove worker's implementation permission successfully
     */
    get asV100(): {implId: number, implBuildVersion: number} {
        assert(this.isV100)
        return this._chain.decodeEvent(this.event)
    }
}

export class OffchainComputingWorkersImplBuildRegisteredEvent {
    private readonly _chain: Chain
    private readonly event: Event

    constructor(ctx: EventContext)
    constructor(ctx: ChainContext, event: Event)
    constructor(ctx: EventContext, event?: Event) {
        event = event || ctx.event
        assert(event.name === 'OffchainComputingWorkers.ImplBuildRegistered')
        this._chain = ctx._chain
        this.event = event
    }

    /**
     * Update worker's implementation permission successfully
     */
    get isV100(): boolean {
        return this._chain.getEventHash('OffchainComputingWorkers.ImplBuildRegistered') === 'e702a7c67fd43ee1a9344f6f419dbb8f5211c58a4fc37aa70488d08acdf695e5'
    }

    /**
     * Update worker's implementation permission successfully
     */
    get asV100(): {implId: number, implBuildVersion: number, magicBytes: (Uint8Array | undefined)} {
        assert(this.isV100)
        return this._chain.decodeEvent(this.event)
    }
}

export class OffchainComputingWorkersImplBuildStatusUpdatedEvent {
    private readonly _chain: Chain
    private readonly event: Event

    constructor(ctx: EventContext)
    constructor(ctx: ChainContext, event: Event)
    constructor(ctx: EventContext, event?: Event) {
        event = event || ctx.event
        assert(event.name === 'OffchainComputingWorkers.ImplBuildStatusUpdated')
        this._chain = ctx._chain
        this.event = event
    }

    get isV100(): boolean {
        return this._chain.getEventHash('OffchainComputingWorkers.ImplBuildStatusUpdated') === '356bb2aa1d8495aceb0ba1ac8ccd487546011b5b3333b479399cc71d3a98f7d8'
    }

    get asV100(): {implId: number, implBuildVersion: number, status: v100.ImplBuildStatus} {
        assert(this.isV100)
        return this._chain.decodeEvent(this.event)
    }
}

export class OffchainComputingWorkersImplDeploymentScopeUpdatedEvent {
    private readonly _chain: Chain
    private readonly event: Event

    constructor(ctx: EventContext)
    constructor(ctx: ChainContext, event: Event)
    constructor(ctx: EventContext, event?: Event) {
        event = event || ctx.event
        assert(event.name === 'OffchainComputingWorkers.ImplDeploymentScopeUpdated')
        this._chain = ctx._chain
        this.event = event
    }

    get isV100(): boolean {
        return this._chain.getEventHash('OffchainComputingWorkers.ImplDeploymentScopeUpdated') === 'a8d51f484c5707408d8dd5e46ac0c7a8782f1ee389019a58156124a38fb90d8f'
    }

    get asV100(): {implId: number, scope: v100.ApplicableScope} {
        assert(this.isV100)
        return this._chain.decodeEvent(this.event)
    }
}

export class OffchainComputingWorkersImplDeregisteredEvent {
    private readonly _chain: Chain
    private readonly event: Event

    constructor(ctx: EventContext)
    constructor(ctx: ChainContext, event: Event)
    constructor(ctx: EventContext, event?: Event) {
        event = event || ctx.event
        assert(event.name === 'OffchainComputingWorkers.ImplDeregistered')
        this._chain = ctx._chain
        this.event = event
    }

    get isV100(): boolean {
        return this._chain.getEventHash('OffchainComputingWorkers.ImplDeregistered') === '243c45b1eb352aca71bba131e47d5eeda77b36576c7265093d0ba2a43295cdf0'
    }

    get asV100(): {implId: number} {
        assert(this.isV100)
        return this._chain.decodeEvent(this.event)
    }
}

export class OffchainComputingWorkersImplMetadataRemovedEvent {
    private readonly _chain: Chain
    private readonly event: Event

    constructor(ctx: EventContext)
    constructor(ctx: ChainContext, event: Event)
    constructor(ctx: EventContext, event?: Event) {
        event = event || ctx.event
        assert(event.name === 'OffchainComputingWorkers.ImplMetadataRemoved')
        this._chain = ctx._chain
        this.event = event
    }

    get isV100(): boolean {
        return this._chain.getEventHash('OffchainComputingWorkers.ImplMetadataRemoved') === '243c45b1eb352aca71bba131e47d5eeda77b36576c7265093d0ba2a43295cdf0'
    }

    get asV100(): {implId: number} {
        assert(this.isV100)
        return this._chain.decodeEvent(this.event)
    }
}

export class OffchainComputingWorkersImplMetadataUpdatedEvent {
    private readonly _chain: Chain
    private readonly event: Event

    constructor(ctx: EventContext)
    constructor(ctx: ChainContext, event: Event)
    constructor(ctx: EventContext, event?: Event) {
        event = event || ctx.event
        assert(event.name === 'OffchainComputingWorkers.ImplMetadataUpdated')
        this._chain = ctx._chain
        this.event = event
    }

    get isV100(): boolean {
        return this._chain.getEventHash('OffchainComputingWorkers.ImplMetadataUpdated') === 'a8b2895eef238f3190b37e1c01543c35b24426912864c569b1f90d9a44dada71'
    }

    get asV100(): {implId: number, metadata: Uint8Array} {
        assert(this.isV100)
        return this._chain.decodeEvent(this.event)
    }
}

export class OffchainComputingWorkersImplRegisteredEvent {
    private readonly _chain: Chain
    private readonly event: Event

    constructor(ctx: EventContext)
    constructor(ctx: ChainContext, event: Event)
    constructor(ctx: EventContext, event?: Event) {
        event = event || ctx.event
        assert(event.name === 'OffchainComputingWorkers.ImplRegistered')
        this._chain = ctx._chain
        this.event = event
    }

    get isV100(): boolean {
        return this._chain.getEventHash('OffchainComputingWorkers.ImplRegistered') === '75000c65c0d1eecfcc869ed0d53b023ed8b7970875387cd7ce5ca4ece26d9bd3'
    }

    get asV100(): {implId: number, owner: Uint8Array, attestationMethod: v100.AttestationMethod, deploymentScope: v100.ApplicableScope} {
        assert(this.isV100)
        return this._chain.decodeEvent(this.event)
    }
}

export class OffchainComputingWorkersWorkerAttestationRefreshedEvent {
    private readonly _chain: Chain
    private readonly event: Event

    constructor(ctx: EventContext)
    constructor(ctx: ChainContext, event: Event)
    constructor(ctx: EventContext, event?: Event) {
        event = event || ctx.event
        assert(event.name === 'OffchainComputingWorkers.WorkerAttestationRefreshed')
        this._chain = ctx._chain
        this.event = event
    }

    /**
     * The worker refresh its attestation successfully
     */
    get isV100(): boolean {
        return this._chain.getEventHash('OffchainComputingWorkers.WorkerAttestationRefreshed') === '11311569c0b5d8d9a141443d6585b6f61dbfbe570c7b44dbb38dec80cbd91856'
    }

    /**
     * The worker refresh its attestation successfully
     */
    get asV100(): {worker: Uint8Array, expiresAt: (bigint | undefined)} {
        assert(this.isV100)
        return this._chain.decodeEvent(this.event)
    }
}

export class OffchainComputingWorkersWorkerDeregisteredEvent {
    private readonly _chain: Chain
    private readonly event: Event

    constructor(ctx: EventContext)
    constructor(ctx: ChainContext, event: Event)
    constructor(ctx: EventContext, event?: Event) {
        event = event || ctx.event
        assert(event.name === 'OffchainComputingWorkers.WorkerDeregistered')
        this._chain = ctx._chain
        this.event = event
    }

    /**
     * The worker registered successfully
     */
    get isV100(): boolean {
        return this._chain.getEventHash('OffchainComputingWorkers.WorkerDeregistered') === '89afc0a50a4c6c7a30b8f0f59fabb0595938157214c865a0e46fcb20cd60fef4'
    }

    /**
     * The worker registered successfully
     */
    get asV100(): {worker: Uint8Array, force: boolean} {
        assert(this.isV100)
        return this._chain.decodeEvent(this.event)
    }
}

export class OffchainComputingWorkersWorkerHeartbeatReceivedEvent {
    private readonly _chain: Chain
    private readonly event: Event

    constructor(ctx: EventContext)
    constructor(ctx: ChainContext, event: Event)
    constructor(ctx: EventContext, event?: Event) {
        event = event || ctx.event
        assert(event.name === 'OffchainComputingWorkers.WorkerHeartbeatReceived')
        this._chain = ctx._chain
        this.event = event
    }

    /**
     * The worker send heartbeat successfully
     */
    get isV100(): boolean {
        return this._chain.getEventHash('OffchainComputingWorkers.WorkerHeartbeatReceived') === '9821db54279b938f80c0331e3b1a0e59c27d3a5404e3ff37153b9e0337713e96'
    }

    /**
     * The worker send heartbeat successfully
     */
    get asV100(): {worker: Uint8Array, next: number, uptime: bigint} {
        assert(this.isV100)
        return this._chain.decodeEvent(this.event)
    }
}

export class OffchainComputingWorkersWorkerOfflineEvent {
    private readonly _chain: Chain
    private readonly event: Event

    constructor(ctx: EventContext)
    constructor(ctx: ChainContext, event: Event)
    constructor(ctx: EventContext, event?: Event) {
        event = event || ctx.event
        assert(event.name === 'OffchainComputingWorkers.WorkerOffline')
        this._chain = ctx._chain
        this.event = event
    }

    /**
     * The worker is offline
     */
    get isV100(): boolean {
        return this._chain.getEventHash('OffchainComputingWorkers.WorkerOffline') === 'be68ea183df47ac51c4c737f9a34a5747f849f9f967d1bc49e63e7bb170ab8d8'
    }

    /**
     * The worker is offline
     */
    get asV100(): {worker: Uint8Array, reason: v100.OfflineReason} {
        assert(this.isV100)
        return this._chain.decodeEvent(this.event)
    }
}

export class OffchainComputingWorkersWorkerOnlineEvent {
    private readonly _chain: Chain
    private readonly event: Event

    constructor(ctx: EventContext)
    constructor(ctx: ChainContext, event: Event)
    constructor(ctx: EventContext, event?: Event) {
        event = event || ctx.event
        assert(event.name === 'OffchainComputingWorkers.WorkerOnline')
        this._chain = ctx._chain
        this.event = event
    }

    /**
     * The worker is online
     */
    get isV100(): boolean {
        return this._chain.getEventHash('OffchainComputingWorkers.WorkerOnline') === 'aaf3d9ba5d2b758f255a0585bcf74f1a8760c5debb4946ed2bf74fdc7a0262f3'
    }

    /**
     * The worker is online
     */
    get asV100(): {worker: Uint8Array, implSpecVersion: number, implBuildVersion: number, attestationMethod: v100.AttestationMethod, attestationExpiresAt: (bigint | undefined), nextHeartbeat: number} {
        assert(this.isV100)
        return this._chain.decodeEvent(this.event)
    }
}

export class OffchainComputingWorkersWorkerRegisteredEvent {
    private readonly _chain: Chain
    private readonly event: Event

    constructor(ctx: EventContext)
    constructor(ctx: ChainContext, event: Event)
    constructor(ctx: EventContext, event?: Event) {
        event = event || ctx.event
        assert(event.name === 'OffchainComputingWorkers.WorkerRegistered')
        this._chain = ctx._chain
        this.event = event
    }

    /**
     * The worker registered successfully
     */
    get isV100(): boolean {
        return this._chain.getEventHash('OffchainComputingWorkers.WorkerRegistered') === '827feabbaece11c6815c2990c6cee79b02f149ad9405967a3410ef1fe8258576'
    }

    /**
     * The worker registered successfully
     */
    get asV100(): {worker: Uint8Array, owner: Uint8Array, implId: number} {
        assert(this.isV100)
        return this._chain.decodeEvent(this.event)
    }
}

export class OffchainComputingWorkersWorkerRequestingOfflineEvent {
    private readonly _chain: Chain
    private readonly event: Event

    constructor(ctx: EventContext)
    constructor(ctx: ChainContext, event: Event)
    constructor(ctx: EventContext, event?: Event) {
        event = event || ctx.event
        assert(event.name === 'OffchainComputingWorkers.WorkerRequestingOffline')
        this._chain = ctx._chain
        this.event = event
    }

    /**
     * The worker is requesting offline
     */
    get isV100(): boolean {
        return this._chain.getEventHash('OffchainComputingWorkers.WorkerRequestingOffline') === '92a756e29fcd5c189000de949169e0c1ab6089e641fedb90b510a59a4b108e28'
    }

    /**
     * The worker is requesting offline
     */
    get asV100(): {worker: Uint8Array} {
        assert(this.isV100)
        return this._chain.decodeEvent(this.event)
    }
}
