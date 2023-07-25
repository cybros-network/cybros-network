import assert from "assert"
import { TypeormDatabase } from "@subsquid/typeorm-store"
import { type Context, processor } from "./processor"

import {
    preprocessImplsEvents,
    preprocessImplBuildsEvents,
    preprocessWorkersEvents,
    preprocessPoolsEvents, preprocessTaskPoliciesEvents, preprocessPoolWorkersEvents, preprocessTasksEvents,
} from "./processor_helpers"
import {
    AccountsManager,
    ImplsManager,
    ImplBuildsManager,
    WorkersManager, WorkerEventsManager,
    PoolsManager, TaskPoliciesManager, PoolWorkersManager,
    TasksManager,
} from "./entity_managers"

import { WorkerStatus, Pool, Worker, Impl, ImplBuild, Account } from "./model"
import { Equal, IsNull, In } from "typeorm"

const database = new TypeormDatabase();

processor.run(database, async (ctx: Context) => {
    // Preprocess events
    const implsChangeSet = preprocessImplsEvents(ctx)
    const implBuildsChangeSet = preprocessImplBuildsEvents(ctx)
    const workersChangeSet = preprocessWorkersEvents(ctx)
    const poolsChangeSet = preprocessPoolsEvents(ctx)
    const taskPoliciesChangeSet = preprocessTaskPoliciesEvents(ctx)
    const poolWorkersChangeSet = preprocessPoolWorkersEvents(ctx)
    const tasksChangeSet = preprocessTasksEvents(ctx)

    // Initialize entity managers
    const accountsManager = new AccountsManager().init(ctx)
    const implsManager = new ImplsManager().init(ctx)
    const implBuildsManager = new ImplBuildsManager().init(ctx)
    const workersManager = new WorkersManager().init(ctx)
    const workerEventsManager = new WorkerEventsManager().init(ctx)
    const poolsManager = new PoolsManager().init(ctx)
    const taskPoliciesManager = new TaskPoliciesManager().init(ctx)
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
    for (let [_id, changes] of taskPoliciesChangeSet) {
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
    for (let [id, _changes] of taskPoliciesChangeSet) {
        taskPoliciesManager.addPrefetchItemId(id)
    }
    for (let [_id, changes] of tasksChangeSet) {
        if (changes.policyId) {
            assert(changes.poolId)
            taskPoliciesManager.addPrefetchItemId(`${changes.poolId}-${changes.policyId}`)
        }
    }
    await taskPoliciesManager.prefetchEntities()

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
            if (!impl.implId) {
                assert(changes.implId)
                impl.implId = changes.implId
            }
            if (!impl.ownerAddress) {
                assert(changes.owner)

                impl.ownerAddress = changes.owner
                impl._owner = await accountsManager.getOrCreate(changes.owner)
            }
            if (changes.attestationMethod) {
                impl.attestationMethod = changes.attestationMethod
            }
            if (changes.deploymentScope) {
                impl.deploymentScope = changes.deploymentScope
            }
            if (changes.metadata !== undefined) {
                impl.metadata = changes.metadata
            }
            if (changes.createdAt) {
                impl.createdAt = changes.createdAt
            }
            if (changes.deletedAt !== undefined) {
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
            implBuild.status = changes.status
            if (changes.magicBytes !== undefined) {
                implBuild.magicBytes = changes.magicBytes
            }
            if (changes.createdAt) {
                implBuild.createdAt = changes.createdAt!
            }
            if (changes.deletedAt !== undefined) {
                implBuild.deletedAt = changes.deletedAt
            }
            implBuild.updatedAt = changes.updatedAt
        })
    }
    await implBuildsManager.saveAll()

    // Process workers' changeset
    for (let [id, changes] of workersChangeSet) {
        await workersManager.upsert(id, async (worker) => {
            if (changes.owner) {
                worker.ownerAddress = changes.owner
                worker._owner = await accountsManager.getOrCreate(changes.owner)
            }
            if (changes.implId) {
                worker.implId = changes.implId
                worker._impl = (await implsManager.get(changes.implId.toString()))!
            }
            if (changes.implBuildVersion !== undefined) {
                assert(worker.implId)

                worker.implBuildVersion = changes.implBuildVersion
                worker._implBuild = (await implBuildsManager.get(`${worker.implId}-${changes.implBuildVersion}`))!
            }
            if (changes.implSpecVersion !== undefined) {
                worker.implSpecVersion = changes.implSpecVersion
            }
            if (changes.attestationMethod !== undefined) {
                worker.attestationMethod = changes.attestationMethod
            }
            if (changes.attestationExpiresAt !== undefined) {
                worker.attestationExpiresAt = changes.attestationExpiresAt
            }
            if (changes.lastAttestedAt !== undefined) {
                worker.lastAttestedAt = changes.lastAttestedAt
            }
            if (changes.lastHeartbeatReceivedAt !== undefined) {
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
            if (!worker.createdAt) {
                worker.createdAt = changes.createdAt
            }
            if (changes.deletedAt !== undefined) {
                worker.status = WorkerStatus.Deregistered
                worker.deletedAt = changes.deletedAt
            }
            worker.updatedAt = changes.updatedAt

            for (let e of changes.events) {
                await workerEventsManager.create(e.id, async (event) => {
                    event._worker = worker

                    event.kind = e.kind
                    event.payload = e.payload
                    event.blockNumber = e.blockNumber
                    event.blockTime = e.blockTime
                })
            }
        });
    }
    await accountsManager.saveAll()
    await workersManager.saveAll()
    await workerEventsManager.saveAll()

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
            if (changes.creatingTaskAvailability) {
                pool.creatingTaskAvailability = changes.creatingTaskAvailability
            }
            if (changes.metadata !== undefined) {
                pool.metadata = changes.metadata
            }
            if (changes.createdAt) {
                pool.createdAt = changes.createdAt
            }
            if (changes.deletedAt !== undefined) {
                pool.deletedAt = changes.deletedAt
            }
            pool.updatedAt = changes.updatedAt
        })
    }
    await accountsManager.saveAll()
    await poolsManager.saveAll()

    // Process create task policies' changeset
    for (let [id, changes] of taskPoliciesChangeSet) {
        await taskPoliciesManager.upsert(id, async (taskPolicy) => {
            if (changes.poolId) {
                taskPolicy._pool = (await poolsManager.get(changes.poolId.toString()))!
            }
            if (changes.policyId) {
                taskPolicy.policyId = changes.policyId
            }
            if (changes.availability) {
                taskPolicy.availability = changes.availability
            }
            if (changes.creatingTaskScope) {
                taskPolicy.creatingTaskScope = changes.creatingTaskScope
            }
            if (changes.startBlock) {
                taskPolicy.startBlock = changes.startBlock
            }
            if (changes.endBlock) {
                taskPolicy.endBlock = changes.endBlock
            }
            if (changes.createdAt) {
                taskPolicy.createdAt = changes.createdAt
            }
            if (changes.deletedAt !== undefined) {
                taskPolicy.deletedAt = changes.deletedAt
            }
            taskPolicy.updatedAt = changes.updatedAt
        })
    }
    await taskPoliciesManager.saveAll()

    // Process pool workers' changeset
    for (let [id, changes] of poolWorkersChangeSet) {
        await poolWorkersManager.upsert(id, async (poolWorker) => {
            if (!poolWorker._pool) {
                assert(changes.poolId)

                poolWorker.poolId = changes.poolId
                poolWorker._pool = (await poolsManager.get(changes.poolId.toString()))!
            }
            if (!poolWorker._worker) {
                assert(changes.worker)

                poolWorker.workerAddress = changes.worker
                poolWorker._worker = (await workersManager.get(changes.worker))!
            }
            if (!poolWorker.createdAt) {
                poolWorker.createdAt = changes.createdAt
            }
            if (changes.deletedAt !== undefined) {
                poolWorker.deletedAt = changes.deletedAt
            }
            poolWorker.updatedAt = changes.updatedAt

            for (let e of changes.workerEvents) {
                await workerEventsManager.create(e.id, async (event) => {
                    event._worker = poolWorker._worker

                    event.kind = e.kind
                    event.payload = e.payload
                    event.blockNumber = e.blockNumber
                    event.blockTime = e.blockTime
                })
            }
        })
    }
    await poolWorkersManager.saveAll()
    await workerEventsManager.saveAll()

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
                task._policy = (await taskPoliciesManager.get(`${changes.poolId}-${changes.policyId}`))!
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
            if (changes.endedAt) {
                task.endedAt = changes.endedAt
            }
            if (changes.createdAt) {
                task.createdAt = changes.createdAt
            }
            if (changes.deletedAt !== undefined) {
                task.deletedAt = changes.deletedAt
            }
            task.updatedAt = changes.updatedAt
        })
    }
    await accountsManager.saveAll()
    await tasksManager.saveAll()

    // Update stats

    await ctx.store.find(Worker, {
        relations: {
            servingPools: true,
        },
        where: {
            servingPools: {
                deletedAt: IsNull(),
                poolId: In(
                    Array.from(poolWorkersChangeSet.values())
                        .filter(changes => changes.poolWorkerCounterChange != 0)
                        .map(changes => changes.poolId)
                )
            }
        }
    }).then(workers => workers.forEach(worker => workersManager.add(worker)))
    await ctx.store.find(Pool, {
        relations: {
            workers: true,
        },
        where: {
            workers: {
                deletedAt: IsNull(),
                workerAddress: In(
                    Array.from(poolWorkersChangeSet.values())
                        .filter(changes => changes.poolWorkerCounterChange != 0)
                        .map(changes => changes.worker)
                )
            }
        }
    }).then(pools => pools.forEach(pool => poolsManager.add(pool)))
    for (let [_id, changes] of poolWorkersChangeSet) {
        if (changes.poolWorkerCounterChange == 0) {
            continue
        }

        const worker = await workersManager.get(changes.worker)
        assert(worker)
        worker.poolsCount += changes.poolWorkerCounterChange

        const pool = await poolsManager.get(changes.poolId.toString())
        assert(pool)
        pool.workersCount += changes.poolWorkerCounterChange
    }
    await poolsManager.saveAll()
    await workersManager.saveAll()

    await ctx.store.find(Worker, {
        where: {
            id: In(
                Array.from(workersChangeSet.values())
                    .filter(changes => changes.onlineWorkerCounterChange != 0 || changes.registerWorkerCounterChange != 0)
                    .map(changes => changes.id)
            )
        }
    }).then(workers => workers.forEach(worker => workersManager.add(worker)))
    await ctx.store.find(Impl, {
        where: {
            id: In(
                Array.from(workersManager.entitiesMap.values())
                    .filter(worker => worker.implId)
                    .map(worker => worker.implId)
            )
        }
    }).then(impls => impls.forEach(impl => implsManager.add(impl)))
    await ctx.store.find(ImplBuild, {
        where: {
            id: In(
                Array.from(workersManager.entitiesMap.values())
                    .filter(worker => worker.implId && worker.implBuildVersion)
                    .map(worker => `${worker.implId}-${worker.implBuildVersion}`)
            )
        }
    }).then(implBuilds => implBuilds.forEach(implBuild => implBuildsManager.add(implBuild)))
    await ctx.store.find(Account, {
        where: {
            id: In(
                Array.from(workersManager.entitiesMap.values())
                    .map(worker => worker.ownerAddress)
            )
        }
    }).then(accounts => accounts.forEach(account => accountsManager.add(account)))
    for (let [id, changes] of workersChangeSet) {
        const worker = await workersManager.get(id)
        assert(worker)

        if (changes.onlineWorkerCounterChange != 0) {
            assert(worker.implId)
            assert(worker.implBuildVersion)

            await implsManager.upsert(worker.implId.toString(), async (impl) => {
                impl.onlineWorkersCount += changes.onlineWorkerCounterChange
            })
            await implBuildsManager.upsert(`${worker.implId}-${worker.implBuildVersion}`, async (implBuild) => {
                implBuild.onlineWorkersCount += changes.onlineWorkerCounterChange
            })

            if (worker.poolsCount > 0) {
                const pools = await ctx.store.find(Pool, {
                    relations: {
                        workers: true,
                    },
                    where: {
                        workers: {
                            deletedAt: IsNull(),
                            workerAddress: Equal(worker.id)
                        }
                    }
                })
                for (let pool of pools) {
                    pool.onlineWorkersCount += changes.onlineWorkerCounterChange
                    poolsManager.add(pool)
                }
            }
        }

        if (changes.registerWorkerCounterChange != 0) {
            assert(worker.ownerAddress)
            await accountsManager.upsert(worker.ownerAddress, async (account) => {
                account.workersCount += changes.registerWorkerCounterChange
            })
        }
    }

    // // Save
    await accountsManager.saveAll()
    await implsManager.saveAll()
    await implBuildsManager.saveAll()
    await workersManager.saveAll()
    await workerEventsManager.saveAll()
    await poolsManager.saveAll()
    await taskPoliciesManager.saveAll()
    await poolWorkersManager.saveAll()
    await tasksManager.saveAll()
})