module.exports = class Data1681077671196 {
    name = 'Data1681077671196'

    async up(db) {
        await db.query(`CREATE TABLE "workers_pools" ("id" character varying NOT NULL, "worker_id" character varying, "pool_id" character varying, CONSTRAINT "PK_ba62ec5b762791e5657f5b09041" PRIMARY KEY ("id"))`)
        await db.query(`CREATE INDEX "IDX_35556385afd6d26d42d5c1d931" ON "workers_pools" ("worker_id") `)
        await db.query(`CREATE INDEX "IDX_7daac032a60cf6dc55c44d24eb" ON "workers_pools" ("pool_id") `)
        await db.query(`CREATE TABLE "creating_task_policy" ("id" character varying NOT NULL, "permission" character varying(6) NOT NULL, "start_block" integer, "end_block" integer, "created_at" TIMESTAMP WITH TIME ZONE NOT NULL, "updated_at" TIMESTAMP WITH TIME ZONE NOT NULL, "deleted_at" TIMESTAMP WITH TIME ZONE, "pool_id" character varying, CONSTRAINT "PK_f7ec0f93d732904de02842b723c" PRIMARY KEY ("id"))`)
        await db.query(`CREATE INDEX "IDX_8615cd4d78316607c11b6a2f80" ON "creating_task_policy" ("pool_id") `)
        await db.query(`CREATE TABLE "task" ("id" character varying NOT NULL, "spec_version" integer, "status" character varying(10) NOT NULL, "result" character varying(7), "input" bytea, "output" bytea, "proof" bytea, "expires_at" TIMESTAMP WITH TIME ZONE NOT NULL, "created_at" TIMESTAMP WITH TIME ZONE NOT NULL, "updated_at" TIMESTAMP WITH TIME ZONE NOT NULL, "deleted_at" TIMESTAMP WITH TIME ZONE, "assigned_at" TIMESTAMP WITH TIME ZONE, "processing_at" TIMESTAMP WITH TIME ZONE, "processed_at" TIMESTAMP WITH TIME ZONE, "pool_id" character varying, "owner_id" character varying, "assignee_id" character varying, CONSTRAINT "PK_fb213f79ee45060ba925ecd576e" PRIMARY KEY ("id"))`)
        await db.query(`CREATE INDEX "IDX_507a74fbacfd89cbf4436a4405" ON "task" ("pool_id") `)
        await db.query(`CREATE INDEX "IDX_cde1069d3c3c483430e8fed530" ON "task" ("owner_id") `)
        await db.query(`CREATE INDEX "IDX_75114a0b55080c15694f3d40ec" ON "task" ("assignee_id") `)
        await db.query(`CREATE INDEX "IDX_2fe7a278e6f08d2be55740a939" ON "task" ("status") `)
        await db.query(`CREATE TABLE "pool" ("id" character varying NOT NULL, "creating_task_ability" boolean NOT NULL, "creating_task_policies_count" integer NOT NULL, "metadata" text, "created_at" TIMESTAMP WITH TIME ZONE NOT NULL, "updated_at" TIMESTAMP WITH TIME ZONE NOT NULL, "deleted_at" TIMESTAMP WITH TIME ZONE, "owner_id" character varying, "impl_id" character varying, CONSTRAINT "PK_db1bfe411e1516c01120b85f8fe" PRIMARY KEY ("id"))`)
        await db.query(`CREATE INDEX "IDX_6ee0abc520db0e34d73c5bdd3c" ON "pool" ("owner_id") `)
        await db.query(`CREATE INDEX "IDX_03240a4c72d0ad7f7904a6502b" ON "pool" ("impl_id") `)
        await db.query(`CREATE TABLE "impl" ("id" character varying NOT NULL, "attestation_method" character varying(6) NOT NULL, "deployment_permission" character varying(6) NOT NULL, "oldest_build_version" integer NOT NULL, "newest_build_version" integer NOT NULL, "blocked_build_versions" integer array NOT NULL, "metadata" text, "created_at" TIMESTAMP WITH TIME ZONE NOT NULL, "updated_at" TIMESTAMP WITH TIME ZONE NOT NULL, "deleted_at" TIMESTAMP WITH TIME ZONE, "owner_id" character varying, CONSTRAINT "PK_a8cc1327d4d3bff68326b317e75" PRIMARY KEY ("id"))`)
        await db.query(`CREATE INDEX "IDX_716b2f06d36b080fe4bf813acd" ON "impl" ("owner_id") `)
        await db.query(`CREATE TABLE "worker" ("id" character varying NOT NULL, "status" character varying(17) NOT NULL, "impl_spec_version" integer, "impl_build_version" integer, "attestation_method" character varying(6), "attestation_expires_at" TIMESTAMP WITH TIME ZONE, "last_attested_at" TIMESTAMP WITH TIME ZONE, "last_heartbeat_received_at" TIMESTAMP WITH TIME ZONE, "offline_at" TIMESTAMP WITH TIME ZONE, "offline_reason" character varying(24), "created_at" TIMESTAMP WITH TIME ZONE NOT NULL, "updated_at" TIMESTAMP WITH TIME ZONE NOT NULL, "deleted_at" TIMESTAMP WITH TIME ZONE, "owner_id" character varying, "impl_id" character varying, CONSTRAINT "PK_dc8175fa0e34ce7a39e4ec73b94" PRIMARY KEY ("id"))`)
        await db.query(`CREATE INDEX "IDX_f694fda42f7d5548c530287837" ON "worker" ("owner_id") `)
        await db.query(`CREATE INDEX "IDX_4c66d21ba11bba1769f21a8f25" ON "worker" ("impl_id") `)
        await db.query(`CREATE INDEX "IDX_fdca7f90c98f110b79c5842108" ON "worker" ("status") `)
        await db.query(`CREATE TABLE "account" ("id" character varying NOT NULL, CONSTRAINT "PK_54115ee388cdb6d86bb4bf5b2ea" PRIMARY KEY ("id"))`)
        await db.query(`ALTER TABLE "workers_pools" ADD CONSTRAINT "FK_35556385afd6d26d42d5c1d9317" FOREIGN KEY ("worker_id") REFERENCES "worker"("id") ON DELETE NO ACTION ON UPDATE NO ACTION`)
        await db.query(`ALTER TABLE "workers_pools" ADD CONSTRAINT "FK_7daac032a60cf6dc55c44d24eb0" FOREIGN KEY ("pool_id") REFERENCES "pool"("id") ON DELETE NO ACTION ON UPDATE NO ACTION`)
        await db.query(`ALTER TABLE "creating_task_policy" ADD CONSTRAINT "FK_8615cd4d78316607c11b6a2f80a" FOREIGN KEY ("pool_id") REFERENCES "pool"("id") ON DELETE NO ACTION ON UPDATE NO ACTION`)
        await db.query(`ALTER TABLE "task" ADD CONSTRAINT "FK_507a74fbacfd89cbf4436a44059" FOREIGN KEY ("pool_id") REFERENCES "pool"("id") ON DELETE NO ACTION ON UPDATE NO ACTION`)
        await db.query(`ALTER TABLE "task" ADD CONSTRAINT "FK_cde1069d3c3c483430e8fed5306" FOREIGN KEY ("owner_id") REFERENCES "account"("id") ON DELETE NO ACTION ON UPDATE NO ACTION`)
        await db.query(`ALTER TABLE "task" ADD CONSTRAINT "FK_75114a0b55080c15694f3d40ec9" FOREIGN KEY ("assignee_id") REFERENCES "worker"("id") ON DELETE NO ACTION ON UPDATE NO ACTION`)
        await db.query(`ALTER TABLE "pool" ADD CONSTRAINT "FK_6ee0abc520db0e34d73c5bdd3cc" FOREIGN KEY ("owner_id") REFERENCES "account"("id") ON DELETE NO ACTION ON UPDATE NO ACTION`)
        await db.query(`ALTER TABLE "pool" ADD CONSTRAINT "FK_03240a4c72d0ad7f7904a6502bc" FOREIGN KEY ("impl_id") REFERENCES "impl"("id") ON DELETE NO ACTION ON UPDATE NO ACTION`)
        await db.query(`ALTER TABLE "impl" ADD CONSTRAINT "FK_716b2f06d36b080fe4bf813acd2" FOREIGN KEY ("owner_id") REFERENCES "account"("id") ON DELETE NO ACTION ON UPDATE NO ACTION`)
        await db.query(`ALTER TABLE "worker" ADD CONSTRAINT "FK_f694fda42f7d5548c5302878374" FOREIGN KEY ("owner_id") REFERENCES "account"("id") ON DELETE NO ACTION ON UPDATE NO ACTION`)
        await db.query(`ALTER TABLE "worker" ADD CONSTRAINT "FK_4c66d21ba11bba1769f21a8f250" FOREIGN KEY ("impl_id") REFERENCES "impl"("id") ON DELETE NO ACTION ON UPDATE NO ACTION`)
    }

    async down(db) {
        await db.query(`DROP TABLE "workers_pools"`)
        await db.query(`DROP INDEX "public"."IDX_35556385afd6d26d42d5c1d931"`)
        await db.query(`DROP INDEX "public"."IDX_7daac032a60cf6dc55c44d24eb"`)
        await db.query(`DROP TABLE "creating_task_policy"`)
        await db.query(`DROP INDEX "public"."IDX_8615cd4d78316607c11b6a2f80"`)
        await db.query(`DROP TABLE "task"`)
        await db.query(`DROP INDEX "public"."IDX_507a74fbacfd89cbf4436a4405"`)
        await db.query(`DROP INDEX "public"."IDX_cde1069d3c3c483430e8fed530"`)
        await db.query(`DROP INDEX "public"."IDX_75114a0b55080c15694f3d40ec"`)
        await db.query(`DROP INDEX "public"."IDX_2fe7a278e6f08d2be55740a939"`)
        await db.query(`DROP TABLE "pool"`)
        await db.query(`DROP INDEX "public"."IDX_6ee0abc520db0e34d73c5bdd3c"`)
        await db.query(`DROP INDEX "public"."IDX_03240a4c72d0ad7f7904a6502b"`)
        await db.query(`DROP TABLE "impl"`)
        await db.query(`DROP INDEX "public"."IDX_716b2f06d36b080fe4bf813acd"`)
        await db.query(`DROP TABLE "worker"`)
        await db.query(`DROP INDEX "public"."IDX_f694fda42f7d5548c530287837"`)
        await db.query(`DROP INDEX "public"."IDX_4c66d21ba11bba1769f21a8f25"`)
        await db.query(`DROP INDEX "public"."IDX_fdca7f90c98f110b79c5842108"`)
        await db.query(`DROP TABLE "account"`)
        await db.query(`ALTER TABLE "workers_pools" DROP CONSTRAINT "FK_35556385afd6d26d42d5c1d9317"`)
        await db.query(`ALTER TABLE "workers_pools" DROP CONSTRAINT "FK_7daac032a60cf6dc55c44d24eb0"`)
        await db.query(`ALTER TABLE "creating_task_policy" DROP CONSTRAINT "FK_8615cd4d78316607c11b6a2f80a"`)
        await db.query(`ALTER TABLE "task" DROP CONSTRAINT "FK_507a74fbacfd89cbf4436a44059"`)
        await db.query(`ALTER TABLE "task" DROP CONSTRAINT "FK_cde1069d3c3c483430e8fed5306"`)
        await db.query(`ALTER TABLE "task" DROP CONSTRAINT "FK_75114a0b55080c15694f3d40ec9"`)
        await db.query(`ALTER TABLE "pool" DROP CONSTRAINT "FK_6ee0abc520db0e34d73c5bdd3cc"`)
        await db.query(`ALTER TABLE "pool" DROP CONSTRAINT "FK_03240a4c72d0ad7f7904a6502bc"`)
        await db.query(`ALTER TABLE "impl" DROP CONSTRAINT "FK_716b2f06d36b080fe4bf813acd2"`)
        await db.query(`ALTER TABLE "worker" DROP CONSTRAINT "FK_f694fda42f7d5548c5302878374"`)
        await db.query(`ALTER TABLE "worker" DROP CONSTRAINT "FK_4c66d21ba11bba1769f21a8f250"`)
    }
}
