import {Entity as Entity_, Column as Column_, PrimaryColumn as PrimaryColumn_, ManyToOne as ManyToOne_, Index as Index_} from "typeorm"
import {Impl} from "./impl.model"

@Entity_()
export class ImplBuild {
    constructor(props?: Partial<ImplBuild>) {
        Object.assign(this, props)
    }

    @PrimaryColumn_()
    id!: string

    @Index_()
    @ManyToOne_(() => Impl, {nullable: true})
    impl!: Impl

    @Column_("int4", {nullable: false})
    version!: number

    @Column_("text", {nullable: true})
    magicBytes!: string | undefined | null

    @Column_("timestamp with time zone", {nullable: false})
    createdAt!: Date

    @Column_("timestamp with time zone", {nullable: true})
    deletedAt!: Date | undefined | null
}
