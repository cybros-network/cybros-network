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
        return this._chain.getEventHash('OffchainComputing.PoolCreated') === '2c616110def23e6045c3547d732748814ca6a6e2352a4daa66936958d7d23b9d'
    }

    get asV100(): {owner: Uint8Array, poolId: number} {
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
        return this._chain.getEventHash('OffchainComputing.PoolMetadataUpdated') === '854e8ac28f2a8b55fc7ddcc56a98b25507f02999d3e9382f1129b5bd2a1a9d2b'
    }

    get asV100(): {poolId: number, newMetadata: Uint8Array} {
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
        return this._chain.getEventHash('OffchainComputing.TaskCreated') === '092aebb658b53425c5cca9a592398faadddb774a0dca248e702c1a99103bda08'
    }

    get asV100(): {poolId: number, taskId: number, owner: Uint8Array, input: (Uint8Array | undefined)} {
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

export class OffchainComputingWorkersAttestationRefreshedEvent {
    private readonly _chain: Chain
    private readonly event: Event

    constructor(ctx: EventContext)
    constructor(ctx: ChainContext, event: Event)
    constructor(ctx: EventContext, event?: Event) {
        event = event || ctx.event
        assert(event.name === 'OffchainComputingWorkers.AttestationRefreshed')
        this._chain = ctx._chain
        this.event = event
    }

    /**
     * The worker refresh its attestation successfully
     */
    get isV100(): boolean {
        return this._chain.getEventHash('OffchainComputingWorkers.AttestationRefreshed') === '022f02b3214bd55462f8f93517f8e23036c75ca3159fbb0de643e8a5ac05a569'
    }

    /**
     * The worker refresh its attestation successfully
     */
    get asV100(): {worker: Uint8Array, method: v100.AttestationMethod} {
        assert(this.isV100)
        return this._chain.decodeEvent(this.event)
    }
}

export class OffchainComputingWorkersDeregisteredEvent {
    private readonly _chain: Chain
    private readonly event: Event

    constructor(ctx: EventContext)
    constructor(ctx: ChainContext, event: Event)
    constructor(ctx: EventContext, event?: Event) {
        event = event || ctx.event
        assert(event.name === 'OffchainComputingWorkers.Deregistered')
        this._chain = ctx._chain
        this.event = event
    }

    /**
     * The worker registered successfully
     */
    get isV100(): boolean {
        return this._chain.getEventHash('OffchainComputingWorkers.Deregistered') === '89afc0a50a4c6c7a30b8f0f59fabb0595938157214c865a0e46fcb20cd60fef4'
    }

    /**
     * The worker registered successfully
     */
    get asV100(): {worker: Uint8Array, force: boolean} {
        assert(this.isV100)
        return this._chain.decodeEvent(this.event)
    }
}

export class OffchainComputingWorkersHeartbeatReceivedEvent {
    private readonly _chain: Chain
    private readonly event: Event

    constructor(ctx: EventContext)
    constructor(ctx: ChainContext, event: Event)
    constructor(ctx: EventContext, event?: Event) {
        event = event || ctx.event
        assert(event.name === 'OffchainComputingWorkers.HeartbeatReceived')
        this._chain = ctx._chain
        this.event = event
    }

    /**
     * The worker send heartbeat successfully
     */
    get isV100(): boolean {
        return this._chain.getEventHash('OffchainComputingWorkers.HeartbeatReceived') === 'f11393175fd830fdd5a7bfa56c34339d09170d98a007cffc7f8fcc4ecef9db43'
    }

    /**
     * The worker send heartbeat successfully
     */
    get asV100(): {worker: Uint8Array, nextHeartbeat: number} {
        assert(this.isV100)
        return this._chain.decodeEvent(this.event)
    }
}

export class OffchainComputingWorkersOfflineEvent {
    private readonly _chain: Chain
    private readonly event: Event

    constructor(ctx: EventContext)
    constructor(ctx: ChainContext, event: Event)
    constructor(ctx: EventContext, event?: Event) {
        event = event || ctx.event
        assert(event.name === 'OffchainComputingWorkers.Offline')
        this._chain = ctx._chain
        this.event = event
    }

    /**
     * The worker is offline
     */
    get isV100(): boolean {
        return this._chain.getEventHash('OffchainComputingWorkers.Offline') === 'c1c5611ea814a3d72ed198816b9911050b165c18f514a76ca026979da4321bfb'
    }

    /**
     * The worker is offline
     */
    get asV100(): {worker: Uint8Array, reason: v100.OfflineReason} {
        assert(this.isV100)
        return this._chain.decodeEvent(this.event)
    }
}

export class OffchainComputingWorkersOnlineEvent {
    private readonly _chain: Chain
    private readonly event: Event

    constructor(ctx: EventContext)
    constructor(ctx: ChainContext, event: Event)
    constructor(ctx: EventContext, event?: Event) {
        event = event || ctx.event
        assert(event.name === 'OffchainComputingWorkers.Online')
        this._chain = ctx._chain
        this.event = event
    }

    /**
     * The worker is online
     */
    get isV100(): boolean {
        return this._chain.getEventHash('OffchainComputingWorkers.Online') === 'f3194ebec04a39936489a690a526e8e923655c06012aa1b4ea84e17dfcf77c83'
    }

    /**
     * The worker is online
     */
    get asV100(): {worker: Uint8Array, implName: Uint8Array, implVersion: number, attestationMethod: (v100.AttestationMethod | undefined), nextHeartbeat: number} {
        assert(this.isV100)
        return this._chain.decodeEvent(this.event)
    }
}

export class OffchainComputingWorkersRegisteredEvent {
    private readonly _chain: Chain
    private readonly event: Event

    constructor(ctx: EventContext)
    constructor(ctx: ChainContext, event: Event)
    constructor(ctx: EventContext, event?: Event) {
        event = event || ctx.event
        assert(event.name === 'OffchainComputingWorkers.Registered')
        this._chain = ctx._chain
        this.event = event
    }

    /**
     * The worker registered successfully
     */
    get isV100(): boolean {
        return this._chain.getEventHash('OffchainComputingWorkers.Registered') === 'dbc381acde3e58a1dc63279d84f3fde21af33601d2a5e26e6755db85003b8aff'
    }

    /**
     * The worker registered successfully
     */
    get asV100(): {worker: Uint8Array, owner: Uint8Array} {
        assert(this.isV100)
        return this._chain.decodeEvent(this.event)
    }
}

export class OffchainComputingWorkersRequestingOfflineEvent {
    private readonly _chain: Chain
    private readonly event: Event

    constructor(ctx: EventContext)
    constructor(ctx: ChainContext, event: Event)
    constructor(ctx: EventContext, event?: Event) {
        event = event || ctx.event
        assert(event.name === 'OffchainComputingWorkers.RequestingOffline')
        this._chain = ctx._chain
        this.event = event
    }

    /**
     * The worker is requesting offline
     */
    get isV100(): boolean {
        return this._chain.getEventHash('OffchainComputingWorkers.RequestingOffline') === '92a756e29fcd5c189000de949169e0c1ab6089e641fedb90b510a59a4b108e28'
    }

    /**
     * The worker is requesting offline
     */
    get asV100(): {worker: Uint8Array} {
        assert(this.isV100)
        return this._chain.decodeEvent(this.event)
    }
}

export class OffchainComputingWorkersWorkerImplHashRemovedEvent {
    private readonly _chain: Chain
    private readonly event: Event

    constructor(ctx: EventContext)
    constructor(ctx: ChainContext, event: Event)
    constructor(ctx: EventContext, event?: Event) {
        event = event || ctx.event
        assert(event.name === 'OffchainComputingWorkers.WorkerImplHashRemoved')
        this._chain = ctx._chain
        this.event = event
    }

    /**
     * Remove worker's implementation permission successfully
     */
    get isV100(): boolean {
        return this._chain.getEventHash('OffchainComputingWorkers.WorkerImplHashRemoved') === 'aacc6cb873b554eef6a2d4170e4a4aaa17ca6a5dd178671082c88d46b2479a8b'
    }

    /**
     * Remove worker's implementation permission successfully
     */
    get asV100(): {implName: Uint8Array, implVersion: number} {
        assert(this.isV100)
        return this._chain.decodeEvent(this.event)
    }
}

export class OffchainComputingWorkersWorkerImplHashUpdatedEvent {
    private readonly _chain: Chain
    private readonly event: Event

    constructor(ctx: EventContext)
    constructor(ctx: ChainContext, event: Event)
    constructor(ctx: EventContext, event?: Event) {
        event = event || ctx.event
        assert(event.name === 'OffchainComputingWorkers.WorkerImplHashUpdated')
        this._chain = ctx._chain
        this.event = event
    }

    /**
     * Update worker's implementation permission successfully
     */
    get isV100(): boolean {
        return this._chain.getEventHash('OffchainComputingWorkers.WorkerImplHashUpdated') === 'aacc6cb873b554eef6a2d4170e4a4aaa17ca6a5dd178671082c88d46b2479a8b'
    }

    /**
     * Update worker's implementation permission successfully
     */
    get asV100(): {implName: Uint8Array, implVersion: number} {
        assert(this.isV100)
        return this._chain.decodeEvent(this.event)
    }
}

export class OffchainComputingWorkersWorkerImplPermissionRemovedEvent {
    private readonly _chain: Chain
    private readonly event: Event

    constructor(ctx: EventContext)
    constructor(ctx: ChainContext, event: Event)
    constructor(ctx: EventContext, event?: Event) {
        event = event || ctx.event
        assert(event.name === 'OffchainComputingWorkers.WorkerImplPermissionRemoved')
        this._chain = ctx._chain
        this.event = event
    }

    /**
     * Remove worker's implementation permission successfully
     */
    get isV100(): boolean {
        return this._chain.getEventHash('OffchainComputingWorkers.WorkerImplPermissionRemoved') === '83b3f8608eac046b157a62bec7a90097b62ab6ff67ceb1b266268febf507fd8f'
    }

    /**
     * Remove worker's implementation permission successfully
     */
    get asV100(): {implName: Uint8Array} {
        assert(this.isV100)
        return this._chain.decodeEvent(this.event)
    }
}

export class OffchainComputingWorkersWorkerImplPermissionUpdatedEvent {
    private readonly _chain: Chain
    private readonly event: Event

    constructor(ctx: EventContext)
    constructor(ctx: ChainContext, event: Event)
    constructor(ctx: EventContext, event?: Event) {
        event = event || ctx.event
        assert(event.name === 'OffchainComputingWorkers.WorkerImplPermissionUpdated')
        this._chain = ctx._chain
        this.event = event
    }

    /**
     * Update worker's implementation permission successfully
     */
    get isV100(): boolean {
        return this._chain.getEventHash('OffchainComputingWorkers.WorkerImplPermissionUpdated') === '83b3f8608eac046b157a62bec7a90097b62ab6ff67ceb1b266268febf507fd8f'
    }

    /**
     * Update worker's implementation permission successfully
     */
    get asV100(): {implName: Uint8Array} {
        assert(this.isV100)
        return this._chain.decodeEvent(this.event)
    }
}
