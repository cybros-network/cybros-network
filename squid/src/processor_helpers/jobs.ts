import type {Context} from "../processor"
import {
  OffchainComputingPoolJobAssignedEventV100 as JobAssignedEventV100,
  OffchainComputingPoolJobCreatedEventV100 as JobCreatedEventV100,
  OffchainComputingPoolJobDestroyedEventV100 as JobDestroyedEventV100,
  OffchainComputingPoolJobReleasedEventV100 as JobReleasedEventV100,
  OffchainComputingPoolJobResultUpdatedEventV100 as JobResultUpdatedEventV100,
  OffchainComputingPoolJobStatusUpdatedEventV100 as JobStatusUpdatedEventV100,
} from "../types/events"
import * as v100 from "../types/v100";
import {JobEventKind, JobResult, JobStatus} from "../model";
import {decodeSS58Address, hexToString, hexToU8a} from "../utils"
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

  input?: string | null
  output?: string | null
  proof?: string | null

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
  const changeSet = new Map<string, JobChanges>();

  for (let block of ctx.blocks) {
    assert(block.header.timestamp)
    const blockNumber = block.header.height
    const blockTime = new Date(block.header.timestamp);

    for (let event of block.events) {
      if (event.name == "OffchainComputingPool.JobCreated") {
        let rec: {
          poolId: number,
          jobId: number,
          policyId: number,
          depositor: string,
          beneficiary: string,
          implSpecVersion: number,
          input?: string,
          uniqueTrackId?: number,
          expiresIn: bigint
        }
        if (JobCreatedEventV100.is(event)) {
          rec = JobCreatedEventV100.decode(event)
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
        changes.depositor = decodeSS58Address(hexToU8a(rec.depositor))
        changes.beneficiary = decodeSS58Address(hexToU8a(rec.beneficiary))
        changes.status = JobStatus.Pending
        changes.implSpecVersion = rec.implSpecVersion
        changes.input = (() => {
          if (rec.input === undefined) {
            return null
          }

          try {
            return JSON.parse(hexToString(rec.input))
          } catch (_e) {}

          return rec.input
        })()
        changes.expiresAt = new Date(block.header.timestamp + Number(rec.expiresIn) * 1000)

        changes.deletedAt = null
        changes.updatedAt = blockTime

        changes.events.push({
          id: `${id}-${blockNumber}-${event.index}`,
          sequence: blockNumber * 100 + changes.events.length,
          kind: JobEventKind.Created,
          blockNumber,
          blockTime,
        })

        changeSet.set(id, changes)
      } else if (event.name == "OffchainComputingPool.JobDestroyed") {
        let rec: {
          poolId: number,
          jobId: number,
          uniqueTrackId?: number,
          destroyer: string,
          force: boolean
        }
        if (JobDestroyedEventV100.is(event)) {
          rec = JobDestroyedEventV100.decode(event)
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
        changes.destroyer = decodeSS58Address(hexToU8a(rec.destroyer))
        changes.updatedAt = blockTime
        changes.deletedAt = blockTime

        changes.events.push({
          id: `${id}-${blockNumber}-${event.index}`,
          sequence: blockNumber * 100 + changes.events.length,
          kind: JobEventKind.Destroyed,
          payload: {force: rec.force},
          blockNumber,
          blockTime,
        })

        changeSet.set(id, changes)
      } else if (event.name == "OffchainComputingPool.JobAssigned") {
        let rec: { poolId: number, jobId: number, assignee: string }
        if (JobAssignedEventV100.is(event)) {
          rec = JobAssignedEventV100.decode(event)
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

        changes.assignee = decodeSS58Address(hexToU8a(rec.assignee))
        changes.assignedAt = blockTime
        changes.updatedAt = blockTime

        changes.events.push({
          id: `${id}-${blockNumber}-${event.index}`,
          sequence: blockNumber * 100 + changes.events.length,
          kind: JobEventKind.Assigned,
          payload: {assignee: changes.assignee},
          blockNumber,
          blockTime,
        })

        changeSet.set(id, changes)
      } else if (event.name == "OffchainComputingPool.JobReleased") {
        let rec: { poolId: number, jobId: number }
        if (JobReleasedEventV100.is(event)) {
          rec = JobReleasedEventV100.decode(event)
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
          id: `${id}-${blockNumber}-${event.index}`,
          sequence: blockNumber * 100 + changes.events.length,
          kind: JobEventKind.Released,
          blockNumber,
          blockTime,
        })

        changeSet.set(id, changes)
      } else if (event.name == "OffchainComputingPool.JobStatusUpdated") {
        let rec: { poolId: number, jobId: number, status: v100.JobStatus }
        if (JobStatusUpdatedEventV100.is(event)) {
          rec = JobStatusUpdatedEventV100.decode(event)
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
            id: `${id}-${blockNumber}-${event.index}`,
            sequence: blockNumber * 100 + changes.events.length,
            kind: JobEventKind.Processing,
            blockNumber,
            blockTime,
          })
        }

        changeSet.set(id, changes)
      } else if (event.name == "OffchainComputingPool.JobResultUpdated") {
        let rec: {
          poolId: number,
          jobId: number,
          result: v100.JobResult,
          output?: string,
          proof?: string
        }
        if (JobResultUpdatedEventV100.is(event)) {
          rec = JobResultUpdatedEventV100.decode(event)
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
        changes.output = (() => {
          if (rec.output === undefined) {
            return null
          }

          try {
            return JSON.parse(hexToString(rec.output))
          } catch (_e) {}

          return rec.output
        })()
        changes.proof = (() => {
          if (rec.proof === undefined) {
            return null
          }

          try {
            return JSON.parse(hexToString(rec.proof))
          } catch (_e) {}

          return rec.proof
        })()
        changes.updatedAt = blockTime

        changes.events.push({
          id: `${id}-${blockNumber}-${event.index}`,
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

