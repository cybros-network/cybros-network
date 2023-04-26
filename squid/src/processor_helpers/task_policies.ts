import { type Context } from "../processor"
import {
    OffchainComputingTaskPolicyCreatedEvent as TaskPolicyCreatedEvent,
    OffchainComputingTaskPolicyDestroyedEvent as TaskPolicyDestroyedEvent,
    OffchainComputingTaskPolicyAvailabilityUpdatedEvent as TaskPolicyAvailabilityUpdatedEvent,
} from "../types/events"
import * as v100 from "../types/v100"
import {ApplicableScope} from "../model";
import assert from "assert";

function decodeScope(scope?: v100.ApplicableScope): ApplicableScope {
    if (!scope) {
        throw new Error("Unexpected undefined scope")
    }

    const kind = scope.__kind
    switch (kind) {
        case "Owner":
            return ApplicableScope.Owner
        case "Public":
            return ApplicableScope.Public
        default:
            throw new Error(`Unrecognized scope ${kind}`)
    }
}

interface TaskPolicyChanges {
    readonly id: string
    readonly poolId: number
    readonly policyId: number

    availability?: boolean
    creatingTaskScope?: ApplicableScope
    startBlock?: number
    endBlock?: number

    createdAt: Date
    updatedAt: Date
    deletedAt?: Date | null
}

export function preprocessTaskPoliciesEvents(ctx: Context): Map<string, TaskPolicyChanges> {
    const changeSet= new Map<string, TaskPolicyChanges>();

    for (let block of ctx.blocks) {
        const blockTime = new Date(block.header.timestamp);

        for (let item of block.items) {
            if (item.name == "OffchainComputing.TaskPolicyCreated") {
                let e = new TaskPolicyCreatedEvent(ctx, item.event)
                let rec: {
                    poolId: number,
                    policyId: number,
                    creatingTaskScope: v100.ApplicableScope,
                    startBlock: (number | undefined),
                    endBlock: (number | undefined)
                }
                if (e.isV100) {
                    rec = e.asV100
                } else {
                    throw new Error("Unsupported spec")
                }

                const id = `${rec.poolId}-${rec.policyId}`
                const changes: TaskPolicyChanges = changeSet.get(id) || {
                    id,
                    poolId: rec.poolId,
                    policyId: rec.policyId,
                    createdAt: blockTime,
                    updatedAt: blockTime
                }

                changes.availability = true
                changes.creatingTaskScope = decodeScope(rec.creatingTaskScope)
                changes.startBlock = rec.startBlock
                changes.endBlock = rec.endBlock

                changes.deletedAt = null
                changes.updatedAt = blockTime

                changeSet.set(id, changes)
            } else if (item.name == "OffchainComputing.TaskPolicyDestroyed") {
                let e = new TaskPolicyDestroyedEvent(ctx, item.event)
                let rec: {poolId: number, policyId: number}
                if (e.isV100) {
                    rec = e.asV100
                } else {
                    throw new Error('Unsupported spec')
                }

                const id = `${rec.poolId}-${rec.policyId}`
                const changes: TaskPolicyChanges = changeSet.get(id) || {
                    id,
                    poolId: rec.poolId,
                    policyId: rec.policyId,
                    createdAt: blockTime,
                    updatedAt: blockTime
                }

                changes.availability = false
                changes.deletedAt = blockTime
                changes.updatedAt = blockTime

                changeSet.set(id, changes)
            } else if (item.name == "OffchainComputing.TaskPolicyAvailabilityUpdated") {
                let e = new TaskPolicyAvailabilityUpdatedEvent(ctx, item.event)
                let rec: {poolId: number, policyId: number, availability: boolean}
                if (e.isV100) {
                    rec = e.asV100
                } else {
                    throw new Error('Unsupported spec')
                }

                const id = `${rec.poolId}-${rec.policyId}`
                const changes: TaskPolicyChanges = changeSet.get(id) || {
                    id,
                    poolId: rec.poolId,
                    policyId: rec.policyId,
                    createdAt: blockTime,
                    updatedAt: blockTime
                }
                assert(!changes.deletedAt)

                changes.availability = rec.availability
                changes.updatedAt = blockTime

                changeSet.set(id, changes)
            }
        }
    }

    return changeSet
}
