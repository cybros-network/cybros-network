import assert from "assert"
import { TypeormDatabase } from "@subsquid/typeorm-store"
import { type Context, processor} from "./processor"

import { WorkerStatus } from "./model"
import { preprocessWorkersEvents } from "./processor_helpers"
import { AccountsManager, WorkersManager } from "./entity_managers"

const database = new TypeormDatabase();

processor.run(database, async (ctx: Context) => {
    // Preprocess events
    const workersChangeSet = preprocessWorkersEvents(ctx)

    // Initialize entity managers
    const accountsManager = new AccountsManager()
    accountsManager.init(ctx)
    const workersManager = new WorkersManager()
    workersManager.init(ctx)

    // Prefetch entities
    // Prefetch worker owners
    for (let [_id, workerChanges] of workersChangeSet) {
        if (workerChanges.owner) {
            accountsManager.addPrefetchItemId(workerChanges.owner)
        }
    }
    await accountsManager.prefetchEntities()

    // Prefetch workers
    for (let [_id, workerChanges] of workersChangeSet) {
        workersManager.addPrefetchItemId(workerChanges.id)
    }
    await workersManager.prefetchEntities()

    // Process workers' changeset
    for (let [id, workerChanges] of workersChangeSet) {
        await workersManager.upsert(id, async (worker) => {
            if (workerChanges.deregistered) {
                worker.status = WorkerStatus.Deregistered
                worker.implName = null
                worker.implVersion = null
                worker.attestationMethod = null
                worker.lastAttestedAt = null
                worker.lastHeartbeatReceivedAt = null
                worker.offlineAt = null
                worker.offlineReason = null
            }

            if (workerChanges.owner) {
                worker.owner = await accountsManager.getOrCreate(workerChanges.owner)
            }
            if (workerChanges.implName) {
                worker.implName = workerChanges.implName
            }
            if (workerChanges.implVersion) {
                worker.implVersion = workerChanges.implVersion
            }
            if (workerChanges.attestationMethod) {
                worker.attestationMethod = workerChanges.attestationMethod
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
            worker.lastUpdatedBlockNumber = workerChanges.lastUpdatedBlockNumber
        });
    }

    // Save
    await accountsManager.saveAll()
    await workersManager.saveAll()
})
