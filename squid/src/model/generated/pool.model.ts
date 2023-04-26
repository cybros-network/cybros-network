import {Entity as Entity_, Column as Column_, PrimaryColumn as PrimaryColumn_, ManyToOne as ManyToOne_, Index as Index_, OneToMany as OneToMany_} from "typeorm"
import {Account} from "./account.model"
import {Impl} from "./impl.model"
import {PoolWorkers} from "./poolWorkers.model"
import {TaskPolicy} from "./taskPolicy.model"
import {Task} from "./task.model"

@Entity_()
export class Pool {
    constructor(props?: Partial<Pool>) {
        Object.assign(this, props)
    }

    @PrimaryColumn_()
    id!: string

    @Column_("int4", {nullable: false})
    poolId!: number

    @Index_()
    @ManyToOne_(() => Account, {nullable: true})
    _owner!: Account

    @Column_("text", {nullable: false})
    ownerAddress!: string

    @Index_()
    @ManyToOne_(() => Impl, {nullable: true})
    _impl!: Impl

    @Column_("int4", {nullable: false})
    implId!: number

    @Column_("bool", {nullable: false})
    creatingTaskAvailability!: boolean

    @Column_("bytea", {nullable: true})
    metadata!: Uint8Array | undefined | null

    @Column_("int4", {nullable: false})
    workersCount!: number

    @Column_("int4", {nullable: false})
    onlineWorkersCount!: number

    @Column_("int4", {nullable: false})
    pendingTasksCount!: number

    @Column_("int4", {nullable: false})
    processingTasksCount!: number

    @Column_("int4", {nullable: false})
    createdTasksCount!: number

    @Column_("int4", {nullable: false})
    successfulTasksCount!: number

    @Column_("int4", {nullable: false})
    failedTasksCount!: number

    @Column_("int4", {nullable: false})
    erroredTasksCount!: number

    @Column_("timestamp with time zone", {nullable: false})
    createdAt!: Date

    @Column_("timestamp with time zone", {nullable: false})
    updatedAt!: Date

    @Column_("timestamp with time zone", {nullable: true})
    deletedAt!: Date | undefined | null

    @OneToMany_(() => PoolWorkers, e => e._pool)
    workers!: PoolWorkers[]

    @OneToMany_(() => TaskPolicy, e => e._pool)
    taskPolicies!: TaskPolicy[]

    @OneToMany_(() => Task, e => e._pool)
    tasks!: Task[]
}
