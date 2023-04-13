import {type Context} from "../processor"
import {
    OffchainComputingTaskAssignedEvent as TaskAssignedEvent,
    OffchainComputingTaskCreatedEvent as TaskCreatedEvent,
    OffchainComputingTaskDestroyedEvent as TaskDestroyedEvent,
    OffchainComputingTaskReleasedEvent as TaskReleasedEvent,
    OffchainComputingTaskResultUpdatedEvent as TaskResultUpdatedEvent,
    OffchainComputingTaskStatusUpdatedEvent as TaskStatusUpdatedEvent,
} from "../types/events"
import * as v100 from "../types/v100";
import {TaskResult, TaskStatus} from "../model/index";
import {decodeSS58Address, u8aToString} from "../utils"
import assert from "assert";

function decodeTaskStatus(taskStatus?: v100.TaskStatus): TaskStatus {
    if (!taskStatus) {
        throw new Error("Unexpected undefined task status")
    }

    const kind = taskStatus.__kind
    switch (kind) {
        case "Pending":
            return TaskStatus.Pending
        case "Processing":
            return TaskStatus.Processing
        case "Processed":
            return TaskStatus.Processed
        default:
            throw new Error(`Unrecognized task status ${kind}`)
    }
}

function decodeTaskResult(taskResult?: v100.TaskResult): TaskResult {
    if (!taskResult) {
        throw new Error("Unexpected undefined task result")
    }

    const kind = taskResult.__kind
    switch (kind) {
        case "Success":
            return TaskResult.Success
        case "Failed":
            return TaskResult.Failed
        case "Errored":
            return TaskResult.Errored
        default:
            throw new Error(`Unrecognized task result ${kind}`)
    }
}

interface TaskChanges {
    readonly id: string
    readonly taskId: number

    poolId?: number
    policyId?: number
    owner?: string
    assignee?: string | null
    destroyer?: string

    implSpecVersion?: number
    status?: TaskStatus
    result?: TaskResult

    input?: string | null
    output?: string | null
    proof?: string | null

    expiresAt?: Date
    assignedAt?: Date | null
    processingAt?: Date
    processedAt?: Date
    createdAt?: Date
    updatedAt: Date
    deletedAt?: Date
}

export function preprocessTasksEvents(ctx: Context): Map<string, TaskChanges> {
    const changeSet= new Map<string, TaskChanges>();

    for (let block of ctx.blocks) {
        const blockTime = new Date(block.header.timestamp);

        for (let item of block.items) {
            if (item.name == "OffchainComputing.TaskCreated") {
                let e = new TaskCreatedEvent(ctx, item.event)
                let rec: {
                    poolId: number,
                    taskId: number,
                    policyId: number,
                    owner: Uint8Array,
                    implSpecVersion: number,
                    input: (Uint8Array | undefined),
                    expiresIn: bigint
                }
                if (e.isV100) {
                    rec = e.asV100
                } else {
                    throw new Error("Unsupported spec")
                }

                const id = `${rec.poolId}-${rec.taskId}`
                const changes: TaskChanges = {
                    id,
                    poolId: rec.poolId,
                    taskId: rec.taskId,
                    policyId: rec.policyId,
                    owner: decodeSS58Address(rec.owner),
                    status: TaskStatus.Pending,
                    implSpecVersion: rec.implSpecVersion,
                    input: rec.input ? u8aToString(rec.input) : null,
                    expiresAt: new Date(block.header.timestamp + Number(rec.expiresIn) * 1000),
                    createdAt: blockTime,
                    updatedAt: blockTime
                }

                changeSet.set(id, changes)
            } else if (item.name == "OffchainComputing.TaskDestroyed") {
                let e = new TaskDestroyedEvent(ctx, item.event)
                let rec: { poolId: number, taskId: number, destroyer: Uint8Array }
                if (e.isV100) {
                    rec = e.asV100
                } else {
                    throw new Error('Unsupported spec')
                }

                const id = `${rec.poolId}-${rec.taskId}`
                let changes: TaskChanges = changeSet.get(id) || {
                    id,
                    taskId: rec.taskId,
                    poolId: rec.poolId,
                    updatedAt: blockTime,
                }
                assert(!changes.deletedAt)

                changes.destroyer = decodeSS58Address(rec.destroyer)
                changes.updatedAt = blockTime
                changes.deletedAt = blockTime

                changeSet.set(id, changes)
            } else if (item.name == "OffchainComputing.TaskAssigned") {
                let e = new TaskAssignedEvent(ctx, item.event)
                let rec: { poolId: number, taskId: number, assignee: Uint8Array }
                if (e.isV100) {
                    rec = e.asV100
                } else {
                    throw new Error('Unsupported spec')
                }

                const id = `${rec.poolId}-${rec.taskId}`
                let changes: TaskChanges = changeSet.get(id) || {
                    id,
                    taskId: rec.taskId,
                    poolId: rec.poolId,
                    updatedAt: blockTime,
                }
                assert(!changes.deletedAt)

                changes.assignee = decodeSS58Address(rec.assignee)
                changes.assignedAt = blockTime
                changes.updatedAt = blockTime

                changeSet.set(id, changes)
            } else if (item.name == "OffchainComputing.TaskReleased") {
                let e = new TaskReleasedEvent(ctx, item.event)
                let rec: { poolId: number, taskId: number }
                if (e.isV100) {
                    rec = e.asV100
                } else {
                    throw new Error('Unsupported spec')
                }

                const id = `${rec.poolId}-${rec.taskId}`
                let changes: TaskChanges = changeSet.get(id) || {
                    id,
                    taskId: rec.taskId,
                    poolId: rec.poolId,
                    updatedAt: blockTime,
                }
                assert(!changes.deletedAt)

                changes.assignee = null
                changes.assignedAt = null
                changes.updatedAt = blockTime

                changeSet.set(id, changes)
            } else if (item.name == "OffchainComputing.TaskStatusUpdated") {
                let e = new TaskStatusUpdatedEvent(ctx, item.event)
                let rec: { poolId: number, taskId: number, status: v100.TaskStatus }
                if (e.isV100) {
                    rec = e.asV100
                } else {
                    throw new Error('Unsupported spec')
                }

                const id = `${rec.poolId}-${rec.taskId}`
                let changes: TaskChanges = changeSet.get(id) || {
                    id,
                    taskId: rec.taskId,
                    poolId: rec.poolId,
                    updatedAt: blockTime,
                }
                assert(!changes.deletedAt)

                changes.status = decodeTaskStatus(rec.status)
                if (changes.status == TaskStatus.Processing) {
                    changes.processingAt = blockTime
                } else if (changes.status == TaskStatus.Processed) {
                    changes.processedAt = blockTime
                }
                changes.updatedAt = blockTime

                changeSet.set(id, changes)
            } else if (item.name == "OffchainComputing.TaskResultUpdated") {
                let e = new TaskResultUpdatedEvent(ctx, item.event)
                let rec: {
                    poolId: number,
                    taskId: number,
                    result: v100.TaskResult,
                    output: (Uint8Array | undefined),
                    proof: (Uint8Array | undefined)
                }
                if (e.isV100) {
                    rec = e.asV100
                } else {
                    throw new Error('Unsupported spec')
                }

                const id = `${rec.poolId}-${rec.taskId}`
                let changes: TaskChanges = changeSet.get(id) || {
                    id,
                    taskId: rec.taskId,
                    poolId: rec.poolId,
                    updatedAt: blockTime,
                }
                assert(!changes.deletedAt)

                changes.result = decodeTaskResult(rec.result)
                changes.output = rec.output ? u8aToString(rec.output) : null
                changes.proof = rec.proof ? u8aToString(rec.proof) : null
                changes.updatedAt = blockTime

                changeSet.set(id, changes)
            }
        }
    }

    return changeSet
}

