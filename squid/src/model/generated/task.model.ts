import {Entity as Entity_, Column as Column_, PrimaryColumn as PrimaryColumn_, ManyToOne as ManyToOne_, Index as Index_} from "typeorm"
import {Pool} from "./pool.model"
import {Account} from "./account.model"
import {TaskStatus} from "./_taskStatus"
import {TaskResult} from "./_taskResult"
import {Worker} from "./worker.model"

@Entity_()
export class Task {
    constructor(props?: Partial<Task>) {
        Object.assign(this, props)
    }

    @PrimaryColumn_()
    id!: string

    @Index_()
    @ManyToOne_(() => Pool, {nullable: true})
    pool!: Pool

    @Index_()
    @ManyToOne_(() => Account, {nullable: true})
    creator!: Account

    @Index_()
    @ManyToOne_(() => Account, {nullable: true})
    owner!: Account

    @Column_("int4", {nullable: false})
    ownerDeposit!: number

    @Column_("varchar", {length: 10, nullable: false})
    status!: TaskStatus

    @Column_("varchar", {length: 7, nullable: true})
    result!: TaskResult | undefined | null

    @Column_("timestamp with time zone", {nullable: false})
    expiresAt!: Date

    @Index_()
    @ManyToOne_(() => Account, {nullable: true})
    createdBy!: Account

    @Column_("timestamp with time zone", {nullable: false})
    createdAt!: Date

    @Index_()
    @ManyToOne_(() => Worker, {nullable: true})
    assignee!: Worker | undefined | null

    @Column_("timestamp with time zone", {nullable: true})
    assignedAt!: Date | undefined | null

    @Column_("timestamp with time zone", {nullable: true})
    processingAt!: Date | undefined | null

    @Column_("timestamp with time zone", {nullable: true})
    processedAt!: Date | undefined | null
}
