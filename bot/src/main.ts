import { Firebot } from "@crowbartools/firebot-custom-scripts-types";
import { Effects, EffectTriggerResponse } from "@crowbartools/firebot-custom-scripts-types/types/effects";
import { EventManager } from "@crowbartools/firebot-custom-scripts-types/types/modules/event-manager";

import axios from "axios";
import { registerEffects as registerUTEffects } from "./undertale";

export interface Params {
  backendPort: number;
  debug: boolean;
}

const script: Firebot.CustomScript<Params> = {
  getScriptManifest: () => {
    return {
      name: "Twitch controls: Undertale",
      description: "A script in order to control various Undertale-related events.",
      author: "cozyGalvinism",
      version: "1.0",
      firebotVersion: "5",
      startupOnly: true
    };
  },
  getDefaultParameters: () => {
    return {
      backendPort: {
        type: "number",
        default: 8080,
        description: "Port",
        secondaryDescription: "Port on which the Undertale remote tool is running."
      },
      debug: {
        type: "boolean",
        default: false,
        description: "Debug",
        secondaryDescription: "Enable debug mode. This will delete the last Tiltify donation ID!"
      }
    };
  },
  run: (runRequest) => {
    const { logger } = runRequest.modules;

    logger.info("Registering Undertale effects...");
    registerUTEffects(runRequest, runRequest.parameters.backendPort);
  },
};

export default script;
