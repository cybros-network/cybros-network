import { type Context } from "../processor"
import {
    OffchainComputingWorkersWorkerRegisteredEvent as WorkerRegisteredEvent,
    OffchainComputingWorkersWorkerDeregisteredEvent as WorkerDeregisteredEvent,
    OffchainComputingWorkersWorkerAttestationRefreshedEvent as WorkerAttestationRefreshedEvent,
    OffchainComputingWorkersWorkerOnlineEvent as WorkerOnlineEvent,
    OffchainComputingWorkersWorkerRequestingOfflineEvent as WorkerRequestingOfflineEvent,
    OffchainComputingWorkersWorkerOfflineEvent as WorkerOfflineEvent,
    OffchainComputingWorkersWorkerHeartbeatReceivedEvent as WorkerHeartbeatReceivedEvent,
} from "../types/events"
import * as v100 from "../types/v100"
import { AttestationMethod, OfflineReason, WorkerStatus } from "../model"
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

function decodeOfflineReason(offlineReason?: v100.OfflineReason): OfflineReason {
    if (!offlineReason) {
        throw new Error("Unexpected undefined offline reason")
    }

    const kind = offlineReason.__kind
    switch (kind) {
        case "Graceful":
            return OfflineReason.Graceful
        case "Forced":
            return OfflineReason.Forced
        case "Unresponsive":
            return OfflineReason.Unresponsive
        case "AttestationExpired":
            return OfflineReason.AttestationExpired
        case "ImplBlocked":
            return OfflineReason.ImplBlocked
        case "InsufficientDepositFunds":
            return OfflineReason.InsufficientDepositFunds
        case "Other":
            return OfflineReason.Other
        default:
            throw new Error(`Unrecognized offline reason ${kind}`)
    }
}

interface WorkerChanges {
    readonly id: string

    owner?: string
    implId?: number

    status?: WorkerStatus
    implSpecVersion?: number
    implBuildVersion?: number
    attestationMethod?: AttestationMethod
    attestationExpiresAt?: Date | null
    lastAttestedAt?: Date
    lastHeartbeatReceivedAt?: Date
    offlineAt?: Date
    offlineReason?: OfflineReason

    createdAt?: Date
    updatedAt: Date
    deletedAt?: Date
}

export function preprocessWorkersEvents(ctx: Context): Map<string, WorkerChanges> {
    const changeSet= new Map<string, WorkerChanges>();

    for (let block of ctx.blocks) {
        const blockTime = new Date(block.header.timestamp);

        for (let item of block.items) {
            if (item.name == "OffchainComputingWorkers.WorkerRegistered") {
                let e = new WorkerRegisteredEvent(ctx, item.event)
                let rec: { worker: Uint8Array, owner: Uint8Array }
                if (e.isV100) {
                    rec = e.asV100
                } else {
                    throw new Error("Unsupported spec")
                }

                const id = decodeSS58Address(rec.worker)
                const changes: WorkerChanges = {
                    id,
                    owner: decodeSS58Address(rec.owner),
                    status: WorkerStatus.Registered,
                    createdAt: blockTime,
                    updatedAt: blockTime,
                }

                changeSet.set(id, changes)
            } else if (item.name == "OffchainComputingWorkers.WorkerDeregistered") {
                let e = new WorkerDeregisteredEvent(ctx, item.event)
                let rec: { worker: Uint8Array, force: boolean }
                if (e.isV100) {
                    rec = e.asV100
                } else {
                    throw new Error('Unsupported spec')
                }

                const id = decodeSS58Address(rec.worker)
                let changes: WorkerChanges = changeSet.get(id) || {
                    id,
                    updatedAt: blockTime,
                }
                assert(!changes.deletedAt)

                changes.updatedAt = blockTime
                changes.deletedAt = blockTime

                changeSet.set(id, changes)
            } else if (item.name == "OffchainComputingWorkers.WorkerOnline") {
                let e = new WorkerOnlineEvent(ctx, item.event)
                let rec: {
                    worker: Uint8Array,
                    implId: number,
                    implSpecVersion: number,
                    implBuildVersion: number,
                    attestationMethod: v100.AttestationMethod,
                    attestationExpiresAt: (bigint | undefined),
                    nextHeartbeat: number
                }
                if (e.isV100) {
                    rec = e.asV100
                } else {
                    throw new Error('Unsupported spec')
                }

                const id = decodeSS58Address(rec.worker)
                let changes: WorkerChanges = changeSet.get(id) || {
                    id,
                    updatedAt: blockTime,
                }
                assert(!changes.deletedAt)

                changes.status = WorkerStatus.Online
                changes.implId = rec.implId
                changes.implSpecVersion = rec.implSpecVersion
                changes.implBuildVersion = rec.implBuildVersion
                changes.attestationMethod = decodeAttestationMethod(rec.attestationMethod)
                changes.attestationExpiresAt = rec.attestationExpiresAt ? new Date(Number(rec.attestationExpiresAt)) : undefined
                changes.lastAttestedAt = blockTime
                changes.updatedAt = blockTime

                changeSet.set(id, changes)
            } else if (item.name == "OffchainComputingWorkers.WorkerRequestingOffline") {
                let e = new WorkerRequestingOfflineEvent(ctx, item.event)
                let rec: { worker: Uint8Array }
                if (e.isV100) {
                    rec = e.asV100
                } else {
                    throw new Error('Unsupported spec')
                }

                const id = decodeSS58Address(rec.worker)
                let changes: WorkerChanges = changeSet.get(id) || {
                    id,
                    updatedAt: blockTime,
                }
                assert(!changes.deletedAt)

                changes.status = WorkerStatus.RequestingOffline
                changes.updatedAt = blockTime

                changeSet.set(id, changes)
            } else if (item.name == "OffchainComputingWorkers.WorkerOffline") {
                let e = new WorkerOfflineEvent(ctx, item.event)
                let rec: { worker: Uint8Array, reason: v100.OfflineReason }
                if (e.isV100) {
                    rec = e.asV100
                } else {
                    throw new Error('Unsupported spec')
                }

                const id = decodeSS58Address(rec.worker)
                let changes: WorkerChanges = changeSet.get(id) || {
                    id,
                    updatedAt: blockTime,
                }
                assert(!changes.deletedAt)

                changes.status = WorkerStatus.Offline
                changes.offlineReason = decodeOfflineReason(rec.reason)
                changes.offlineAt = blockTime
                changes.updatedAt = blockTime

                changeSet.set(id, changes)
            } else if (item.name == "OffchainComputingWorkers.WorkerHeartbeatReceived") {
                let e = new WorkerHeartbeatReceivedEvent(ctx, item.event)
                let rec: { worker: Uint8Array, next: number }
                if (e.isV100) {
                    rec = e.asV100
                } else {
                    throw new Error('Unsupported spec')
                }

                const id = decodeSS58Address(rec.worker)
                let changes: WorkerChanges = changeSet.get(id) || {
                    id,
                    updatedAt: blockTime,
                }
                assert(!changes.deletedAt)

                changes.lastHeartbeatReceivedAt = blockTime
                changes.updatedAt = blockTime

                changeSet.set(id, changes)
            } else if (item.name == "OffchainComputingWorkers.WorkerAttestationRefreshed") {
                let e = new WorkerAttestationRefreshedEvent(ctx, item.event)
                let rec: { worker: Uint8Array, expiresAt: (bigint | undefined) }
                if (e.isV100) {
                    rec = e.asV100
                } else {
                    throw new Error('Unsupported spec')
                }

                const id = decodeSS58Address(rec.worker)
                let changes: WorkerChanges = changeSet.get(id) || {
                    id,
                    updatedAt: blockTime,
                }
                assert(!changes.deletedAt)

                changes.attestationExpiresAt = rec.expiresAt ? new Date(Number(rec.expiresAt)) : null
                changes.lastAttestedAt = blockTime
                changes.updatedAt = blockTime

                changeSet.set(id, changes)
            }
        }
    }

    return changeSet
}
