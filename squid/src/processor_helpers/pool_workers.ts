import { type Context } from "../processor"
import {
    OffchainComputingWorkerSubscribedEvent as WorkerSubscribedEvent,
    OffchainComputingWorkerUnsubscribedEvent as WorkerUnsubscribedEvent,
} from "../types/events"
import { decodeSS58Address } from "../utils"
import { WorkerEventKind } from "../model";

interface WorkerEvent {
    readonly id: string

    readonly kind: WorkerEventKind
    readonly payload?: any

    readonly blockNumber: number
    readonly blockTime: Date
}

interface PoolWorkerChanges {
    readonly id: string
    readonly poolId: number
    readonly worker: string

    createdAt: Date
    updatedAt: Date
    deletedAt?: Date | null

    poolWorkerCounterChange: number

    workerEvents: WorkerEvent[]
}

export function preprocessPoolWorkersEvents(ctx: Context): Map<string, PoolWorkerChanges> {
    const changeSet= new Map<string, PoolWorkerChanges>();

    for (let block of ctx.blocks) {
        const blockNumber = block.header.height
        const blockTime = new Date(block.header.timestamp);

        for (let item of block.items) {
            if (item.name == "OffchainComputing.WorkerSubscribed") {
                let e = new WorkerSubscribedEvent(ctx, item.event)
                let rec: {worker: Uint8Array, poolId: number}
                if (e.isV100) {
                    rec = e.asV100
                } else {
                    throw new Error("Unsupported spec")
                }

                const worker = decodeSS58Address(rec.worker)
                const id = `${rec.poolId}-${rec.worker}`
                const changes: PoolWorkerChanges = changeSet.get(id) || {
                    id,
                    poolId: rec.poolId,
                    worker,
                    createdAt: blockTime,
                    updatedAt: blockTime,
                    poolWorkerCounterChange: 0,
                    workerEvents: []
                }

                changes.deletedAt = null
                changes.updatedAt = blockTime
                changes.poolWorkerCounterChange = 1
                changes.workerEvents.push({
                    id: `${id}-${blockNumber}-${item.event.indexInBlock}`,
                    kind: WorkerEventKind.SubscribedPool,
                    payload: {poolId: rec.poolId},
                    blockNumber,
                    blockTime,
                })

                changeSet.set(id, changes)
            } else if (item.name == "OffchainComputing.WorkerUnsubscribed") {
                let e = new WorkerUnsubscribedEvent(ctx, item.event)
                let rec: {worker: Uint8Array, poolId: number}
                if (e.isV100) {
                    rec = e.asV100
                } else {
                    throw new Error('Unsupported spec')
                }

                const worker = decodeSS58Address(rec.worker)
                const id = `${rec.poolId}-${rec.worker}`
                const changes: PoolWorkerChanges = changeSet.get(id) || {
                    id,
                    poolId: rec.poolId,
                    worker,
                    createdAt: blockTime,
                    updatedAt: blockTime,
                    poolWorkerCounterChange: 0,
                    workerEvents: []
                }

                changes.deletedAt = blockTime
                changes.updatedAt = blockTime
                changes.poolWorkerCounterChange = -1
                changes.workerEvents.push({
                    id: `${id}-${blockNumber}-${item.event.indexInBlock}`,
                    kind: WorkerEventKind.UnsubscribedPool,
                    payload: {poolId: rec.poolId},
                    blockNumber,
                    blockTime,
                })

                changeSet.set(id, changes)
            }
        }
    }

    return changeSet
}
