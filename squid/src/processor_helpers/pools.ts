import { type Context } from "../processor"
import {
    OffchainComputingPoolCreatedEvent as PoolCreatedEvent,
    OffchainComputingPoolDestroyedEvent as PoolDestroyedEvent,
    OffchainComputingPoolMetadataUpdatedEvent as PoolMetadataUpdatedEvent,
    OffchainComputingPoolMetadataRemovedEvent as PoolMetadataRemovedEvent,
    OffchainComputingPoolCreatingTaskAbilityEnabledEvent as PoolCreatingTaskAbilityEnabledEvent,
    OffchainComputingPoolCreatingTaskAbilityDisabledEvent as PoolCreatingTaskAbilityDisabledEvent,
} from "../types/events"
import { decodeSS58Address, u8aToString } from "../utils";
import assert from "assert";

interface PoolChanges {
    readonly id: string
    readonly poolId: number

    owner?: string
    implId?: number

    creatingTaskAbility?: boolean
    metadata?: string | null

    createdAt: Date
    updatedAt: Date
    deletedAt?: Date | null
}

export function preprocessPoolsEvents(ctx: Context): Map<string, PoolChanges> {
    const changeSet = new Map<string, PoolChanges>();

    for (let block of ctx.blocks) {
        const blockTime = new Date(block.header.timestamp);

        for (let item of block.items) {
            if (item.name == "OffchainComputing.PoolCreated") {
                let e = new PoolCreatedEvent(ctx, item.event)
                let rec: { owner: Uint8Array, poolId: number, implId: number }
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
                changes.creatingTaskAbility = true
                changes.metadata = null

                changes.deletedAt = null
                changes.updatedAt = blockTime

                changeSet.set(id, changes)
            } else if (item.name == "OffchainComputing.PoolDestroyed") {
                let e = new PoolDestroyedEvent(ctx, item.event)
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
            } else if (item.name == "OffchainComputing.PoolMetadataUpdated") {
                let e = new PoolMetadataUpdatedEvent(ctx, item.event)
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

                changes.metadata = u8aToString(rec.metadata)
                changes.updatedAt = blockTime

                changeSet.set(id, changes)
            } else if (item.name == "OffchainComputing.PoolMetadataRemoved") {
                let e = new PoolMetadataRemovedEvent(ctx, item.event)
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
            } else if (item.name == "OffchainComputing.PoolCreatingTaskAbilityEnabled") {
                let e = new PoolCreatingTaskAbilityEnabledEvent(ctx, item.event)
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

                changes.creatingTaskAbility = true
                changes.updatedAt = blockTime

                changeSet.set(id, changes)
            } else if (item.name == "OffchainComputing.PoolCreatingTaskAbilityDisabled") {
                let e = new PoolCreatingTaskAbilityDisabledEvent(ctx, item.event)
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

                changes.creatingTaskAbility = false
                changes.updatedAt = blockTime

                changeSet.set(id, changes)
            }
        }
    }

    return changeSet
}
