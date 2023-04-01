import {Entity as Entity_, Column as Column_, PrimaryColumn as PrimaryColumn_, ManyToOne as ManyToOne_, Index as Index_} from "typeorm"
import {Worker} from "./worker.model"
import {Pool} from "./pool.model"

@Entity_()
export class WorkersPools {
    constructor(props?: Partial<WorkersPools>) {
        Object.assign(this, props)
    }

    @PrimaryColumn_()
    id!: string

    @Index_()
    @ManyToOne_(() => Worker, {nullable: true})
    worker!: Worker

    @Index_()
    @ManyToOne_(() => Pool, {nullable: true})
    pool!: Pool
}
