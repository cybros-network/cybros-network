import assert from 'assert'
import {Chain, ChainContext, EventContext, Event, Result, Option} from './support'
import * as v100 from './v100'

export class BalancesTransferEvent {
    private readonly _chain: Chain
    private readonly event: Event

    constructor(ctx: EventContext)
    constructor(ctx: ChainContext, event: Event)
    constructor(ctx: EventContext, event?: Event) {
        event = event || ctx.event
        assert(event.name === 'Balances.Transfer')
        this._chain = ctx._chain
        this.event = event
    }

    /**
     * Transfer succeeded.
     */
    get isV100(): boolean {
        return this._chain.getEventHash('Balances.Transfer') === '0ffdf35c495114c2d42a8bf6c241483fd5334ca0198662e14480ad040f1e3a66'
    }

    /**
     * Transfer succeeded.
     */
    get asV100(): {from: Uint8Array, to: Uint8Array, amount: bigint} {
        assert(this.isV100)
        return this._chain.decodeEvent(this.event)
    }
}

export class OffchainComputingCreatingTaskPolicyCreatedEvent {
    private readonly _chain: Chain
    private readonly event: Event

    constructor(ctx: EventContext)
    constructor(ctx: ChainContext, event: Event)
    constructor(ctx: EventContext, event?: Event) {
        event = event || ctx.event
        assert(event.name === 'OffchainComputing.CreatingTaskPolicyCreated')
        this._chain = ctx._chain
        this.event = event
    }

    get isV100(): boolean {
        return this._chain.getEventHash('OffchainComputing.CreatingTaskPolicyCreated') === '9d17c79bfaaf9e911a76594d33144c3e8ef0b1bd8adf3df7c972026e61f2a80c'
    }

    get asV100(): {poolId: number, policyId: number, policy: v100.CreatingTaskPolicy} {
        assert(this.isV100)
        return this._chain.decodeEvent(this.event)
    }
}

export class OffchainComputingCreatingTaskPolicyDestroyedEvent {
    private readonly _chain: Chain
    private readonly event: Event

    constructor(ctx: EventContext)
    constructor(ctx: ChainContext, event: Event)
    constructor(ctx: EventContext, event?: Event) {
        event = event || ctx.event
        assert(event.name === 'OffchainComputing.CreatingTaskPolicyDestroyed')
        this._chain = ctx._chain
        this.event = event
    }

    get isV100(): boolean {
        return this._chain.getEventHash('OffchainComputing.CreatingTaskPolicyDestroyed') === '0a745627569b6e48894cab5a1744a734efdb1d3dfb30516a25ee14ebc11efd29'
    }

    get asV100(): {poolId: number, policyId: number} {
        assert(this.isV100)
        return this._chain.decodeEvent(this.event)
    }
}

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

export class OffchainComputingPoolCreatingTaskAbilityDisabledEvent {
    private readonly _chain: Chain
    private readonly event: Event

    constructor(ctx: EventContext)
    constructor(ctx: ChainContext, event: Event)
    constructor(ctx: EventContext, event?: Event) {
        event = event || ctx.event
        assert(event.name === 'OffchainComputing.PoolCreatingTaskAbilityDisabled')
        this._chain = ctx._chain
        this.event = event
    }

    get isV100(): boolean {
        return this._chain.getEventHash('OffchainComputing.PoolCreatingTaskAbilityDisabled') === 'a662258b1bdb045a915972ea29e9ec0b46cdd5598b0da37b0e70ac766e3735a0'
    }

    get asV100(): {poolId: number} {
        assert(this.isV100)
        return this._chain.decodeEvent(this.event)
    }
}

export class OffchainComputingPoolCreatingTaskAbilityEnabledEvent {
    private readonly _chain: Chain
    private readonly event: Event

    constructor(ctx: EventContext)
    constructor(ctx: ChainContext, event: Event)
    constructor(ctx: EventContext, event?: Event) {
        event = event || ctx.event
        assert(event.name === 'OffchainComputing.PoolCreatingTaskAbilityEnabled')
        this._chain = ctx._chain
        this.event = event
    }

    get isV100(): boolean {
        return this._chain.getEventHash('OffchainComputing.PoolCreatingTaskAbilityEnabled') === 'a662258b1bdb045a915972ea29e9ec0b46cdd5598b0da37b0e70ac766e3735a0'
    }

    get asV100(): {poolId: number} {
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
        return this._chain.getEventHash('OffchainComputing.TaskCreated') === 'b3c3478ca5437b2afc09df5dbd4cfcee90b496e2d83b57d71ac6d58bffefbf9b'
    }

    get asV100(): {poolId: number, taskId: number, owner: Uint8Array, implSpecVersion: number, input: (Uint8Array | undefined)} {
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
        return this._chain.getEventHash('OffchainComputing.TaskStatusUpdated') === '2c49657978f9d3d701dba6fad01ddc4bf656097e97a5b8d9f183e32676f2323e'
    }

    get asV100(): {poolId: number, taskId: number, status: v100.TaskStatus} {
        assert(this.isV100)
        return this._chain.decodeEvent(this.event)
    }
}

export class OffchainComputingWorkerAddedEvent {
    private readonly _chain: Chain
    private readonly event: Event

    constructor(ctx: EventContext)
    constructor(ctx: ChainContext, event: Event)
    constructor(ctx: EventContext, event?: Event) {
        event = event || ctx.event
        assert(event.name === 'OffchainComputing.WorkerAdded')
        this._chain = ctx._chain
        this.event = event
    }

    get isV100(): boolean {
        return this._chain.getEventHash('OffchainComputing.WorkerAdded') === '797c331fcd879fe634a85213860a97ecd0575adecd2689f258b95e6c04d3cdf3'
    }

    get asV100(): {poolId: number, worker: Uint8Array} {
        assert(this.isV100)
        return this._chain.decodeEvent(this.event)
    }
}

export class OffchainComputingWorkerRemovedEvent {
    private readonly _chain: Chain
    private readonly event: Event

    constructor(ctx: EventContext)
    constructor(ctx: ChainContext, event: Event)
    constructor(ctx: EventContext, event?: Event) {
        event = event || ctx.event
        assert(event.name === 'OffchainComputing.WorkerRemoved')
        this._chain = ctx._chain
        this.event = event
    }

    get isV100(): boolean {
        return this._chain.getEventHash('OffchainComputing.WorkerRemoved') === '797c331fcd879fe634a85213860a97ecd0575adecd2689f258b95e6c04d3cdf3'
    }

    get asV100(): {poolId: number, worker: Uint8Array} {
        assert(this.isV100)
        return this._chain.decodeEvent(this.event)
    }
}

export class OffchainComputingWorkersImplBuildMagicBytesDeregisteredEvent {
    private readonly _chain: Chain
    private readonly event: Event

    constructor(ctx: EventContext)
    constructor(ctx: ChainContext, event: Event)
    constructor(ctx: EventContext, event?: Event) {
        event = event || ctx.event
        assert(event.name === 'OffchainComputingWorkers.ImplBuildMagicBytesDeregistered')
        this._chain = ctx._chain
        this.event = event
    }

    /**
     * Remove worker's implementation permission successfully
     */
    get isV100(): boolean {
        return this._chain.getEventHash('OffchainComputingWorkers.ImplBuildMagicBytesDeregistered') === 'e62f88665216d9128c4b0263040c3a99e1740c0f644f00da67b2091ba9a518b4'
    }

    /**
     * Remove worker's implementation permission successfully
     */
    get asV100(): {implId: number, version: number} {
        assert(this.isV100)
        return this._chain.decodeEvent(this.event)
    }
}

export class OffchainComputingWorkersImplBuildMagicBytesRegisteredEvent {
    private readonly _chain: Chain
    private readonly event: Event

    constructor(ctx: EventContext)
    constructor(ctx: ChainContext, event: Event)
    constructor(ctx: EventContext, event?: Event) {
        event = event || ctx.event
        assert(event.name === 'OffchainComputingWorkers.ImplBuildMagicBytesRegistered')
        this._chain = ctx._chain
        this.event = event
    }

    /**
     * Update worker's implementation permission successfully
     */
    get isV100(): boolean {
        return this._chain.getEventHash('OffchainComputingWorkers.ImplBuildMagicBytesRegistered') === 'aa21801f9e15ed0370b3dd75d5717f56aeae55aa98ec04ff7b21a89b18a6696e'
    }

    /**
     * Update worker's implementation permission successfully
     */
    get asV100(): {implId: number, version: number, magicBytes: Uint8Array} {
        assert(this.isV100)
        return this._chain.decodeEvent(this.event)
    }
}

export class OffchainComputingWorkersImplBuildRestrictionUpdatedEvent {
    private readonly _chain: Chain
    private readonly event: Event

    constructor(ctx: EventContext)
    constructor(ctx: ChainContext, event: Event)
    constructor(ctx: EventContext, event?: Event) {
        event = event || ctx.event
        assert(event.name === 'OffchainComputingWorkers.ImplBuildRestrictionUpdated')
        this._chain = ctx._chain
        this.event = event
    }

    /**
     * Update worker's implementation permission successfully
     */
    get isV100(): boolean {
        return this._chain.getEventHash('OffchainComputingWorkers.ImplBuildRestrictionUpdated') === '9e16eb1c7e4daee3c0478eb3c34e1be9c4c267072a56cb3380bc118166750a42'
    }

    /**
     * Update worker's implementation permission successfully
     */
    get asV100(): {implId: number, restriction: v100.ImplBuildRestriction} {
        assert(this.isV100)
        return this._chain.decodeEvent(this.event)
    }
}

export class OffchainComputingWorkersImplDeploymentPermissionUpdatedEvent {
    private readonly _chain: Chain
    private readonly event: Event

    constructor(ctx: EventContext)
    constructor(ctx: ChainContext, event: Event)
    constructor(ctx: EventContext, event?: Event) {
        event = event || ctx.event
        assert(event.name === 'OffchainComputingWorkers.ImplDeploymentPermissionUpdated')
        this._chain = ctx._chain
        this.event = event
    }

    get isV100(): boolean {
        return this._chain.getEventHash('OffchainComputingWorkers.ImplDeploymentPermissionUpdated') === '0763fa693ed209293d902998637037491337060220b73fc06bd1a9f1745dfe38'
    }

    get asV100(): {implId: number, permission: v100.ImplDeploymentPermission} {
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
        return this._chain.getEventHash('OffchainComputingWorkers.ImplRegistered') === 'ba1e6763b00cbe201f80688ce9a1c67e1ffaaf5533e833a3c3522d1010bef1d9'
    }

    get asV100(): {implId: number, owner: Uint8Array, attestationMethod: v100.AttestationMethod, deploymentPermission: v100.ImplDeploymentPermission} {
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
        return this._chain.getEventHash('OffchainComputingWorkers.WorkerHeartbeatReceived') === '7403811140e553a4cf3aec9ce48e018c7cd49db3a167b82a38a69405f624faa5'
    }

    /**
     * The worker send heartbeat successfully
     */
    get asV100(): {worker: Uint8Array, next: number} {
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
        return this._chain.getEventHash('OffchainComputingWorkers.WorkerOffline') === '4588e786dfa06b399d2a2382d0a63a2bd9e2ab1cbf297329e9c2db55cc4882a3'
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
        return this._chain.getEventHash('OffchainComputingWorkers.WorkerOnline') === '632d286ce41219bc0f48bfe4d80d993b03cfce1f26e49e617a19c9fecb433100'
    }

    /**
     * The worker is online
     */
    get asV100(): {worker: Uint8Array, implId: number, implSpecVersion: number, implBuildVersion: number, attestationMethod: v100.AttestationMethod, attestationExpiresAt: (bigint | undefined), nextHeartbeat: number} {
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
        return this._chain.getEventHash('OffchainComputingWorkers.WorkerRegistered') === 'dbc381acde3e58a1dc63279d84f3fde21af33601d2a5e26e6755db85003b8aff'
    }

    /**
     * The worker registered successfully
     */
    get asV100(): {worker: Uint8Array, owner: Uint8Array} {
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
