import { BatchContext, BatchProcessorItem, SubstrateBatchProcessor } from "@subsquid/substrate-processor"
import { Store } from "@subsquid/typeorm-store"
import config from "./config"

export const processor = new SubstrateBatchProcessor()
    .setDataSource(config.dataSource)
    // OffchainComputingWorkers
    .addEvent("OffchainComputingWorkers.WorkerRegistered")
    .addEvent("OffchainComputingWorkers.WorkerDeregistered")
    .addEvent("OffchainComputingWorkers.WorkerOnline")
    .addEvent("OffchainComputingWorkers.WorkerRequestingOffline")
    .addEvent("OffchainComputingWorkers.WorkerOffline")
    .addEvent("OffchainComputingWorkers.WorkerHeartbeatReceived")
    .addEvent("OffchainComputingWorkers.WorkerAttestationRefreshed")
    .addEvent("OffchainComputingWorkers.ImplRegistered")
    .addEvent("OffchainComputingWorkers.ImplDeregistered")
    .addEvent("OffchainComputingWorkers.ImplDeploymentPermissionUpdated")
    .addEvent("OffchainComputingWorkers.ImplMetadataUpdated")
    .addEvent("OffchainComputingWorkers.ImplMetadataRemoved")
    .addEvent("OffchainComputingWorkers.ImplBuildRestrictionUpdated")
    .addEvent("OffchainComputingWorkers.ImplBuildRegistered")
    .addEvent("OffchainComputingWorkers.ImplBuildDeregistered")
    .addEvent("OffchainComputingWorkers.ImplBuildStatusUpdated")
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
