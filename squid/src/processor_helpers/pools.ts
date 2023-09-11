import type {Context} from "../processor"
import {
  OffchainComputingPoolPoolCreatedEventV100 as PoolCreatedEventV100,
  OffchainComputingPoolPoolDestroyedEventV100 as PoolDestroyedEventV100,
  OffchainComputingPoolPoolMetadataRemovedEventV100 as PoolMetadataRemovedEventV100,
  OffchainComputingPoolPoolMetadataUpdatedEventV100 as PoolMetadataUpdatedEventV100,
  OffchainComputingPoolPoolSettingsUpdatedEventV100 as PoolSettingsUpdatedEventV100,
} from "../types/events"
import {decodeSS58Address, hexToString, hexToU8a} from "../utils";
import assert from "assert";
import * as v100 from "../types/v100";
import {JobScheduler} from "../model";

function decodeJobScheduler(scheduler?: v100.JobScheduler): JobScheduler {
  if (!scheduler) {
    throw new Error("Unexpected undefined scheduler")
  }

  const kind = scheduler.__kind
  switch (kind) {
    case "External":
      return JobScheduler.External
    default:
      throw new Error(`Unrecognized scope ${kind}`)
  }
}


interface PoolChanges {
  readonly id: string
  readonly poolId: number

  owner?: string
  implId?: number

  minImplSpecVersion?: number,
  maxImplSpecVersion?: number,
  jobScheduler?: JobScheduler,
  createJobEnabled?: boolean
  autoDestroyProcessedJobEnabled?: boolean
  metadata?: Uint8Array | null

  createdAt: Date
  updatedAt: Date
  deletedAt?: Date | null
}

export function preprocessPoolsEvents(ctx: Context): Map<string, PoolChanges> {
  const changeSet = new Map<string, PoolChanges>();

  for (let block of ctx.blocks) {
    assert(block.header.timestamp)
    const blockTime = new Date(block.header.timestamp);

    for (let event of block.events) {
      if (event.name == "OffchainComputingPool.PoolCreated") {
        let rec: {
          owner: string,
          poolId: number,
          implId: number,
          jobScheduler: v100.JobScheduler,
          createJobEnabled: boolean,
          autoDestroyProcessedJobEnabled: boolean
        }
        if (PoolCreatedEventV100.is(event)) {
          rec = PoolCreatedEventV100.decode(event)
        } else {
          throw new Error("Unsupported spec")
        }

        const id = rec.poolId.toString()
        const changes: PoolChanges = changeSet.get(id) || {
          id,
          poolId: rec.poolId,
          createdAt: blockTime,
          updatedAt: blockTime
        }

        changes.owner = decodeSS58Address(hexToU8a(rec.owner))
        changes.implId = rec.poolId
        changes.minImplSpecVersion = 1
        changes.maxImplSpecVersion = 1
        changes.jobScheduler = decodeJobScheduler(rec.jobScheduler)
        changes.createJobEnabled = rec.createJobEnabled
        changes.autoDestroyProcessedJobEnabled = rec.autoDestroyProcessedJobEnabled
        changes.metadata = null

        changes.deletedAt = null
        changes.updatedAt = blockTime

        changeSet.set(id, changes)
      } else if (event.name == "OffchainComputingPool.PoolDestroyed") {
        let rec: { poolId: number }
        if (PoolDestroyedEventV100.is(event)) {
          rec = PoolDestroyedEventV100.decode(event)
        } else {
          throw new Error('Unsupported spec')
        }

        const id = rec.poolId.toString()
        const changes: PoolChanges = changeSet.get(id) || {
          id,
          poolId: rec.poolId,
          createdAt: blockTime,
          updatedAt: blockTime
        }

        changes.deletedAt = blockTime
        changes.updatedAt = blockTime

        changeSet.set(id, changes)
      } else if (event.name == "OffchainComputingPool.PoolMetadataUpdated") {
        let rec: { poolId: number, metadata: string }
        if (PoolMetadataUpdatedEventV100.is(event)) {
          rec = PoolMetadataUpdatedEventV100.decode(event)
        } else {
          throw new Error('Unsupported spec')
        }

        const id = rec.poolId.toString()
        const changes: PoolChanges = changeSet.get(id) || {
          id,
          poolId: rec.poolId,
          createdAt: blockTime,
          updatedAt: blockTime
        }
        assert(!changes.deletedAt)

        changes.metadata = (() => {
          if (rec.metadata === undefined) {
            return null
          }

          try {
            return JSON.parse(hexToString(rec.metadata))
          } catch (_e) {}

          return rec.metadata
        })()
        changes.updatedAt = blockTime

        changeSet.set(id, changes)
      } else if (event.name == "OffchainComputingPool.PoolMetadataRemoved") {
        let rec: { poolId: number }
        if (PoolMetadataRemovedEventV100.is(event)) {
          rec = PoolMetadataRemovedEventV100.decode(event)
        } else {
          throw new Error('Unsupported spec')
        }

        const id = rec.poolId.toString()
        const changes: PoolChanges = changeSet.get(id) || {
          id,
          poolId: rec.poolId,
          createdAt: blockTime,
          updatedAt: blockTime
        }
        assert(!changes.deletedAt)

        changes.metadata = null
        changes.updatedAt = blockTime

        changeSet.set(id, changes)
      } else if (event.name == "OffchainComputingPool.PoolSettingsUpdated") {
        let rec: {
          poolId: number,
          minImplSpecVersion: number,
          maxImplSpecVersion: number,
          createJobEnabled: boolean,
          autoDestroyProcessedJobEnabled: boolean
        }
        if (PoolSettingsUpdatedEventV100.is(event)) {
          rec = PoolSettingsUpdatedEventV100.decode(event)
        } else {
          throw new Error('Unsupported spec')
        }

        const id = rec.poolId.toString()
        const changes: PoolChanges = changeSet.get(id) || {
          id,
          poolId: rec.poolId,
          createdAt: blockTime,
          updatedAt: blockTime
        }
        assert(!changes.deletedAt)

        changes.minImplSpecVersion = rec.minImplSpecVersion
        changes.maxImplSpecVersion = rec.maxImplSpecVersion
        changes.createJobEnabled = rec.createJobEnabled
        changes.autoDestroyProcessedJobEnabled = rec.autoDestroyProcessedJobEnabled
        changes.updatedAt = blockTime

        changeSet.set(id, changes)
      }
    }
  }

  return changeSet
}
