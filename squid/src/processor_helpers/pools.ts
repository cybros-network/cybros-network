import type { Context } from "../processor"
import {
    OffchainComputingPoolPoolCreatedEvent as PoolCreatedEvent,
    OffchainComputingPoolPoolDestroyedEvent as PoolDestroyedEvent,
    OffchainComputingPoolPoolMetadataUpdatedEvent as PoolMetadataUpdatedEvent,
    OffchainComputingPoolPoolMetadataRemovedEvent as PoolMetadataRemovedEvent,
    OffchainComputingPoolPoolSettingsUpdatedEvent as PoolSettingsUpdatedEvent,
} from "../types/events"
import { decodeSS58Address } from "../utils";
import assert from "assert";

interface PoolChanges {
    readonly id: string
    readonly poolId: number

    owner?: string
    implId?: number

    minImplSpecVersion?: number,
    maxImplSpecVersion?: number,
    createJobEnabled?: boolean
    autoDestroyProcessedJobEnabled?: boolean
    metadata?: Uint8Array | null

    createdAt: Date
    updatedAt: Date
    deletedAt?: Date | null
}

export function preprocessPoolsEvents(ctx: Context): Map<string, PoolChanges> {
    const changeSet = new Map<string, PoolChanges>();

    for (let block of ctx.blocks) {
        assert(block.header.timestamp)
        const blockTime = new Date(block.header.timestamp);

        for (let event of block.events) {
            if (event.name == "OffchainComputingPool.PoolCreated") {
                let e = new PoolCreatedEvent(ctx, event)
                let rec: {
                    owner: Uint8Array,
                    poolId: number,
                    implId: number,
                    createJobEnabled: boolean,
                    autoDestroyProcessedJobEnabled: boolean
                }
                if (e.isV100) {
                    rec = e.asV100
                } else {
                    throw new Error("Unsupported spec")
                }

                const id = rec.poolId.toString()
                const changes: PoolChanges = changeSet.get(id) || {
                    id,
                    poolId: rec.poolId,
                    createdAt: blockTime,
                    updatedAt: blockTime
                }

                changes.owner = decodeSS58Address(rec.owner)
                changes.implId = rec.poolId
                changes.minImplSpecVersion = 1
                changes.maxImplSpecVersion = 1
                changes.createJobEnabled = rec.createJobEnabled
                changes.autoDestroyProcessedJobEnabled = rec.autoDestroyProcessedJobEnabled
                changes.metadata = null

                changes.deletedAt = null
                changes.updatedAt = blockTime

                changeSet.set(id, changes)
            } else if (event.name == "OffchainComputingPool.PoolDestroyed") {
                let e = new PoolDestroyedEvent(ctx, event)
                let rec: { poolId: number }
                if (e.isV100) {
                    rec = e.asV100
                } else {
                    throw new Error('Unsupported spec')
                }

                const id = rec.poolId.toString()
                const changes: PoolChanges = changeSet.get(id) || {
                    id,
                    poolId: rec.poolId,
                    createdAt: blockTime,
                    updatedAt: blockTime
                }

                changes.deletedAt = blockTime
                changes.updatedAt = blockTime

                changeSet.set(id, changes)
            } else if (event.name == "OffchainComputingPool.PoolMetadataUpdated") {
                let e = new PoolMetadataUpdatedEvent(ctx, event)
                let rec: { poolId: number, metadata: Uint8Array }
                if (e.isV100) {
                    rec = e.asV100
                } else {
                    throw new Error('Unsupported spec')
                }

                const id = rec.poolId.toString()
                const changes: PoolChanges = changeSet.get(id) || {
                    id,
                    poolId: rec.poolId,
                    createdAt: blockTime,
                    updatedAt: blockTime
                }
                assert(!changes.deletedAt)

                changes.metadata = rec.metadata
                changes.updatedAt = blockTime

                changeSet.set(id, changes)
            } else if (event.name == "OffchainComputingPool.PoolMetadataRemoved") {
                let e = new PoolMetadataRemovedEvent(ctx, event)
                let rec: { poolId: number }
                if (e.isV100) {
                    rec = e.asV100
                } else {
                    throw new Error('Unsupported spec')
                }

                const id = rec.poolId.toString()
                const changes: PoolChanges = changeSet.get(id) || {
                    id,
                    poolId: rec.poolId,
                    createdAt: blockTime,
                    updatedAt: blockTime
                }
                assert(!changes.deletedAt)

                changes.metadata = null
                changes.updatedAt = blockTime

                changeSet.set(id, changes)
            } else if (event.name == "OffchainComputingPool.PoolSettingsUpdated") {
                let e = new PoolSettingsUpdatedEvent(ctx, event)
                let rec: {
                    poolId: number,
                    minImplSpecVersion: number,
                    maxImplSpecVersion: number,
                    createJobEnabled: boolean,
                    autoDestroyProcessedJobEnabled: boolean
                }
                if (e.isV100) {
                    rec = e.asV100
                } else {
                    throw new Error('Unsupported spec')
                }

                const id = rec.poolId.toString()
                const changes: PoolChanges = changeSet.get(id) || {
                    id,
                    poolId: rec.poolId,
                    createdAt: blockTime,
                    updatedAt: blockTime
                }
                assert(!changes.deletedAt)

                changes.minImplSpecVersion = rec.minImplSpecVersion
                changes.maxImplSpecVersion = rec.maxImplSpecVersion
                changes.createJobEnabled = rec.createJobEnabled
                changes.autoDestroyProcessedJobEnabled = rec.autoDestroyProcessedJobEnabled
                changes.updatedAt = blockTime

                changeSet.set(id, changes)
            }
        }
    }

    return changeSet
}
