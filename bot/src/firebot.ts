import { RunRequest } from "@crowbartools/firebot-custom-scripts-types";
import { EventManager } from "@crowbartools/firebot-custom-scripts-types/types/modules/event-manager";

let eventManager: EventManager;
let jsonDb: any;

function initFirebot(runRequest: RunRequest) {
    eventManager = runRequest.modules.eventManager;
    jsonDb = runRequest.modules.JsonDb;
}

export {
    eventManager,
    jsonDb,
    initFirebot
};