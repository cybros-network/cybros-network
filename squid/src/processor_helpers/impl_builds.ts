import { type Context } from "../processor"
import {
    OffchainComputingWorkersImplBuildRegisteredEvent as ImplBuildRegisteredEvent,
    OffchainComputingWorkersImplBuildDeregisteredEvent as ImplBuildDeregisteredEvent,
} from "../types/events"
import { u8aToHex } from "../utils"

interface ImplBuildChanges {
    readonly id: string
    readonly implId: string

    version?: number
    magicBytes?: string

    createdAt?: Date
    deletedAt?: Date
}

export function preprocessImplBuildsEvents(ctx: Context): Map<string, ImplBuildChanges> {
    const implBuildsChangeSet= new Map<string, ImplBuildChanges>();

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
                const implBuildChanges: ImplBuildChanges = {
                    id,
                    implId: rec.implId.toString(),
                    version: rec.version,
                    magicBytes: u8aToHex(rec.magicBytes),
                    createdAt: blockTime
                }

                implBuildsChangeSet.set(id, implBuildChanges)
            } else if (item.name == "OffchainComputingWorkers.ImplBuildDeregistered") {
                let e = new ImplBuildDeregisteredEvent(ctx, item.event)
                let rec: { implId: number, version: number }
                if (e.isV100) {
                    rec = e.asV100
                } else {
                    throw new Error('Unsupported spec')
                }

                const id = `${rec.implId}-${rec.version}`
                const implBuildChanges: ImplBuildChanges = {
                    id,
                    implId: rec.implId.toString(),
                    version: rec.version,
                    deletedAt: blockTime
                }

                implBuildsChangeSet.set(id, implBuildChanges)
            }
        }
    }

    return implBuildsChangeSet
}
