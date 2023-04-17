import {Entity as Entity_, Column as Column_, PrimaryColumn as PrimaryColumn_, ManyToOne as ManyToOne_, Index as Index_} from "typeorm"
import {Task} from "./task.model"
import {TaskEventKind} from "./_taskEventKind"

@Entity_()
export class TaskEvent {
    constructor(props?: Partial<TaskEvent>) {
        Object.assign(this, props)
    }

    @PrimaryColumn_()
    id!: string

    @Index_()
    @ManyToOne_(() => Task, {nullable: true})
    _task!: Task

    @Column_("varchar", {length: 10, nullable: false})
    kind!: TaskEventKind

    @Column_("text", {nullable: true})
    payload!: string | undefined | null

    @Column_("int4", {nullable: false})
    blockNumber!: number

    @Column_("timestamp with time zone", {nullable: false})
    blockTime!: Date
}
