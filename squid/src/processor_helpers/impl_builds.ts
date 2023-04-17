import { type Context } from "../processor"
import {
    OffchainComputingWorkersImplBuildRegisteredEvent as ImplBuildRegisteredEvent,
    OffchainComputingWorkersImplBuildDeregisteredEvent as ImplBuildDeregisteredEvent,
} from "../types/events"
import { u8aToHex } from "../utils"

interface ImplBuildChanges {
    readonly id: string
    readonly implId: number

    version: number
    magicBytes?: string

    createdAt?: Date
    deletedAt?: Date
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
                    version: number,
                    magicBytes: Uint8Array
                }
                if (e.isV100) {
                    rec = e.asV100
                } else {
                    throw new Error("Unsupported spec")
                }

                const id = `${rec.implId}-${rec.version}`
                const changes: ImplBuildChanges = {
                    id,
                    implId: rec.implId,
                    version: rec.version,
                    magicBytes: u8aToHex(rec.magicBytes),
                    createdAt: blockTime
                }

                changeSet.set(id, changes)
            } else if (item.name == "OffchainComputingWorkers.ImplBuildDeregistered") {
                let e = new ImplBuildDeregisteredEvent(ctx, item.event)
                let rec: { implId: number, version: number }
                if (e.isV100) {
                    rec = e.asV100
                } else {
                    throw new Error('Unsupported spec')
                }

                const id = `${rec.implId}-${rec.version}`
                const changes: ImplBuildChanges = {
                    id,
                    implId: rec.implId,
                    version: rec.version,
                    deletedAt: blockTime
                }

                changeSet.set(id, changes)
            }
        }
    }

    return changeSet
}
