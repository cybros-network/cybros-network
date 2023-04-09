import { EntitiesManager } from "../entity_manager";
import { Impl } from "../model"

export default class ImplsManager extends EntitiesManager<Impl> {
    constructor(entityClass?: typeof Impl) {
        super({
            entityClass: entityClass ? entityClass : Impl,
            newEntityFunc: id => new Impl({id})
        });
    }
}
