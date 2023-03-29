import {BatchContext, BatchProcessorItem, SubstrateBatchProcessor} from "@subsquid/substrate-processor"
import {Store} from "@subsquid/typeorm-store"
import * as config from "./config"

export const processor = new SubstrateBatchProcessor()
    .setDataSource({
        // Lookup archive by the network name in the Subsquid registry
        //archive: lookupArchive("kusama", {release: "FireSquid"})

        // Use archive created by archive/docker-compose.yml
        archive: config.archiveGatewayEndpoint,
        chain: config.chainNodeRPCEndpoint,
    })
    // Balances
    .addEvent("Balances.Transfer")
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
    .addEvent("OffchainComputing.PoolStashAccountUpdated")
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

export type Item = BatchProcessorItem<typeof processor>
export type Ctx = BatchContext<Store, Item>
