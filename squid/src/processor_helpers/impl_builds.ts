import type {Context} from "../processor"
import {
  OffchainComputingInfraImplBuildDeregisteredEventV100 as ImplBuildDeregisteredEventV100,
  OffchainComputingInfraImplBuildRegisteredEventV100 as ImplBuildRegisteredEventV100,
  OffchainComputingInfraImplBuildStatusUpdatedEventV100 as ImplBuildStatusUpdatedEventV100,
} from "../types/events"
import * as v100 from "../types/v100"
import {ImplBuildStatus} from "../model"
import assert from "assert";

function decodeImplBuildStatus(implBuildStatus?: v100.ImplBuildStatus): ImplBuildStatus {
  if (!implBuildStatus) {
    throw new Error("Unexpected undefined impl build status")
  }

  const kind = implBuildStatus.__kind
  switch (kind) {
    case "Released":
      return ImplBuildStatus.Released
    case "Deprecated":
      return ImplBuildStatus.Deprecated
    case "Retired":
      return ImplBuildStatus.Retired
    default:
      throw new Error(`Unrecognized impl build status ${kind}`)
  }
}

interface ImplBuildChanges {
  readonly id: string
  readonly implId: number

  readonly version: number
  magicBytes?: string | null
  status: ImplBuildStatus

  createdAt: Date
  updatedAt: Date
  deletedAt?: Date | null
}

export function preprocessImplBuildsEvents(ctx: Context): Map<string, ImplBuildChanges> {
  const changeSet = new Map<string, ImplBuildChanges>();

  for (let block of ctx.blocks) {
    assert(block.header.timestamp)
    const blockTime = new Date(block.header.timestamp);

    for (let event of block.events) {
      if (event.name == "OffchainComputingInfra.ImplBuildRegistered") {
        let rec: {
          implId: number,
          implBuildVersion: number,
          magicBytes?: string
        }
        if (ImplBuildRegisteredEventV100.is(event)) {
          rec = ImplBuildRegisteredEventV100.decode(event)
        } else {
          throw new Error("Unsupported spec")
        }

        const id = `${rec.implId}-${rec.implBuildVersion}`
        const changes: ImplBuildChanges = changeSet.get(id) || {
          id,
          implId: rec.implId,
          version: rec.implBuildVersion,
          status: ImplBuildStatus.Released,
          createdAt: blockTime,
          updatedAt: blockTime
        }

        changes.status = ImplBuildStatus.Released
        changes.magicBytes = rec.magicBytes ? rec.magicBytes : null

        changes.deletedAt = null
        changes.updatedAt = blockTime

        changeSet.set(id, changes)
      } else if (event.name == "OffchainComputingInfra.ImplBuildDeregistered") {
        let rec: { implId: number, implBuildVersion: number }
        if (ImplBuildDeregisteredEventV100.is(event)) {
          rec = ImplBuildDeregisteredEventV100.decode(event)
        } else {
          throw new Error('Unsupported spec')
        }

        const id = `${rec.implId}-${rec.implBuildVersion}`
        const changes: ImplBuildChanges = changeSet.get(id) || {
          id,
          implId: rec.implId,
          version: rec.implBuildVersion,
          status: ImplBuildStatus.Deregistered,
          createdAt: blockTime,
          updatedAt: blockTime
        }

        changes.status = ImplBuildStatus.Deregistered
        changes.deletedAt = blockTime
        changes.updatedAt = blockTime

        changeSet.set(id, changes)
      } else if (event.name == "OffchainComputingInfra.ImplBuildStatusUpdated") {
        let rec: { implId: number, implBuildVersion: number, status: v100.ImplBuildStatus }
        if (ImplBuildStatusUpdatedEventV100.is(event)) {
          rec = ImplBuildStatusUpdatedEventV100.decode(event)
        } else {
          throw new Error('Unsupported spec')
        }

        const id = `${rec.implId}-${rec.implBuildVersion}`
        const changes: ImplBuildChanges = changeSet.get(id) || {
          id,
          implId: rec.implId,
          version: rec.implBuildVersion,
          status: decodeImplBuildStatus(rec.status),
          createdAt: blockTime,
          updatedAt: blockTime
        }
        assert(!changes.deletedAt)

        changes.status = decodeImplBuildStatus(rec.status)

        changes.updatedAt = blockTime

        changeSet.set(id, changes)
      }
    }
  }

  return changeSet
}
