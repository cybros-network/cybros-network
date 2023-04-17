import {Entity as Entity_, Column as Column_, PrimaryColumn as PrimaryColumn_, ManyToOne as ManyToOne_, Index as Index_} from "typeorm"
import {Worker} from "./worker.model"
import {WorkerEventKind} from "./_workerEventKind"

@Entity_()
export class WorkerEvent {
    constructor(props?: Partial<WorkerEvent>) {
        Object.assign(this, props)
    }

    @PrimaryColumn_()
    id!: string

    @Index_()
    @ManyToOne_(() => Worker, {nullable: true})
    _worker!: Worker

    @Column_("text", {nullable: false})
    workerAddress!: string

    @Column_("varchar", {length: 20, nullable: false})
    kind!: WorkerEventKind

    @Column_("text", {nullable: true})
    payload!: string | undefined | null

    @Column_("int4", {nullable: false})
    blockNumber!: number

    @Column_("timestamp with time zone", {nullable: false})
    blockTime!: Date
}
