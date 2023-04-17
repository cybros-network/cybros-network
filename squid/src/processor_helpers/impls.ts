import { type Context } from "../processor"
import {
    OffchainComputingWorkersImplRegisteredEvent as ImplRegisteredEvent,
    OffchainComputingWorkersImplDeregisteredEvent as ImplDeregisteredEvent,
    OffchainComputingWorkersImplDeploymentPermissionUpdatedEvent as ImplDeploymentPermissionUpdatedEvent,
    OffchainComputingWorkersImplMetadataUpdatedEvent as ImplMetadataUpdatedEvent,
    OffchainComputingWorkersImplMetadataRemovedEvent as ImplMetadataRemovedEvent,
    OffchainComputingWorkersImplBuildRestrictionUpdatedEvent as ImplBuildRestrictionUpdatedEvent,
} from "../types/events"
import * as v100 from "../types/v100"
import { AttestationMethod, ImplDeploymentPermission } from "../model"
import { decodeSS58Address, u8aToString } from "../utils"
import assert from "assert";

function decodeAttestationMethod(attestationMethod?: v100.AttestationMethod): AttestationMethod {
    if (!attestationMethod) {
        throw new Error("Unexpected undefined attestation method")
    }

    const kind = attestationMethod.__kind
    switch (kind) {
        case "OptOut":
            return AttestationMethod.OptOut
        default:
            throw new Error(`Unrecognized attestation method ${kind}`)
    }
}

function decodeImplDeploymentPermission(implDeploymentPermission?: v100.ImplDeploymentPermission): ImplDeploymentPermission {
    if (!implDeploymentPermission) {
        throw new Error("Unexpected undefined impl deployment permission")
    }

    const kind = implDeploymentPermission.__kind
    switch (kind) {
        case "Owner":
            return ImplDeploymentPermission.Owner
        case "Public":
            return ImplDeploymentPermission.Public
        default:
            throw new Error(`Unrecognized impl deployment permission ${kind}`)
    }
}

interface ImplChanges {
    readonly id: string
    readonly implId: number

    owner?: string

    attestationMethod?: AttestationMethod
    deploymentPermission?: ImplDeploymentPermission
    oldestBuildVersion?: number
    newestBuildVersion?: number
    blockedBuildVersions?: number[]
    metadata?: string | null

    createdAt?: Date
    updatedAt: Date
    deletedAt?: Date
}

export function preprocessImplsEvents(ctx: Context): Map<string, ImplChanges> {
    const changeSet= new Map<string, ImplChanges>();

    for (let block of ctx.blocks) {
        const blockTime = new Date(block.header.timestamp);

        for (let item of block.items) {
            if (item.name == "OffchainComputingWorkers.ImplRegistered") {
                let e = new ImplRegisteredEvent(ctx, item.event)
                let rec: {
                    implId: number,
                    owner: Uint8Array,
                    attestationMethod: v100.AttestationMethod,
                    deploymentPermission: v100.ImplDeploymentPermission
                }
                if (e.isV100) {
                    rec = e.asV100
                } else {
                    throw new Error("Unsupported spec")
                }

                const id = rec.implId.toString()
                const changes: ImplChanges = {
                    id,
                    implId: rec.implId,
                    owner: decodeSS58Address(rec.owner),
                    attestationMethod: decodeAttestationMethod(rec.attestationMethod),
                    deploymentPermission: decodeImplDeploymentPermission(rec.deploymentPermission),
                    oldestBuildVersion: 1,
                    newestBuildVersion: 1,
                    blockedBuildVersions: [],
                    createdAt: blockTime,
                    updatedAt: blockTime,
                }

                changeSet.set(id, changes)
            } else if (item.name == "OffchainComputingWorkers.ImplDeregistered") {
                let e = new ImplDeregisteredEvent(ctx, item.event)
                let rec: { implId: number }
                if (e.isV100) {
                    rec = e.asV100
                } else {
                    throw new Error('Unsupported spec')
                }

                const id = rec.implId.toString()
                let changes: ImplChanges = changeSet.get(id) || {
                    id,
                    implId: rec.implId,
                    updatedAt: blockTime,
                }
                changes.updatedAt = blockTime
                changes.deletedAt = blockTime

                changeSet.set(id, changes)
            } if (item.name == "OffchainComputingWorkers.ImplDeploymentPermissionUpdated") {
                let e = new ImplDeploymentPermissionUpdatedEvent(ctx, item.event)
                let rec: {
                    implId: number, permission: v100.ImplDeploymentPermission
                }
                if (e.isV100) {
                    rec = e.asV100
                } else {
                    throw new Error('Unsupported spec')
                }

                const id = rec.implId.toString()
                let changes: ImplChanges = changeSet.get(id) || {
                    id,
                    implId: rec.implId,
                    updatedAt: blockTime,
                }
                assert(!changes.deletedAt)

                changes.deploymentPermission = decodeImplDeploymentPermission(rec.permission)
                changes.updatedAt = blockTime

                changeSet.set(id, changes)
            } else if (item.name == "OffchainComputingWorkers.ImplMetadataUpdated") {
                let e = new ImplMetadataUpdatedEvent(ctx, item.event)
                let rec: { implId: number, metadata: Uint8Array }
                if (e.isV100) {
                    rec = e.asV100
                } else {
                    throw new Error('Unsupported spec')
                }

                const id = rec.implId.toString()
                let changes: ImplChanges = changeSet.get(id) || {
                    id,
                    implId: rec.implId,
                    updatedAt: blockTime,
                }
                assert(!changes.deletedAt)

                changes.metadata = u8aToString(rec.metadata)
                changes.updatedAt = blockTime

                changeSet.set(id, changes)
            } else if (item.name == "OffchainComputingWorkers.ImplMetadataRemoved") {
                let e = new ImplMetadataRemovedEvent(ctx, item.event)
                let rec: { implId: number }
                if (e.isV100) {
                    rec = e.asV100
                } else {
                    throw new Error('Unsupported spec')
                }

                const id = rec.implId.toString()
                let changes: ImplChanges = changeSet.get(id) || {
                    id,
                    implId: rec.implId,
                    updatedAt: blockTime,
                }
                assert(!changes.deletedAt)

                changes.metadata = null
                changes.updatedAt = blockTime

                changeSet.set(id, changes)
            } else if (item.name == "OffchainComputingWorkers.ImplBuildRestrictionUpdated") {
                let e = new ImplBuildRestrictionUpdatedEvent(ctx, item.event)
                let rec: { implId: number, restriction: v100.ImplBuildRestriction }
                if (e.isV100) {
                    rec = e.asV100
                } else {
                    throw new Error('Unsupported spec')
                }

                const id = rec.implId.toString()
                let changes: ImplChanges = changeSet.get(id) || {
                    id,
                    implId: rec.implId,
                    updatedAt: blockTime,
                }
                assert(!changes.deletedAt)

                changes.oldestBuildVersion = rec.restriction.oldest
                changes.newestBuildVersion = rec.restriction.newest
                changes.blockedBuildVersions = rec.restriction.blocked
                changes.updatedAt = blockTime

                changeSet.set(id, changes)
            }
        }
    }

    return changeSet
}
