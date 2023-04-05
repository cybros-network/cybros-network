import { EntitiesManager } from "../entity_manager";
import { Pool } from "../model"

export default class PoolsManager extends EntitiesManager<Pool> {
    constructor(entityClass?: typeof Pool) {
        super({
            entityClass: entityClass ? entityClass : Pool,
            newEntityFunc: id => new Pool({id})
        });
    }
}
