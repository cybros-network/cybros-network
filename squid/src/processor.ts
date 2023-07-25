import {
    BatchContext,
    BatchProcessorItem,
    SubstrateBatchProcessor
} from "@subsquid/substrate-processor"
import {Store} from "@subsquid/typeorm-store"
import config from "./config"

export const processor = new SubstrateBatchProcessor()
    .setDataSource(config.dataSource)
    // OffchainComputingWorkers
    .addEvent("OffchainComputingWorkers.WorkerRegistered")
    .addEvent("OffchainComputingWorkers.WorkerDeregistered")
    .addEvent("OffchainComputingWorkers.WorkerOnline")
    .addEvent("OffchainComputingWorkers.WorkerUnresponsive")
    .addEvent("OffchainComputingWorkers.WorkerRequestingOffline")
    .addEvent("OffchainComputingWorkers.WorkerOffline")
    .addEvent("OffchainComputingWorkers.WorkerHeartbeatReceived")
    .addEvent("OffchainComputingWorkers.WorkerAttestationRefreshed")
    .addEvent("OffchainComputingWorkers.ImplRegistered")
    .addEvent("OffchainComputingWorkers.ImplDeregistered")
    .addEvent("OffchainComputingWorkers.ImplDeploymentScopeUpdated")
    .addEvent("OffchainComputingWorkers.ImplMetadataUpdated")
    .addEvent("OffchainComputingWorkers.ImplMetadataRemoved")
    .addEvent("OffchainComputingWorkers.ImplBuildRegistered")
    .addEvent("OffchainComputingWorkers.ImplBuildDeregistered")
    .addEvent("OffchainComputingWorkers.ImplBuildStatusUpdated")
    // OffchainComputing
    .addEvent("OffchainComputing.PoolCreated")
    .addEvent("OffchainComputing.PoolDestroyed")
    .addEvent("OffchainComputing.PoolMetadataUpdated")
    .addEvent("OffchainComputing.PoolMetadataRemoved")
    .addEvent("OffchainComputing.PoolSettingsUpdated")
    .addEvent("OffchainComputing.JobPolicyCreated")
    .addEvent("OffchainComputing.JobPolicyDestroyed")
    .addEvent("OffchainComputing.JobPolicyEnablementUpdated")
    .addEvent("OffchainComputing.WorkerProvisioned")
    .addEvent("OffchainComputing.WorkerRevoked")
    .addEvent("OffchainComputing.WorkerSubscribed")
    .addEvent("OffchainComputing.WorkerUnsubscribed")
    .addEvent("OffchainComputing.JobCreated")
    .addEvent("OffchainComputing.JobDestroyed")
    .addEvent("OffchainComputing.JobAssigned")
    .addEvent("OffchainComputing.JobReleased")
    .addEvent("OffchainComputing.JobStatusUpdated")
    .addEvent("OffchainComputing.JobResultUpdated")

type Item = BatchProcessorItem<typeof processor>
export type Context = BatchContext<Store, Item>
