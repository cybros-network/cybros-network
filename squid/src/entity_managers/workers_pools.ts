import { EntitiesManager } from "../entity_manager";
import { WorkersPools } from "../model"

export default class WorkersPoolsManager extends EntitiesManager<WorkersPools> {
    constructor(entityClass?: typeof WorkersPools) {
        super({
            entityClass: entityClass ? entityClass : WorkersPools,
            newEntityFunc: id => new WorkersPools({id})
        });
    }
}
