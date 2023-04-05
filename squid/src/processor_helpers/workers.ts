import { type Context } from "../processor"
import {
    OffchainComputingWorkersAttestationRefreshedEvent as WorkerAttestationRefreshedEvent,
    OffchainComputingWorkersDeregisteredEvent as WorkerDeregisteredEvent,
    OffchainComputingWorkersHeartbeatReceivedEvent as WorkerHeartbeatReceivedEvent,
    OffchainComputingWorkersOfflineEvent as WorkerOfflineEvent,
    OffchainComputingWorkersOnlineEvent as WorkerOnlineEvent,
    OffchainComputingWorkersRegisteredEvent as WorkerRegisteredEvent,
    OffchainComputingWorkersRequestingOfflineEvent as WorkerRequestingOfflineEvent,
} from "../types/events"
import { AttestationMethod, OfflineReason, WorkerStatus } from "../model"
import * as v100 from "../types/v100"
import { decodeSS58Address, u8aToString } from "../utils"

// import { toHex } from "@subsquid/substrate-processor"

function decodeAttestationMethod(attestationMethod?: v100.AttestationMethod): AttestationMethod {
    if (!attestationMethod) {
        throw new Error("Unexpected undefined attestation method")
    }

    const kind = attestationMethod.__kind
    switch (kind) {
        case "NonTEE":
            return AttestationMethod.NoneTEE
        case "Root":
            return AttestationMethod.Root
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
        case "WorkerImplBlocked":
            return OfflineReason.WorkerImplBlocked
        case "InsufficientReservedFunds":
            return OfflineReason.InsufficientReservedFunds
        case "Other":
            return OfflineReason.Other
        default:
            throw new Error(`Unrecognized offline reason ${kind}`)
    }
}

interface WorkerChanges {
    readonly id: string

    owner?: string
    status?: WorkerStatus
    implName?: string
    implVersion?: number
    attestationMethod?: AttestationMethod
    lastAttestedAt?: Date
    lastHeartbeatReceivedAt?: Date
    offlineAt?: Date
    offlineReason?: OfflineReason

    createdAt?: Date
    updatedAt: Date
    deletedAt?: Date
}

export function preprocessWorkersEvents(ctx: Context): Map<string, WorkerChanges> {
    const workersChangeSet= new Map<string, WorkerChanges>();

    for (let block of ctx.blocks) {
        const blockTime = new Date(block.header.timestamp);

        for (let item of block.items) {
            if (item.name == "OffchainComputingWorkers.Registered") {
                let e = new WorkerRegisteredEvent(ctx, item.event)
                let rec: { worker: Uint8Array, owner: Uint8Array }
                if (e.isV100) {
                    rec = e.asV100
                } else {
                    throw new Error("Unsupported spec")
                }

                const id = decodeSS58Address(rec.worker)
                const workerChanges: WorkerChanges = {
                    id,
                    owner: decodeSS58Address(rec.owner),
                    status: WorkerStatus.Registered,
                    deregistered: false,
                    updatedAtBlockNumber: block.header.height
                }

                workersChangeSet.set(id, workerChanges)
            } else if (item.name == "OffchainComputingWorkers.Deregistered") {
                let e = new WorkerDeregisteredEvent(ctx, item.event)
                let rec: { worker: Uint8Array, force: boolean }
                if (e.isV100) {
                    rec = e.asV100
                } else {
                    throw new Error('Unsupported spec')
                }

                const id = decodeSS58Address(rec.worker)
                let workerChanges = workersChangeSet.get(id)
                if (!workerChanges) {
                    workerChanges = {
                        id,
                        deregistered: true,
                        updatedAtBlockNumber: block.header.height
                    }
                } else {
                    workerChanges.deregistered = true
                    workerChanges.updatedAtBlockNumber = block.header.height
                }

                workersChangeSet.set(id, workerChanges)
            } else if (item.name == "OffchainComputingWorkers.Online") {
                let e = new WorkerOnlineEvent(ctx, item.event)
                let rec: {
                    worker: Uint8Array,
                    implName: Uint8Array,
                    implVersion: number,
                    attestationMethod: v100.AttestationMethod | undefined,
                    nextHeartbeat: number
                }
                if (e.isV100) {
                    rec = e.asV100
                } else {
                    throw new Error('Unsupported spec')
                }

                const id = decodeSS58Address(rec.worker)
                let workerChanges: WorkerChanges = workersChangeSet.get(id) || {
                    id,
                    deregistered: false,
                    updatedAtBlockNumber: block.header.height
                }
                workerChanges.status = WorkerStatus.Online
                workerChanges.implName = u8aToString(rec.implName)
                workerChanges.implVersion = rec.implVersion
                workerChanges.attestationMethod = decodeAttestationMethod(rec.attestationMethod)
                workerChanges.lastAttestedAt = blockTime
                workerChanges.updatedAtBlockNumber = block.header.height

                workersChangeSet.set(id, workerChanges)
            } else if (item.name == "OffchainComputingWorkers.RequestingOffline") {
                let e = new WorkerRequestingOfflineEvent(ctx, item.event)
                let rec: { worker: Uint8Array }
                if (e.isV100) {
                    rec = e.asV100
                } else {
                    throw new Error('Unsupported spec')
                }

                const id = decodeSS58Address(rec.worker)
                let workerChanges: WorkerChanges = workersChangeSet.get(id) || {
                    id,
                    deregistered: false,
                    updatedAtBlockNumber: block.header.height
                }
                workerChanges.status = WorkerStatus.RequestingOffline
                workerChanges.updatedAtBlockNumber = block.header.height

                workersChangeSet.set(id, workerChanges)
            } else if (item.name == "OffchainComputingWorkers.Offline") {
                let e = new WorkerOfflineEvent(ctx, item.event)
                let rec: { worker: Uint8Array, reason: v100.OfflineReason }
                if (e.isV100) {
                    rec = e.asV100
                } else {
                    throw new Error('Unsupported spec')
                }

                const id = decodeSS58Address(rec.worker)
                let workerChanges: WorkerChanges = workersChangeSet.get(id) || {
                    id,
                    deregistered: false,
                    updatedAtBlockNumber: block.header.height
                }
                workerChanges.status = WorkerStatus.Offline
                workerChanges.offlineReason = decodeOfflineReason(rec.reason)
                workerChanges.offlineAt = blockTime
                workerChanges.updatedAtBlockNumber = block.header.height

                workersChangeSet.set(id, workerChanges)
            } else if (item.name == "OffchainComputingWorkers.HeartbeatReceived") {
                let e = new WorkerHeartbeatReceivedEvent(ctx, item.event)
                let rec: { worker: Uint8Array, nextHeartbeat: number }
                if (e.isV100) {
                    rec = e.asV100
                } else {
                    throw new Error('Unsupported spec')
                }

                const id = decodeSS58Address(rec.worker)
                let workerChanges: WorkerChanges = workersChangeSet.get(id) || {
                    id,
                    deregistered: false,
                    updatedAtBlockNumber: block.header.height
                }
                workerChanges.lastHeartbeatReceivedAt = blockTime
                workerChanges.updatedAtBlockNumber = block.header.height

                workersChangeSet.set(id, workerChanges)
            } else if (item.name == "OffchainComputingWorkers.AttestationRefreshed") {
                let e = new WorkerAttestationRefreshedEvent(ctx, item.event)
                let rec: { worker: Uint8Array, method: v100.AttestationMethod }
                if (e.isV100) {
                    rec = e.asV100
                } else {
                    throw new Error('Unsupported spec')
                }

                const id = decodeSS58Address(rec.worker)
                let workerChanges: WorkerChanges = workersChangeSet.get(id) || {
                    id,
                    deregistered: false,
                    updatedAtBlockNumber: block.header.height
                }
                workerChanges.attestationMethod = decodeAttestationMethod(rec.method)
                workerChanges.lastAttestedAt = blockTime
                workerChanges.updatedAtBlockNumber = block.header.height

                workersChangeSet.set(id, workerChanges)
            }
        }
    }

    return workersChangeSet
}
