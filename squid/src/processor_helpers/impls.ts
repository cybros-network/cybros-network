import type { Context } from "../processor"
import {
    OffchainComputingInfraImplRegisteredEvent as ImplRegisteredEvent,
    OffchainComputingInfraImplDeregisteredEvent as ImplDeregisteredEvent,
    OffchainComputingInfraImplDeploymentScopeUpdatedEvent as ImplDeploymentScopeUpdatedEvent,
    OffchainComputingInfraImplMetadataUpdatedEvent as ImplMetadataUpdatedEvent,
    OffchainComputingInfraImplMetadataRemovedEvent as ImplMetadataRemovedEvent,
} from "../types/events"
import * as v100 from "../types/v100"
import { AttestationMethod, ApplicableScope } from "../model"
import { decodeSS58Address } from "../utils"
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

interface ImplChanges {
    readonly id: string
    readonly implId: number

    owner?: string

    attestationMethod?: AttestationMethod
    deploymentScope?: ApplicableScope
    metadata?: Uint8Array | null

    createdAt: Date
    updatedAt: Date
    deletedAt?: Date | null
}

export function preprocessImplsEvents(ctx: Context): Map<string, ImplChanges> {
    const changeSet= new Map<string, ImplChanges>();

    for (let block of ctx.blocks) {
        assert(block.header.timestamp)
        const blockTime = new Date(block.header.timestamp);

        for (let event of block.events) {
            if (event.name == "OffchainComputingInfra.ImplRegistered") {
                let e = new ImplRegisteredEvent(ctx, event)
                let rec: {
                    implId: number,
                    owner: Uint8Array,
                    attestationMethod: v100.AttestationMethod,
                    deploymentScope: v100.ApplicableScope
                }
                if (e.isV100) {
                    rec = e.asV100
                } else {
                    throw new Error("Unsupported spec")
                }

                const id = rec.implId.toString()
                const changes: ImplChanges = changeSet.get(id) || {
                    id,
                    implId: rec.implId,
                    createdAt: blockTime,
                    updatedAt: blockTime
                }

                changes.owner = decodeSS58Address(rec.owner)
                changes.attestationMethod = decodeAttestationMethod(rec.attestationMethod)
                changes.deploymentScope = decodeScope(rec.deploymentScope)

                changes.deletedAt = null
                changes.updatedAt = blockTime

                changeSet.set(id, changes)
            } else if (event.name == "OffchainComputingInfra.ImplDeregistered") {
                let e = new ImplDeregisteredEvent(ctx, event)
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
                    createdAt: blockTime,
                    updatedAt: blockTime
                }
                changes.updatedAt = blockTime
                changes.deletedAt = blockTime

                changeSet.set(id, changes)
            } if (event.name == "OffchainComputingInfra.ImplDeploymentScopeUpdated") {
                let e = new ImplDeploymentScopeUpdatedEvent(ctx, event)
                let rec: {
                    implId: number, scope: v100.ApplicableScope
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
                    createdAt: blockTime,
                    updatedAt: blockTime
                }
                assert(!changes.deletedAt)

                changes.deploymentScope = decodeScope(rec.scope)
                changes.updatedAt = blockTime

                changeSet.set(id, changes)
            } else if (event.name == "OffchainComputingInfra.ImplMetadataUpdated") {
                let e = new ImplMetadataUpdatedEvent(ctx, event)
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
                    createdAt: blockTime,
                    updatedAt: blockTime
                }
                assert(!changes.deletedAt)

                changes.metadata = rec.metadata
                changes.updatedAt = blockTime

                changeSet.set(id, changes)
            } else if (event.name == "OffchainComputingInfra.ImplMetadataRemoved") {
                let e = new ImplMetadataRemovedEvent(ctx, event)
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
                    createdAt: blockTime,
                    updatedAt: blockTime
                }
                assert(!changes.deletedAt)

                changes.metadata = null
                changes.updatedAt = blockTime

                changeSet.set(id, changes)
            }
        }
    }

    return changeSet
}
