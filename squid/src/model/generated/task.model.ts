import {Entity as Entity_, Column as Column_, PrimaryColumn as PrimaryColumn_, ManyToOne as ManyToOne_, Index as Index_, OneToMany as OneToMany_} from "typeorm"
import {Pool} from "./pool.model"
import {TaskPolicy} from "./taskPolicy.model"
import {Account} from "./account.model"
import {Worker} from "./worker.model"
import {TaskStatus} from "./_taskStatus"
import {TaskResult} from "./_taskResult"
import {TaskEvent} from "./taskEvent.model"

@Entity_()
export class Task {
    constructor(props?: Partial<Task>) {
        Object.assign(this, props)
    }

    @PrimaryColumn_()
    id!: string

    @Column_("int4", {nullable: false})
    taskId!: number

    @Index_()
    @ManyToOne_(() => Pool, {nullable: true})
    _pool!: Pool

    @Column_("int4", {nullable: false})
    poolId!: number

    @Index_()
    @ManyToOne_(() => TaskPolicy, {nullable: true})
    _policy!: TaskPolicy

    @Column_("int4", {nullable: false})
    policyId!: number

    @Index_()
    @ManyToOne_(() => Account, {nullable: true})
    _owner!: Account

    @Column_("text", {nullable: false})
    ownerAddress!: string

    @Index_()
    @ManyToOne_(() => Worker, {nullable: true})
    _assignee!: Worker | undefined | null

    @Column_("text", {nullable: true})
    assigneeAddress!: string | undefined | null

    @Index_()
    @ManyToOne_(() => Account, {nullable: true})
    _destroyer!: Account | undefined | null

    @Column_("text", {nullable: true})
    destroyerAddress!: string | undefined | null

    @Index_()
    @Column_("varchar", {length: 10, nullable: false})
    status!: TaskStatus

    @Column_("varchar", {length: 7, nullable: true})
    result!: TaskResult | undefined | null

    @Column_("int4", {nullable: false})
    implSpecVersion!: number

    @Column_("bytea", {nullable: true})
    input!: Uint8Array | undefined | null

    @Column_("bytea", {nullable: true})
    output!: Uint8Array | undefined | null

    @Column_("bytea", {nullable: true})
    proof!: Uint8Array | undefined | null

    @Column_("timestamp with time zone", {nullable: false})
    expiresAt!: Date

    @Column_("timestamp with time zone", {nullable: true})
    assignedAt!: Date | undefined | null

    @Column_("timestamp with time zone", {nullable: true})
    processingAt!: Date | undefined | null

    @Column_("timestamp with time zone", {nullable: true})
    endedAt!: Date | undefined | null

    @Column_("timestamp with time zone", {nullable: false})
    createdAt!: Date

    @Column_("timestamp with time zone", {nullable: false})
    updatedAt!: Date

    @Column_("timestamp with time zone", {nullable: true})
    deletedAt!: Date | undefined | null

    @OneToMany_(() => TaskEvent, e => e._task)
    events!: TaskEvent[]
}
