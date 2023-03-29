import {Entity as Entity_, Column as Column_, PrimaryColumn as PrimaryColumn_, ManyToOne as ManyToOne_, Index as Index_} from "typeorm"
import * as marshal from "./marshal"
import {Account} from "./account.model"
import {WorkerStatus} from "./_workerStatus"
import {AttestationMethod} from "./_attestationMethod"

@Entity_()
export class Worker {
    constructor(props?: Partial<Worker>) {
        Object.assign(this, props)
    }

    @PrimaryColumn_()
    id!: string

    @Index_()
    @ManyToOne_(() => Account, {nullable: true})
    account!: Account

    @Index_()
    @ManyToOne_(() => Account, {nullable: true})
    owner!: Account

    @Column_("numeric", {transformer: marshal.bigintTransformer, nullable: false})
    reserved!: bigint

    @Index_()
    @Column_("varchar", {length: 17, nullable: false})
    status!: WorkerStatus

    @Column_("text", {nullable: false})
    implName!: string

    @Column_("int4", {nullable: false})
    implVersion!: number

    @Column_("varchar", {length: 7, nullable: true})
    attestationMethod!: AttestationMethod | undefined | null

    @Column_("int4", {nullable: true})
    attestedAt!: number | undefined | null
}
