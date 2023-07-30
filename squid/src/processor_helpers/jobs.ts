import type {Context} from "../processor"
import {
    OffchainComputingJobAssignedEvent as JobAssignedEvent,
    OffchainComputingJobCreatedEvent as JobCreatedEvent,
    OffchainComputingJobDestroyedEvent as JobDestroyedEvent,
    OffchainComputingJobReleasedEvent as JobReleasedEvent,
    OffchainComputingJobResultUpdatedEvent as JobResultUpdatedEvent,
    OffchainComputingJobStatusUpdatedEvent as JobStatusUpdatedEvent,
} from "../types/events"
import * as v100 from "../types/v100";
import {JobEventKind, JobResult, JobStatus} from "../model";
import {decodeSS58Address} from "../utils"
import assert from "assert";

function decodeJobStatus(jobStatus?: v100.JobStatus): JobStatus {
    if (!jobStatus) {
        throw new Error("Unexpected undefined job status")
    }

    const kind = jobStatus.__kind
    switch (kind) {
        case "Pending":
            return JobStatus.Pending
        case "Processing":
            return JobStatus.Processing
        case "Processed":
            return JobStatus.Processed
        case "Discarded":
            return JobStatus.Discarded
        default:
            throw new Error(`Unrecognized job status ${kind}`)
    }
}

function decodeJobResult(jobResult?: v100.JobResult): JobResult {
    if (!jobResult) {
        throw new Error("Unexpected undefined job result")
    }

    const kind = jobResult.__kind
    switch (kind) {
        case "Success":
            return JobResult.Success
        case "Fail":
            return JobResult.Fail
        case "Error":
            return JobResult.Error
        case "Panic":
            return JobResult.Panic
        default:
            throw new Error(`Unrecognized job result ${kind}`)
    }
}

function convertJobResultToEventKind(jobResult: JobResult): JobEventKind {
    switch (jobResult) {
        case "Success":
            return JobEventKind.Success
        case "Fail":
            return JobEventKind.Fail
        case "Error":
            return JobEventKind.Error
        case "Panic":
            return JobEventKind.Panic
        default:
            throw new Error(`Unrecognized job result ${jobResult}`)
    }
}

interface JobEvent {
    readonly id: string
    readonly sequence: number

    readonly kind: JobEventKind
    readonly payload?: any

    readonly blockNumber: number
    readonly blockTime: Date
}

interface JobChanges {
    readonly id: string
    readonly jobId: number

    uniqueTrackId?: number
    poolId?: number
    policyId?: number
    depositor?: string
    beneficiary?: string
    assignee?: string | null
    destroyer?: string

    implSpecVersion?: number
    status?: JobStatus
    result?: JobResult

    input?: Uint8Array | null
    output?: Uint8Array | null
    proof?: Uint8Array | null

    expiresAt?: Date
    assignedAt?: Date | null
    processingAt?: Date
    endedAt?: Date
    createdAt: Date
    updatedAt: Date
    deletedAt?: Date | null

    events: JobEvent[]
}

export function preprocessJobsEvents(ctx: Context): Map<string, JobChanges> {
    const changeSet= new Map<string, JobChanges>();

    for (let block of ctx.blocks) {
        assert(block.header.timestamp)
        const blockNumber = block.header.height
        const blockTime = new Date(block.header.timestamp);

        for (let event of block.events) {
            if (event.name == "OffchainComputing.JobCreated") {
                let e = new JobCreatedEvent(ctx, event)
                let rec: {
                    poolId: number,
                    jobId: number,
                    uniqueTrackId: (number | undefined),
                    policyId: number,
                    depositor: Uint8Array,
                    beneficiary: Uint8Array,
                    implSpecVersion: number,
                    input: (Uint8Array | undefined),
                    expiresIn: bigint
                }
                if (e.isV100) {
                    rec = e.asV100
                } else {
                    throw new Error("Unsupported spec")
                }

                const id = `${rec.poolId}-${rec.jobId}`
                const changes: JobChanges = changeSet.get(id) || {
                    id,
                    poolId: rec.poolId,
                    jobId: rec.jobId,
                    createdAt: blockTime,
                    updatedAt: blockTime,
                    events: []
                }

                changes.uniqueTrackId = rec.uniqueTrackId
                changes.policyId = rec.policyId
                changes.depositor = decodeSS58Address(rec.depositor)
                changes.beneficiary = decodeSS58Address(rec.beneficiary)
                changes.status = JobStatus.Pending
                changes.implSpecVersion = rec.implSpecVersion
                changes.input = rec.input ? rec.input : null
                changes.expiresAt = new Date(block.header.timestamp + Number(rec.expiresIn) * 1000)

                changes.deletedAt = null
                changes.updatedAt = blockTime

                changes.events.push({
                    id: `${id}-${blockNumber}-${event.extrinsicIndex}-${changes.events.length}`,
                    sequence: blockNumber * 100 + changes.events.length,
                    kind: JobEventKind.Created,
                    blockNumber,
                    blockTime,
                })

                changeSet.set(id, changes)
            } else if (event.name == "OffchainComputing.JobDestroyed") {
                let e = new JobDestroyedEvent(ctx, event)
                let rec: {
                    poolId: number,
                    jobId: number,
                    uniqueTrackId: (number | undefined),
                    destroyer: Uint8Array,
                    force: boolean
                }
                if (e.isV100) {
                    rec = e.asV100
                } else {
                    throw new Error('Unsupported spec')
                }

                const id = `${rec.poolId}-${rec.jobId}`
                const changes: JobChanges = changeSet.get(id) || {
                    id,
                    poolId: rec.poolId,
                    jobId: rec.jobId,
                    createdAt: blockTime,
                    updatedAt: blockTime,
                    events: []
                }
                assert(!changes.deletedAt)

                changes.uniqueTrackId = rec.uniqueTrackId
                changes.destroyer = decodeSS58Address(rec.destroyer)
                changes.updatedAt = blockTime
                changes.deletedAt = blockTime

                changes.events.push({
                    id: `${id}-${blockNumber}-${event.extrinsicIndex}-${changes.events.length}`,
                    sequence: blockNumber * 100 + changes.events.length,
                    kind: JobEventKind.Destroyed,
                    payload: {force: rec.force},
                    blockNumber,
                    blockTime,
                })

                changeSet.set(id, changes)
            } else if (event.name == "OffchainComputing.JobAssigned") {
                let e = new JobAssignedEvent(ctx, event)
                let rec: { poolId: number, jobId: number, assignee: Uint8Array }
                if (e.isV100) {
                    rec = e.asV100
                } else {
                    throw new Error('Unsupported spec')
                }

                const id = `${rec.poolId}-${rec.jobId}`
                const changes: JobChanges = changeSet.get(id) || {
                    id,
                    poolId: rec.poolId,
                    jobId: rec.jobId,
                    createdAt: blockTime,
                    updatedAt: blockTime,
                    events: []
                }
                assert(!changes.deletedAt)

                changes.assignee = decodeSS58Address(rec.assignee)
                changes.assignedAt = blockTime
                changes.updatedAt = blockTime

                changes.events.push({
                    id: `${id}-${blockNumber}-${event.extrinsicIndex}-${changes.events.length}`,
                    sequence: blockNumber * 100 + changes.events.length,
                    kind: JobEventKind.Assigned,
                    payload: {assignee: changes.assignee},
                    blockNumber,
                    blockTime,
                })

                changeSet.set(id, changes)
            } else if (event.name == "OffchainComputing.JobReleased") {
                let e = new JobReleasedEvent(ctx, event)
                let rec: { poolId: number, jobId: number }
                if (e.isV100) {
                    rec = e.asV100
                } else {
                    throw new Error('Unsupported spec')
                }

                const id = `${rec.poolId}-${rec.jobId}`
                const changes: JobChanges = changeSet.get(id) || {
                    id,
                    poolId: rec.poolId,
                    jobId: rec.jobId,
                    createdAt: blockTime,
                    updatedAt: blockTime,
                    events: []
                }
                assert(!changes.deletedAt)

                changes.assignee = null
                changes.assignedAt = null
                changes.updatedAt = blockTime

                changes.events.push({
                    id: `${id}-${blockNumber}-${event.extrinsicIndex}-${changes.events.length}`,
                    sequence: blockNumber * 100 + changes.events.length,
                    kind: JobEventKind.Released,
                    blockNumber,
                    blockTime,
                })

                changeSet.set(id, changes)
            } else if (event.name == "OffchainComputing.JobStatusUpdated") {
                let e = new JobStatusUpdatedEvent(ctx, event)
                let rec: { poolId: number, jobId: number, status: v100.JobStatus }
                if (e.isV100) {
                    rec = e.asV100
                } else {
                    throw new Error('Unsupported spec')
                }

                const id = `${rec.poolId}-${rec.jobId}`
                const changes: JobChanges = changeSet.get(id) || {
                    id,
                    poolId: rec.poolId,
                    jobId: rec.jobId,
                    createdAt: blockTime,
                    updatedAt: blockTime,
                    events: []
                }
                assert(!changes.deletedAt)

                changes.status = decodeJobStatus(rec.status)
                if (changes.status == JobStatus.Processing) {
                    changes.processingAt = blockTime
                } else if (changes.status == JobStatus.Processed) {
                    changes.endedAt = blockTime
                }
                changes.updatedAt = blockTime

                if (changes.status == JobStatus.Processing) {
                    changes.events.push({
                        id: `${id}-${blockNumber}-${event.extrinsicIndex}-${changes.events.length}`,
                        sequence: blockNumber * 100 + changes.events.length,
                        kind: JobEventKind.Processing,
                        blockNumber,
                        blockTime,
                    })
                }

                changeSet.set(id, changes)
            } else if (event.name == "OffchainComputing.JobResultUpdated") {
                let e = new JobResultUpdatedEvent(ctx, event)
                let rec: {
                    poolId: number,
                    jobId: number,
                    result: v100.JobResult,
                    output: (Uint8Array | undefined),
                    proof: (Uint8Array | undefined)
                }
                if (e.isV100) {
                    rec = e.asV100
                } else {
                    throw new Error('Unsupported spec')
                }

                const id = `${rec.poolId}-${rec.jobId}`
                const changes: JobChanges = changeSet.get(id) || {
                    id,
                    poolId: rec.poolId,
                    jobId: rec.jobId,
                    createdAt: blockTime,
                    updatedAt: blockTime,
                    events: []
                }
                assert(!changes.deletedAt)

                changes.result = decodeJobResult(rec.result)
                changes.output = rec.output ? rec.output : null
                changes.proof = rec.proof ? rec.proof : null
                changes.updatedAt = blockTime

                changes.events.push({
                    id: `${id}-${blockNumber}-${event.extrinsicIndex}-${changes.events.length}`,
                    sequence: blockNumber * 100 + changes.events.length,
                    kind: convertJobResultToEventKind(changes.result),
                    blockNumber,
                    blockTime,
                })

                changeSet.set(id, changes)
            }
        }
    }

    return changeSet
}

