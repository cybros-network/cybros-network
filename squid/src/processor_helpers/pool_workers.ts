import { type Context } from "../processor"
import {
    OffchainComputingWorkerAddedEvent as WorkerAddedEvent,
    OffchainComputingWorkerRemovedEvent as WorkerRemovedEvent,
} from "../types/events"
import { decodeSS58Address } from "../utils"

interface PoolWorkerChanges {
    readonly id: string
    readonly poolId: number
    readonly worker: string

    createdAt?: Date
    deletedAt?: Date
}

export function preprocessPoolWorkersEvents(ctx: Context): Map<string, PoolWorkerChanges> {
    const changeSet= new Map<string, PoolWorkerChanges>();

    for (let block of ctx.blocks) {
        const blockTime = new Date(block.header.timestamp);

        for (let item of block.items) {
            if (item.name == "OffchainComputing.WorkerAdded") {
                let e = new WorkerAddedEvent(ctx, item.event)
                let rec: { poolId: number, worker: Uint8Array }
                if (e.isV100) {
                    rec = e.asV100
                } else {
                    throw new Error("Unsupported spec")
                }

                const worker = decodeSS58Address(rec.worker)
                const id = `${rec.poolId}-${rec.worker}`
                const changes: PoolWorkerChanges = {
                    id,
                    poolId: rec.poolId,
                    worker,
                    createdAt: blockTime
                }

                changeSet.set(id, changes)
            } else if (item.name == "OffchainComputing.WorkerRemoved") {
                let e = new WorkerRemovedEvent(ctx, item.event)
                let rec: { poolId: number, worker: Uint8Array }
                if (e.isV100) {
                    rec = e.asV100
                } else {
                    throw new Error('Unsupported spec')
                }

                const worker = decodeSS58Address(rec.worker)
                const id = `${rec.poolId}-${rec.worker}`
                const changes: PoolWorkerChanges = {
                    id,
                    poolId: rec.poolId,
                    worker,
                    deletedAt: blockTime
                }

                changeSet.set(id, changes)
            }
        }
    }

    return changeSet
}
