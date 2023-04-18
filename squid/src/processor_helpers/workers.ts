import {type Context} from "../processor"
import {
    OffchainComputingWorkersWorkerAttestationRefreshedEvent as WorkerAttestationRefreshedEvent,
    OffchainComputingWorkersWorkerDeregisteredEvent as WorkerDeregisteredEvent,
    OffchainComputingWorkersWorkerHeartbeatReceivedEvent as WorkerHeartbeatReceivedEvent,
    OffchainComputingWorkersWorkerOfflineEvent as WorkerOfflineEvent,
    OffchainComputingWorkersWorkerOnlineEvent as WorkerOnlineEvent,
    OffchainComputingWorkersWorkerRegisteredEvent as WorkerRegisteredEvent,
    OffchainComputingWorkersWorkerRequestingOfflineEvent as WorkerRequestingOfflineEvent,
} from "../types/events"
import * as v100 from "../types/v100"
import {AttestationMethod, OfflineReason, WorkerEventKind, WorkerStatus} from "../model"
import {decodeSS58Address} from "../utils"
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

interface WorkerEvent {
    readonly id: string

    readonly kind: WorkerEventKind
    readonly payload?: any

    readonly blockNumber: number
    readonly blockTime: Date
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

    createdAt: Date
    updatedAt: Date
    deletedAt?: Date

    registerWorkerCounterChange: number
    onlineWorkerCounterChange: number

    events: WorkerEvent[]
}

export function preprocessWorkersEvents(ctx: Context): Map<string, WorkerChanges> {
    const changeSet= new Map<string, WorkerChanges>();

    for (let block of ctx.blocks) {
        const blockNumber = block.header.height
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
                const changes: WorkerChanges = changeSet.get(id) || {
                    id,
                    createdAt: blockTime,
                    updatedAt: blockTime,
                    registerWorkerCounterChange: 0,
                    onlineWorkerCounterChange: 0,
                    events: []
                }

                changes.updatedAt = blockTime
                changes.deletedAt = undefined

                changes.owner = decodeSS58Address(rec.owner)
                changes.status = WorkerStatus.Registered
                changes.registerWorkerCounterChange = 1
                changes.onlineWorkerCounterChange = 0
                changes.events.push({
                    id: `${id}-${blockNumber}-${item.event.indexInBlock}`,
                    kind: WorkerEventKind.Registered,
                    blockNumber,
                    blockTime,
                })

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
                    createdAt: blockTime,
                    updatedAt: blockTime,
                    registerWorkerCounterChange: 0,
                    onlineWorkerCounterChange: 0,
                    events: []
                }
                assert(!changes.deletedAt)

                changes.updatedAt = blockTime
                changes.deletedAt = blockTime

                changes.registerWorkerCounterChange -= 1
                changes.onlineWorkerCounterChange -= rec.force ? 1 : 0
                changes.events.push({
                    id: `${id}-${blockNumber}-${item.event.indexInBlock}`,
                    kind: WorkerEventKind.Deregistered,
                    payload: {force: rec.force},
                    blockNumber,
                    blockTime,
                })

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
                const changes: WorkerChanges = changeSet.get(id) || {
                    id,
                    createdAt: blockTime,
                    updatedAt: blockTime,
                    registerWorkerCounterChange: 0,
                    onlineWorkerCounterChange: 0,
                    events: []
                }
                assert(!changes.deletedAt)

                changes.status = WorkerStatus.Online
                changes.implId = rec.implId
                changes.implSpecVersion = rec.implSpecVersion
                changes.implBuildVersion = rec.implBuildVersion
                changes.attestationMethod = decodeAttestationMethod(rec.attestationMethod)
                changes.attestationExpiresAt = rec.attestationExpiresAt ? new Date(Number(rec.attestationExpiresAt)) : undefined
                changes.lastAttestedAt = blockTime
                changes.lastHeartbeatReceivedAt = blockTime
                changes.updatedAt = blockTime

                changes.onlineWorkerCounterChange += 1
                changes.events.push({
                    id: `${id}-${blockNumber}-${item.event.indexInBlock}`,
                    kind: WorkerEventKind.Online,
                    blockNumber,
                    blockTime,
                })

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
                const changes: WorkerChanges = changeSet.get(id) || {
                    id,
                    createdAt: blockTime,
                    updatedAt: blockTime,
                    registerWorkerCounterChange: 0,
                    onlineWorkerCounterChange: 0,
                    events: []
                }
                assert(!changes.deletedAt)

                changes.status = WorkerStatus.RequestingOffline
                changes.updatedAt = blockTime

                changes.events.push({
                    id: `${id}-${blockNumber}-${item.event.indexInBlock}`,
                    kind: WorkerEventKind.RequestingOffline,
                    blockNumber,
                    blockTime,
                })

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
                const changes: WorkerChanges = changeSet.get(id) || {
                    id,
                    createdAt: blockTime,
                    updatedAt: blockTime,
                    registerWorkerCounterChange: 0,
                    onlineWorkerCounterChange: 0,
                    events: []
                }
                assert(!changes.deletedAt)

                changes.status = WorkerStatus.Offline
                changes.offlineReason = decodeOfflineReason(rec.reason)
                changes.offlineAt = blockTime
                changes.updatedAt = blockTime

                changes.onlineWorkerCounterChange -= 1
                changes.events.push({
                    id: `${id}-${blockNumber}-${item.event.indexInBlock}`,
                    kind: WorkerEventKind.Offline,
                    blockNumber,
                    blockTime,
                })

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
                const changes: WorkerChanges = changeSet.get(id) || {
                    id,
                    createdAt: blockTime,
                    updatedAt: blockTime,
                    registerWorkerCounterChange: 0,
                    onlineWorkerCounterChange: 0,
                    events: []
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
                const changes: WorkerChanges = changeSet.get(id) || {
                    id,
                    createdAt: blockTime,
                    updatedAt: blockTime,
                    registerWorkerCounterChange: 0,
                    onlineWorkerCounterChange: 0,
                    events: []
                }
                assert(!changes.deletedAt)

                changes.attestationExpiresAt = rec.expiresAt ? new Date(Number(rec.expiresAt)) : null
                changes.lastAttestedAt = blockTime
                changes.updatedAt = blockTime

                changes.events.push({
                    id: `${id}-${blockNumber}-${item.event.indexInBlock}`,
                    kind: WorkerEventKind.AttestationRefreshed,
                    blockNumber,
                    blockTime,
                })

                changeSet.set(id, changes)
            }
        }
    }

    return changeSet
}
