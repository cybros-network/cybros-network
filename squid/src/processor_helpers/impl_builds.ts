import {type Context} from "../processor"
import {
    OffchainComputingWorkersImplBuildDeregisteredEvent as ImplBuildDeregisteredEvent,
    OffchainComputingWorkersImplBuildRegisteredEvent as ImplBuildRegisteredEvent,
    OffchainComputingWorkersImplBuildStatusUpdatedEvent as ImplBuildStatusUpdatedEvent,
} from "../types/events"
import * as v100 from "../types/v100"
import {ImplBuildStatus} from "../model"
import {u8aToHex} from "../utils"
import assert from "assert";

function decodeImplBuildStatus(implBuildStatus?: v100.ImplBuildStatus): ImplBuildStatus {
    if (!implBuildStatus) {
        throw new Error("Unexpected undefined impl build status")
    }

    const kind = implBuildStatus.__kind
    switch (kind) {
        case "Released":
            return ImplBuildStatus.Released
        case "Deprecated":
            return ImplBuildStatus.Deprecated
        case "Blocked":
            return ImplBuildStatus.Blocked
        default:
            throw new Error(`Unrecognized impl build status ${kind}`)
    }
}

interface ImplBuildChanges {
    readonly id: string
    readonly implId: number

    readonly version: number
    magicBytes?: string | null
    status: ImplBuildStatus

    createdAt: Date
    updatedAt: Date
    deletedAt?: Date | null
}

export function preprocessImplBuildsEvents(ctx: Context): Map<string, ImplBuildChanges> {
    const changeSet= new Map<string, ImplBuildChanges>();

    for (let block of ctx.blocks) {
        const blockTime = new Date(block.header.timestamp);

        for (let item of block.items) {
            if (item.name == "OffchainComputingWorkers.ImplBuildRegistered") {
                let e = new ImplBuildRegisteredEvent(ctx, item.event)
                let rec: {
                    implId: number,
                    implBuildVersion: number,
                    magicBytes: (Uint8Array | undefined)
                }
                if (e.isV100) {
                    rec = e.asV100
                } else {
                    throw new Error("Unsupported spec")
                }

                const id = `${rec.implId}-${rec.implBuildVersion}`
                const changes: ImplBuildChanges = changeSet.get(id) || {
                    id,
                    implId: rec.implId,
                    version: rec.implBuildVersion,
                    status: ImplBuildStatus.Released,
                    createdAt: blockTime,
                    updatedAt: blockTime
                }

                changes.status = ImplBuildStatus.Released
                changes.magicBytes = rec.magicBytes ? u8aToHex(rec.magicBytes) : null

                changes.deletedAt = null
                changes.updatedAt = blockTime

                changeSet.set(id, changes)
            } else if (item.name == "OffchainComputingWorkers.ImplBuildDeregistered") {
                let e = new ImplBuildDeregisteredEvent(ctx, item.event)
                let rec: { implId: number, implBuildVersion: number }
                if (e.isV100) {
                    rec = e.asV100
                } else {
                    throw new Error('Unsupported spec')
                }

                const id = `${rec.implId}-${rec.implBuildVersion}`
                const changes: ImplBuildChanges = changeSet.get(id) || {
                    id,
                    implId: rec.implId,
                    version: rec.implBuildVersion,
                    status: ImplBuildStatus.Deregistered,
                    createdAt: blockTime,
                    updatedAt: blockTime
                }

                changes.status = ImplBuildStatus.Deregistered
                changes.deletedAt = blockTime
                changes.updatedAt = blockTime

                changeSet.set(id, changes)
            } else if (item.name == "OffchainComputingWorkers.ImplBuildStatusUpdated") {
                let e = new ImplBuildStatusUpdatedEvent(ctx, item.event)
                let rec: { implId: number, implBuildVersion: number, status: v100.ImplBuildStatus }
                if (e.isV100) {
                    rec = e.asV100
                } else {
                    throw new Error('Unsupported spec')
                }

                const id = `${rec.implId}-${rec.implBuildVersion}`
                const changes: ImplBuildChanges = changeSet.get(id) || {
                    id,
                    implId: rec.implId,
                    version: rec.implBuildVersion,
                    status: decodeImplBuildStatus(rec.status),
                    createdAt: blockTime,
                    updatedAt: blockTime
                }
                assert(!changes.deletedAt)

                changes.status = decodeImplBuildStatus(rec.status)

                changes.updatedAt = blockTime

                changeSet.set(id, changes)
            }
        }
    }

    return changeSet
}
