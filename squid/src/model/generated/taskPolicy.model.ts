import {Entity as Entity_, Column as Column_, PrimaryColumn as PrimaryColumn_, ManyToOne as ManyToOne_, Index as Index_} from "typeorm"
import {Pool} from "./pool.model"
import {ApplicableScope} from "./_applicableScope"

@Entity_()
export class TaskPolicy {
    constructor(props?: Partial<TaskPolicy>) {
        Object.assign(this, props)
    }

    @PrimaryColumn_()
    id!: string

    @Column_("int4", {nullable: false})
    policyId!: number

    @Index_()
    @ManyToOne_(() => Pool, {nullable: true})
    _pool!: Pool

    @Column_("bool", {nullable: false})
    availability!: boolean

    @Column_("varchar", {length: 6, nullable: false})
    creatingTaskScope!: ApplicableScope

    @Column_("int4", {nullable: true})
    startBlock!: number | undefined | null

    @Column_("int4", {nullable: true})
    endBlock!: number | undefined | null

    @Column_("timestamp with time zone", {nullable: false})
    createdAt!: Date

    @Column_("timestamp with time zone", {nullable: false})
    updatedAt!: Date

    @Column_("timestamp with time zone", {nullable: true})
    deletedAt!: Date | undefined | null
}
