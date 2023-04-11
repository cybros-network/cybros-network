import { EntitiesManager } from "../entity_manager";
import { ImplBuild } from "../model"

export default class ImplBuildsManager extends EntitiesManager<ImplBuild> {
    constructor(entityClass?: typeof ImplBuild) {
        super({
            entityClass: entityClass ? entityClass : ImplBuild,
            newEntityFunc: id => new ImplBuild({id})
        });
    }
}
