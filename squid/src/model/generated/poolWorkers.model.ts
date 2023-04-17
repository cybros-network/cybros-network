import {Entity as Entity_, Column as Column_, PrimaryColumn as PrimaryColumn_, ManyToOne as ManyToOne_, Index as Index_} from "typeorm"
import {Pool} from "./pool.model"
import {Worker} from "./worker.model"

@Entity_()
export class PoolWorkers {
    constructor(props?: Partial<PoolWorkers>) {
        Object.assign(this, props)
    }

    @PrimaryColumn_()
    id!: string

    @Index_()
    @ManyToOne_(() => Pool, {nullable: true})
    _pool!: Pool

    @Index_()
    @ManyToOne_(() => Worker, {nullable: true})
    _worker!: Worker

    @Column_("timestamp with time zone", {nullable: false})
    createdAt!: Date

    @Column_("timestamp with time zone", {nullable: true})
    deletedAt!: Date | undefined | null
}
