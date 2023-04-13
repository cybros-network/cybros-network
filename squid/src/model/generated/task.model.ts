import {Entity as Entity_, Column as Column_, PrimaryColumn as PrimaryColumn_, ManyToOne as ManyToOne_, Index as Index_} from "typeorm"
import {Pool} from "./pool.model"
import {CreatingTaskPolicy} from "./creatingTaskPolicy.model"
import {Account} from "./account.model"
import {Worker} from "./worker.model"
import {TaskStatus} from "./_taskStatus"
import {TaskResult} from "./_taskResult"

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
    pool!: Pool

    @Index_()
    @ManyToOne_(() => CreatingTaskPolicy, {nullable: true})
    policy!: CreatingTaskPolicy

    @Index_()
    @ManyToOne_(() => Account, {nullable: true})
    owner!: Account

    @Index_()
    @ManyToOne_(() => Worker, {nullable: true})
    assignee!: Worker | undefined | null

    @Index_()
    @ManyToOne_(() => Account, {nullable: true})
    destroyer!: Account | undefined | null

    @Index_()
    @Column_("varchar", {length: 10, nullable: false})
    status!: TaskStatus

    @Column_("varchar", {length: 7, nullable: true})
    result!: TaskResult | undefined | null

    @Column_("int4", {nullable: false})
    implSpecVersion!: number

    @Column_("text", {nullable: true})
    input!: string | undefined | null

    @Column_("text", {nullable: true})
    output!: string | undefined | null

    @Column_("text", {nullable: true})
    proof!: string | undefined | null

    @Column_("timestamp with time zone", {nullable: false})
    expiresAt!: Date

    @Column_("timestamp with time zone", {nullable: true})
    assignedAt!: Date | undefined | null

    @Column_("timestamp with time zone", {nullable: true})
    processingAt!: Date | undefined | null

    @Column_("timestamp with time zone", {nullable: true})
    processedAt!: Date | undefined | null

    @Column_("timestamp with time zone", {nullable: false})
    createdAt!: Date

    @Column_("timestamp with time zone", {nullable: false})
    updatedAt!: Date

    @Column_("timestamp with time zone", {nullable: true})
    deletedAt!: Date | undefined | null
}
