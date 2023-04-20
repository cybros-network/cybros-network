import { type Context } from "../processor"
import {
    OffchainComputingCreatingTaskPolicyCreatedEvent as CreatingTaskPolicyCreatedEvent,
    OffchainComputingCreatingTaskPolicyDestroyedEvent as CreatingTaskPolicyDestroyedEvent,
} from "../types/events"
import * as v100 from "../types/v100"
import { CreatingTaskPermission } from "../model";

function decodeCreatingTaskPermission(creatingTaskPermission?: v100.CreatingTaskPermission): CreatingTaskPermission {
    if (!creatingTaskPermission) {
        throw new Error("Unexpected undefined creating task permission")
    }

    const kind = creatingTaskPermission.__kind
    switch (kind) {
        case "Owner":
            return CreatingTaskPermission.Owner
        case "Public":
            return CreatingTaskPermission.Public
        default:
            throw new Error(`Unrecognized creating task permission ${kind}`)
    }
}

interface CreatingTaskPolicyChanges {
    readonly id: string
    readonly poolId: number
    readonly policyId: number

    permission?: CreatingTaskPermission
    startBlock?: number
    endBlock?: number

    createdAt: Date
    updatedAt: Date
    deletedAt?: Date | null
}

export function preprocessCreatingTaskPoliciesEvents(ctx: Context): Map<string, CreatingTaskPolicyChanges> {
    const changeSet= new Map<string, CreatingTaskPolicyChanges>();

    for (let block of ctx.blocks) {
        const blockTime = new Date(block.header.timestamp);

        for (let item of block.items) {
            if (item.name == "OffchainComputing.CreatingTaskPolicyCreated") {
                let e = new CreatingTaskPolicyCreatedEvent(ctx, item.event)
                let rec: { poolId: number, policyId: number, policy: v100.CreatingTaskPolicy }
                if (e.isV100) {
                    rec = e.asV100
                } else {
                    throw new Error("Unsupported spec")
                }

                const id = `${rec.poolId}-${rec.policyId}`
                const changes: CreatingTaskPolicyChanges = changeSet.get(id) || {
                    id,
                    poolId: rec.poolId,
                    policyId: rec.policyId,
                    createdAt: blockTime,
                    updatedAt: blockTime
                }

                changes.permission = decodeCreatingTaskPermission(rec.policy.permission)
                changes.startBlock = rec.policy.startBlock
                changes.endBlock = rec.policy.endBlock

                changes.deletedAt = null
                changes.updatedAt = blockTime

                changeSet.set(id, changes)
            } else if (item.name == "OffchainComputing.CreatingTaskPolicyDestroyed") {
                let e = new CreatingTaskPolicyDestroyedEvent(ctx, item.event)
                let rec: { poolId: number, policyId: number }
                if (e.isV100) {
                    rec = e.asV100
                } else {
                    throw new Error('Unsupported spec')
                }

                const id = `${rec.poolId}-${rec.policyId}`
                const changes: CreatingTaskPolicyChanges = changeSet.get(id) || {
                    id,
                    poolId: rec.poolId,
                    policyId: rec.policyId,
                    createdAt: blockTime,
                    updatedAt: blockTime
                }

                changes.deletedAt = blockTime
                changes.updatedAt = blockTime

                changeSet.set(id, changes)
            }
        }
    }

    return changeSet
}
