import assert from "assert"
import { TypeormDatabase } from "@subsquid/typeorm-store"
import { type Context, processor } from "./processor"

import {
    preprocessImplsEvents,
    preprocessImplBuildsEvents,
    preprocessWorkersEvents,
    preprocessPoolsEvents, preprocessCreatingTaskPoliciesEvents, preprocessPoolWorkersEvents, preprocessTasksEvents,
} from "./processor_helpers"
import {
    AccountsManager,
    ImplsManager,
    ImplBuildsManager,
    WorkersManager, WorkerEventsManager,
    PoolsManager, CreatingTaskPoliciesManager, PoolWorkersManager,
    TasksManager,
} from "./entity_managers"

import { WorkerStatus } from "./model"

const database = new TypeormDatabase();

processor.run(database, async (ctx: Context) => {
    // Preprocess events
    const implsChangeSet = preprocessImplsEvents(ctx)
    const implBuildsChangeSet = preprocessImplBuildsEvents(ctx)
    const workersChangeSet = preprocessWorkersEvents(ctx)
    const poolsChangeSet = preprocessPoolsEvents(ctx)
    const creatingTaskPoliciesChangeSet = preprocessCreatingTaskPoliciesEvents(ctx)
    const poolWorkersChangeSet = preprocessPoolWorkersEvents(ctx)
    const tasksChangeSet = preprocessTasksEvents(ctx)

    // Initialize entity managers
    const accountsManager = new AccountsManager().init(ctx)
    const implsManager = new ImplsManager().init(ctx)
    const implBuildsManager = new ImplBuildsManager().init(ctx)
    const workersManager = new WorkersManager().init(ctx)
    const workerEventsManager = new WorkerEventsManager().init(ctx)
    const poolsManager = new PoolsManager().init(ctx)
    const creatingTaskPoliciesManager = new CreatingTaskPoliciesManager().init(ctx)
    const poolWorkersManager = new PoolWorkersManager().init(ctx)
    const tasksManager = new TasksManager().init(ctx)

    // Prefetch entities

    // Accounts
    for (let [_id, changes] of workersChangeSet) {
        if (changes.owner) {
            accountsManager.addPrefetchItemId(changes.owner)
        }
    }
    for (let [_id, changes] of implsChangeSet) {
        if (changes.owner) {
            accountsManager.addPrefetchItemId(changes.owner)
        }
    }
    for (let [_id, changes] of poolsChangeSet) {
        if (changes.owner) {
            accountsManager.addPrefetchItemId(changes.owner)
        }
    }
    for (let [_id, changes] of tasksChangeSet) {
        if (changes.owner) {
            accountsManager.addPrefetchItemId(changes.owner)
        }
        if (changes.destroyer) {
            accountsManager.addPrefetchItemId(changes.destroyer)
        }
    }
    await accountsManager.prefetchEntities()

    // Impls
    for (let [id, _changes] of implsChangeSet) {
        implsManager.addPrefetchItemId(id)
    }
    for (let [_id, changes] of implBuildsChangeSet) {
        implsManager.addPrefetchItemId(changes.implId.toString())
    }
    for (let [_id, changes] of workersChangeSet) {
        if (changes.implId) {
            implsManager.addPrefetchItemId(changes.implId.toString())
        }
    }
    for (let [_id, changes] of poolsChangeSet) {
        if (changes.implId) {
            implsManager.addPrefetchItemId(changes.implId.toString())
        }
    }
    await implsManager.prefetchEntities()

    // Impl builds
    for (let [id, _changes] of implBuildsChangeSet) {
        implBuildsManager.addPrefetchItemId(id)
    }
    for (let [_id, changes] of workersChangeSet) {
        if (changes.implId && changes.implBuildVersion) {
            implBuildsManager.addPrefetchItemId(`${changes.implId}-${changes.implBuildVersion}`)
        }
    }
    await implBuildsManager.prefetchEntities()

    // Workers
    for (let [id, _changes] of workersChangeSet) {
        workersManager.addPrefetchItemId(id)
    }
    for (let [_id, changes] of poolWorkersChangeSet) {
        workersManager.addPrefetchItemId(changes.worker)
    }
    for (let [_id, changes] of tasksChangeSet) {
        if (changes.assignee) {
            workersManager.addPrefetchItemId(changes.assignee)
        }
    }
    await workersManager.prefetchEntities()

    // Pools
    for (let [id, _changes] of poolsChangeSet) {
        poolsManager.addPrefetchItemId(id)
    }
    for (let [_id, changes] of creatingTaskPoliciesChangeSet) {
        if (changes.poolId) {
            poolsManager.addPrefetchItemId(changes.poolId.toString())
        }
    }
    for (let [_id, changes] of poolWorkersChangeSet) {
        poolsManager.addPrefetchItemId(changes.poolId.toString())
    }
    for (let [_id, changes] of tasksChangeSet) {
        if (changes.poolId) {
            poolsManager.addPrefetchItemId(changes.poolId.toString())
        }
    }
    await poolsManager.prefetchEntities()

    // Creating task policies
    for (let [id, _changes] of creatingTaskPoliciesChangeSet) {
        creatingTaskPoliciesManager.addPrefetchItemId(id)
    }
    for (let [_id, changes] of tasksChangeSet) {
        if (changes.policyId) {
            assert(changes.poolId)
            creatingTaskPoliciesManager.addPrefetchItemId(`${changes.poolId}-${changes.policyId}`)
        }
    }
    await creatingTaskPoliciesManager.prefetchEntities()

    // Pool workers
    for (let [id, _changes] of poolWorkersChangeSet) {
        poolWorkersManager.addPrefetchItemId(id)
    }
    await poolWorkersManager.prefetchEntities()

    // Tasks
    for (let [id, _changes] of tasksChangeSet) {
        tasksManager.addPrefetchItemId(id)
    }
    await tasksManager.prefetchEntities()

    // Process

    // Process impls' changeset
    for (let [id, changes] of implsChangeSet) {
        await implsManager.upsert(id, async (impl) => {
            if (!impl.ownerAddress) {
                assert(changes.owner)

                impl.ownerAddress = changes.owner
                impl._owner = await accountsManager.getOrCreate(changes.owner)
            }
            if (changes.attestationMethod) {
                impl.attestationMethod = changes.attestationMethod
            }
            if (changes.deploymentPermission) {
                impl.deploymentPermission = changes.deploymentPermission
            }
            if (changes.oldestBuildVersion) {
                impl.oldestBuildVersion = changes.oldestBuildVersion
            }
            if (changes.newestBuildVersion) {
                impl.newestBuildVersion = changes.newestBuildVersion
            }
            if (changes.blockedBuildVersions) {
                impl.blockedBuildVersions = changes.blockedBuildVersions
            }
            if (changes.metadata !== undefined) {
                impl.metadata = changes.metadata
            }
            if (changes.createdAt) {
                impl.createdAt = changes.createdAt
            }
            if (changes.deletedAt) {
                impl.deletedAt = changes.deletedAt
            }
            impl.updatedAt = changes.updatedAt
        })
    }
    await accountsManager.saveAll()
    await implsManager.saveAll()

    // Process impl builds' changeset
    for (let [id, changes] of implBuildsChangeSet) {
        await implBuildsManager.upsert(id, async (implBuild) => {
            if (!implBuild.implId) {
                assert(changes.implId)

                implBuild.implId = changes.implId
                implBuild._impl = (await implsManager.get(changes.implId.toString()))!
            }
            if (!implBuild.version) {
                assert(changes.version)

                implBuild.version = changes.version
            }
            if (!implBuild.magicBytes) {
                assert(changes.magicBytes)

                implBuild.magicBytes = changes.magicBytes
            }
            if (changes.createdAt) {
                implBuild.createdAt = changes.createdAt!
            }
            if (changes.deletedAt) {
                implBuild.deletedAt = changes.deletedAt
            }
        })
    }
    await implBuildsManager.saveAll()

    // Process workers' changeset
    for (let [id, changes] of workersChangeSet) {
        await workersManager.upsert(id, async (worker) => {
            if (!worker.ownerAddress) {
                assert(changes.owner)

                worker.ownerAddress = changes.owner
                worker._owner = await accountsManager.getOrCreate(changes.owner)
            }
            if (changes.implId) {
                worker.implId = changes.implId
                worker._impl = await implsManager.get(changes.implId.toString())
            }
            if (changes.implBuildVersion) {
                assert(worker.implId)

                worker.implBuildVersion = changes.implBuildVersion
                worker._implBuild = (await implBuildsManager.get(`${worker.implId}-${changes.implBuildVersion}`))!
            }
            if (changes.implSpecVersion) {
                worker.implSpecVersion = changes.implSpecVersion
            }
            if (changes.attestationMethod) {
                worker.attestationMethod = changes.attestationMethod
            }
            if (changes.attestationExpiresAt !== undefined) {
                worker.attestationExpiresAt = changes.attestationExpiresAt
            }
            if (changes.lastAttestedAt) {
                worker.lastAttestedAt = changes.lastAttestedAt
            }
            if (changes.lastHeartbeatReceivedAt) {
                worker.lastHeartbeatReceivedAt = changes.lastHeartbeatReceivedAt
            }
            if (changes.status) {
                worker.status = changes.status

                if (changes.status == WorkerStatus.Offline) {
                    assert(changes.offlineAt)
                    assert(changes.offlineReason)

                    worker.offlineAt = changes.offlineAt
                    worker.offlineReason = changes.offlineReason
                } else {
                    worker.offlineAt = null
                    worker.offlineReason = null
                }
            }
            if (changes.createdAt) {
                worker.createdAt = changes.createdAt
            }
            if (changes.deletedAt) {
                worker.status = WorkerStatus.Deregistered
                worker.deletedAt = changes.deletedAt
            }
            worker.updatedAt = changes.updatedAt

            if (changes.onlineWorkerCounterChange != 0) {
                assert(worker.implId)
                assert(worker.implBuildVersion)

                await implsManager.upsert(worker.implId.toString(), async (impl) => {
                    impl.workersCount += changes.onlineWorkerCounterChange
                })
                await implBuildsManager.upsert(`${worker.implId}-${worker.implBuildVersion}`, async (implBuild) => {
                    implBuild.workersCount += changes.onlineWorkerCounterChange
                })
            }
        });
    }
    await accountsManager.saveAll()
    await workersManager.saveAll()
    await implsManager.saveAll()
    await implBuildsManager.saveAll()

    // Process pools' changeset
    for (let [id, changes] of poolsChangeSet) {
        await poolsManager.upsert(id, async (pool) => {
            if (!pool.ownerAddress) {
                assert(changes.owner)

                pool.ownerAddress = changes.owner
                pool._owner = await accountsManager.getOrCreate(changes.owner)
            }
            if (!pool.poolId) {
                assert(changes.poolId)

                pool.poolId = changes.poolId
            }
            if (!pool.implId) {
                assert(changes.implId)

                pool.implId = changes.implId
                pool._impl = (await implsManager.get(changes.implId.toString()))!
            }
            if (changes.creatingTaskAbility) {
                pool.creatingTaskAbility = changes.creatingTaskAbility
            }
            if (changes.metadata !== undefined) {
                pool.metadata = changes.metadata
            }
            if (changes.createdAt) {
                pool.createdAt = changes.createdAt
            }
            if (changes.deletedAt) {
                pool.deletedAt = changes.deletedAt
            }
            pool.updatedAt = changes.updatedAt
        })
    }
    await accountsManager.saveAll()
    await poolsManager.saveAll()

    // Process create task policies' changeset
    for (let [id, changes] of creatingTaskPoliciesChangeSet) {
        await creatingTaskPoliciesManager.upsert(id, async (createTaskPolicy) => {
            if (changes.poolId) {
                createTaskPolicy._pool = (await poolsManager.get(changes.poolId.toString()))!
            }
            if (changes.policyId) {
                createTaskPolicy.policyId = changes.policyId
            }
            if (changes.permission) {
                createTaskPolicy.permission = changes.permission
            }
            if (changes.startBlock) {
                createTaskPolicy.startBlock = changes.startBlock
            }
            if (changes.endBlock) {
                createTaskPolicy.endBlock = changes.endBlock
            }
            if (changes.createdAt) {
                createTaskPolicy.createdAt = changes.createdAt
            }
            if (changes.deletedAt) {
                createTaskPolicy.deletedAt = changes.deletedAt
            }
        })
    }
    await creatingTaskPoliciesManager.saveAll()

    // Process pool workers' changeset
    for (let [id, changes] of poolWorkersChangeSet) {
        await poolWorkersManager.upsert(id, async (poolWorker) => {
            if (!poolWorker._pool) {
                poolWorker._pool = (await poolsManager.get(changes.poolId.toString()))!
            }
            if (!poolWorker._worker) {
                poolWorker._worker = (await workersManager.get(changes.worker))!
            }
            if (changes.createdAt) {
                poolWorker.createdAt = changes.createdAt
            }
            if (changes.deletedAt) {
                poolWorker.deletedAt = changes.deletedAt
            }
        })
    }
    await poolWorkersManager.saveAll()

    // Process tasks' changeset
    for (let [id, changes] of tasksChangeSet) {
        await tasksManager.upsert(id, async (task) => {
            if (!task.taskId) {
                task.taskId = changes.taskId
            }
            if (!task.poolId) {
                assert(changes.poolId)

                task.poolId = changes.poolId
                task._pool = (await poolsManager.get(changes.poolId.toString()))!
            }
            if (!task.policyId) {
                assert(changes.policyId)
                task.policyId = changes.policyId
                task._policy = (await creatingTaskPoliciesManager.get(`${changes.poolId}-${changes.policyId}`))!
            }
            if (changes.owner) {
                task.ownerAddress = changes.owner
                task._owner = await accountsManager.getOrCreate(changes.owner)
            }
            if (changes.assignee) {
                task.assigneeAddress = changes.assignee
                task._assignee = (await workersManager.get(changes.assignee))!
            }
            if (changes.destroyer) {
                task.destroyerAddress = changes.destroyer
                task._destroyer = (await accountsManager.get(changes.destroyer))!
            }
            if (changes.implSpecVersion) {
                task.implSpecVersion = changes.implSpecVersion
            }
            if (changes.status) {
                task.status = changes.status
            }
            if (changes.result) {
                task.result = changes.result
            }
            if (changes.input !== undefined) {
                task.input = changes.input
            }
            if (changes.output !== undefined) {
                task.output = changes.output
            }
            if (changes.proof !== undefined) {
                task.proof = changes.proof
            }
            if (changes.expiresAt) {
                task.expiresAt = changes.expiresAt
            }
            if (changes.assignedAt !== undefined) {
                task.assignedAt = changes.assignedAt
            }
            if (changes.processingAt) {
                task.processingAt = changes.processingAt
            }
            if (changes.processedAt) {
                task.processedAt = changes.processedAt
            }
            if (changes.createdAt) {
                task.createdAt = changes.createdAt
            }
            if (changes.deletedAt) {
                task.deletedAt = changes.deletedAt
            }

            task.updatedAt = changes.updatedAt
        })
    }
    await accountsManager.saveAll()
    await tasksManager.saveAll()

    // Save
    await accountsManager.saveAll()
    await implsManager.saveAll()
    await implBuildsManager.saveAll()
    await workersManager.saveAll()
    await workerEventsManager.saveAll()
    await poolsManager.saveAll()
    await creatingTaskPoliciesManager.saveAll()
    await poolWorkersManager.saveAll()
    await tasksManager.saveAll()
})
