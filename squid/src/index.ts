import assert from "assert"
import { TypeormDatabase } from "@subsquid/typeorm-store"
import { type Context, processor } from "./processor"

import {AttestationMethod, ImplDeploymentPermission, WorkerStatus} from "./model"
import { preprocessWorkersEvents, preprocessImplsEvents } from "./processor_helpers"
import { AccountsManager, WorkersManager, ImplsManager } from "./entity_managers"

const database = new TypeormDatabase();

processor.run(database, async (ctx: Context) => {
    // Preprocess events
    const implsChangeSet = preprocessImplsEvents(ctx)
    const workersChangeSet = preprocessWorkersEvents(ctx)

    // Initialize entity managers
    const accountsManager = new AccountsManager()
    accountsManager.init(ctx)
    const implsManager = new ImplsManager()
    implsManager.init(ctx)
    const workersManager = new WorkersManager()
    workersManager.init(ctx)

    // Prefetch entities

    // Accounts
    for (let [_id, workerChanges] of workersChangeSet) {
        if (workerChanges.owner) {
            accountsManager.addPrefetchItemId(workerChanges.owner)
        }
    }
    for (let [_id, implChanges] of implsChangeSet) {
        if (implChanges.owner) {
            accountsManager.addPrefetchItemId(implChanges.owner)
        }
    }
    await accountsManager.prefetchEntities()

    // Impls
    for (let [_id, workerChanges] of workersChangeSet) {
        if (workerChanges.implId) {
            implsManager.addPrefetchItemId(workerChanges.implId)
        }
    }
    for (let [_id, implChanges] of implsChangeSet) {
        if (implChanges.id) {
            accountsManager.addPrefetchItemId(implChanges.id)
        }
    }
    await implsManager.prefetchEntities()

    // Workers
    for (let [_id, workerChanges] of workersChangeSet) {
        workersManager.addPrefetchItemId(workerChanges.id)
    }
    await workersManager.prefetchEntities()

    // Process impls' changeset
    for (let [id, implChanges] of implsChangeSet) {
        await implsManager.upsert(id, async (impl) => {
            if (implChanges.owner) {
                impl.owner = await accountsManager.getOrCreate(implChanges.owner)
            }
            if (implChanges.attestationMethod) {
                impl.attestationMethod = implChanges.attestationMethod
            }
            if (implChanges.deploymentPermission) {
                impl.deploymentPermission = implChanges.deploymentPermission
            }
            if (implChanges.oldestBuildVersion) {
                impl.oldestBuildVersion = implChanges.oldestBuildVersion
            }
            if (implChanges.newestBuildVersion) {
                impl.newestBuildVersion = implChanges.newestBuildVersion
            }
            if (implChanges.blockedBuildVersions) {
                impl.blockedBuildVersions = implChanges.blockedBuildVersions
            }
            if (implChanges.metadata !== undefined) {
                impl.metadata = implChanges.metadata
            }
            if (implChanges.createdAt) {
                impl.createdAt = implChanges.createdAt
            }
            if (implChanges.deletedAt) {
                impl.deletedAt = implChanges.deletedAt
            }
            impl.updatedAt = implChanges.updatedAt
        })
    }

    // Process workers' changeset
    for (let [id, workerChanges] of workersChangeSet) {
        await workersManager.upsert(id, async (worker) => {
            if (workerChanges.owner) {
                worker.owner = await accountsManager.getOrCreate(workerChanges.owner)
            }
            if (workerChanges.implId) {
                worker.impl = await implsManager.getOrCreate(workerChanges.implId)
            }
            if (workerChanges.implSpecVersion) {
                worker.implSpecVersion = workerChanges.implSpecVersion
            }
            if (workerChanges.implBuildVersion) {
                worker.implBuildVersion = workerChanges.implBuildVersion
            }
            if (workerChanges.attestationMethod) {
                worker.attestationMethod = workerChanges.attestationMethod
            }
            if (workerChanges.attestationExpiresAt) {
                worker.attestationExpiresAt = workerChanges.attestationExpiresAt
            }
            if (workerChanges.lastAttestedAt) {
                worker.lastAttestedAt = workerChanges.lastAttestedAt
            }
            if (workerChanges.lastHeartbeatReceivedAt) {
                worker.lastHeartbeatReceivedAt = workerChanges.lastHeartbeatReceivedAt
            }
            if (workerChanges.status) {
                worker.status = workerChanges.status

                if (workerChanges.status == WorkerStatus.Offline) {
                    assert(workerChanges.offlineAt)
                    assert(workerChanges.offlineReason)
                    worker.offlineAt = workerChanges.offlineAt
                    worker.offlineReason = workerChanges.offlineReason
                } else {
                    worker.offlineAt = null
                    worker.offlineReason = null
                }
            }
            if (workerChanges.createdAt) {
                worker.createdAt = workerChanges.createdAt
            }
            if (workerChanges.deletedAt) {
                worker.status = WorkerStatus.Deregistered
                worker.deletedAt = workerChanges.deletedAt
            }
            worker.updatedAt = workerChanges.updatedAt
        });
    }

    // Save
    await accountsManager.saveAll()
    await implsManager.saveAll()
    await workersManager.saveAll()
})
