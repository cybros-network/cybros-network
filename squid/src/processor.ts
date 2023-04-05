import { BatchContext, BatchProcessorItem, SubstrateBatchProcessor } from "@subsquid/substrate-processor"
import { Store } from "@subsquid/typeorm-store"
import config from "./config"

export const processor = new SubstrateBatchProcessor()
    .setDataSource(config.dataSource)
    // OffchainComputingWorkers
    .addEvent("OffchainComputingWorkers.Registered")
    .addEvent("OffchainComputingWorkers.Deregistered")
    .addEvent("OffchainComputingWorkers.Online")
    .addEvent("OffchainComputingWorkers.RequestingOffline")
    .addEvent("OffchainComputingWorkers.Offline")
    .addEvent("OffchainComputingWorkers.HeartbeatReceived")
    .addEvent("OffchainComputingWorkers.AttestationRefreshed")
    .addEvent("OffchainComputingWorkers.WorkerImplPermissionUpdated")
    .addEvent("OffchainComputingWorkers.WorkerImplPermissionRemoved")
    .addEvent("OffchainComputingWorkers.WorkerImplHashUpdated")
    .addEvent("OffchainComputingWorkers.WorkerImplHashRemoved")
    // OffchainComputing
    .addEvent("OffchainComputing.PoolCreated")
    .addEvent("OffchainComputing.PoolDestroyed")
    .addEvent("OffchainComputing.PoolMetadataUpdated")
    .addEvent("OffchainComputing.PoolMetadataRemoved")
    .addEvent("OffchainComputing.PoolCreatingTaskAbilityEnabled")
    .addEvent("OffchainComputing.PoolCreatingTaskAbilityDisabled")
    .addEvent("OffchainComputing.CreatingTaskPolicyCreated")
    .addEvent("OffchainComputing.CreatingTaskPolicyDestroyed")
    .addEvent("OffchainComputing.WorkerAdded")
    .addEvent("OffchainComputing.WorkerRemoved")
    .addEvent("OffchainComputing.TaskCreated")
    .addEvent("OffchainComputing.TaskDestroyed")
    .addEvent("OffchainComputing.TaskAssigned")
    .addEvent("OffchainComputing.TaskReleased")
    .addEvent("OffchainComputing.TaskStatusUpdated")
    .addEvent("OffchainComputing.TaskResultUpdated")

type Item = BatchProcessorItem<typeof processor>
export type Context = BatchContext<Store, Item>
