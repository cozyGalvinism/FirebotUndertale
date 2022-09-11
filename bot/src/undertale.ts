import { Firebot, RunRequest } from "@crowbartools/firebot-custom-scripts-types";
import { CustomVariableManager } from "@crowbartools/firebot-custom-scripts-types/types/modules/custom-variable-manager";
import axios from "axios";
import items  from "./items";

let SERVER_PORT: number;
let customVarManager: CustomVariableManager;

type UndertaleItem = {
    name: string;
    id: number;
};

const SetHealthEffect: Firebot.EffectType<{
    newHealth: number;
}> = {
    definition: {
        id: "tcu:set-health",
        name: "Set Health",
        description: "Sets Frisk's health to a specific value.",
        icon: "fad fa-heart",
        categories: ["fun"]
    },
    optionsTemplate: `
        <eos-container header="New Health">
            <input type="text" class="form-control" ng-model="effect.newHealth" placeholder="20" replace-variables menu-position="below" />
        </eos-container>
    `,
    optionsController: () => { },
    optionsValidator: (effect) => {
        if (effect.newHealth < 0) {
            return ["Health must be greater than or equal to 0."];
        }
        return [];
    },
    onTriggerEvent: async ({ effect }) => {
        await axios.post(`http://localhost:${SERVER_PORT}/setHealth`, {
            health: effect.newHealth
        });
    }
};

const GetHealthEffect: Firebot.EffectType<{
    variableName: string
}> = {
    definition: {
        id: "tcu:get-health",
        name: "Get Health",
        description: "Gets Frisk's current health.",
        icon: "fad fa-heart",
        categories: ["fun"]
    },
    optionsTemplate: `
        <eos-container header="Health Variable">
            <input type="text" class="form-control" ng-model="effect.variableName" placeholder="health" replace-variables menu-position="below" />
        </eos-container>
    `,
    optionsController: () => { },
    optionsValidator: () => { return []; },
    onTriggerEvent: async ({effect}) => {
        const response = await axios.get(`http://localhost:${SERVER_PORT}/getHealth`);

        if (effect.variableName) {
            customVarManager.addCustomVariable(effect.variableName, response.data.health);
        }
    }
};

const GetMaxHealthEffect: Firebot.EffectType<{
    variableName: string
}> = {
    definition: {
        id: "tcu:get-max-health",
        name: "Get Max Health",
        description: "Gets Frisk's maximum health.",
        icon: "fad fa-heart",
        categories: ["fun"]
    },
    optionsTemplate: `
        <eos-container header="Max Health Variable">
            <input type="text" class="form-control" ng-model="effect.variableName" placeholder="maxHealth" replace-variables menu-position="below" />
        </eos-container>
    `,
    optionsController: () => { },
    optionsValidator: () => { return []; },
    onTriggerEvent: async ({effect}) => {
        const response = await axios.get(`http://localhost:${SERVER_PORT}/getMaxHealth`);

        if (effect.variableName) {
            customVarManager.addCustomVariable(effect.variableName, response.data.maxHealth);
        }
    }
};

const SetGoldEffect: Firebot.EffectType<{
    newGold: number;
}> = {
    definition: {
        id: "tcu:set-gold",
        name: "Set Gold",
        description: "Sets Frisk's gold to a specific value.",
        icon: "fad fa-coins",
        categories: ["fun"]
    },
    optionsTemplate: `
        <eos-container header="New Gold">
            <input type="text" class="form-control" ng-model="effect.newGold" placeholder="100" replace-variables menu-position="below" />
        </eos-container>
    `,
    optionsController: () => { },
    optionsValidator: (effect) => {
        if (effect.newGold < 0) {
            return ["Gold must be greater than or equal to 0."];
        }
        return [];
    },
    onTriggerEvent: async ({ effect }) => {
        await axios.post(`http://localhost:${SERVER_PORT}/setGold`, {
            gold: effect.newGold
        });
    }
};

const GetGoldEffect: Firebot.EffectType<{
    variableName: string
}> = {
    definition: {
        id: "tcu:get-gold",
        name: "Get Gold",
        description: "Gets Frisk's current gold.",
        icon: "fad fa-coins",
        categories: ["fun"]
    },
    optionsTemplate: `
        <eos-container header="Gold Variable">
            <input type="text" class="form-control" ng-model="effect.variableName" placeholder="gold" replace-variables menu-position="below" />
        </eos-container>
    `,
    optionsController: () => { },
    optionsValidator: () => { return []; },
    onTriggerEvent: async ({effect}) => {
        const response = await axios.get(`http://localhost:${SERVER_PORT}/getGold`);

        if (effect.variableName) {
            customVarManager.addCustomVariable(effect.variableName, response.data.gold);
        }
    }
};

const GetInventorySlotEffect: Firebot.EffectType<{
    slot: number;
    variableName: string;
}> = {
    definition: {
        id: "tcu:get-inventory-slot",
        name: "Get Inventory Slot",
        description: "Gets the item in a specific inventory slot.",
        icon: "fad fa-box",
        categories: ["fun"]
    },
    optionsTemplate: `
        <eos-container header="Slot">
            <input type="text" class="form-control" ng-model="effect.slot" placeholder="1" replace-variables menu-position="below" />
        </eos-container>
        <eos-container header="Item Variable" pad-top="true">
            <input type="text" class="form-control" ng-model="effect.variableName" placeholder="item" replace-variables menu-position="below" />
        </eos-container>
    `,
    optionsController: () => { },
    optionsValidator: (effect) => {
        if (effect.slot < 0 || effect.slot > 8) {
            return ["Slot must be between 0 and 7."];
        }
        return [];
    },
    onTriggerEvent: async ({effect}) => {
        const response = await axios.post(`http://localhost:${SERVER_PORT}/getInventory`, {
            slot: effect.slot
        });

        if (effect.variableName) {
            customVarManager.addCustomVariable(effect.variableName, response.data.item);
        }
    }
};

const SetEncounterCounterEffect: Firebot.EffectType<{
    newEncounterCounter: number;
}> = {
    definition: {
        id: "tcu:set-encounter-counter",
        name: "Set Encounter Counter",
        description: "Sets the encounter counter to a specific value.",
        icon: "fad fa-dice-d20",
        categories: ["fun"]
    },
    optionsTemplate: `
        <eos-container header="New Encounter Counter">
            <input type="text" class="form-control" ng-model="effect.newEncounterCounter" placeholder="0" replace-variables menu-position="below" />
        </eos-container>
    `,
    optionsController: () => { },
    optionsValidator: (effect) => {
        if (effect.newEncounterCounter < 0) {
            return ["Encounter counter must be greater than or equal to 0."];
        }
        return [];
    },
    onTriggerEvent: async ({ effect }) => {
        await axios.post(`http://localhost:${SERVER_PORT}/setEncounter`, {
            encounterCounter: effect.newEncounterCounter
        });
    }
};

const SetSpeedEffect: Firebot.EffectType<{
    newSpeed: number;
}> = {
    definition: {
        id: "tcu:set-speed",
        name: "Set Speed",
        description: "Sets Frisk's speed to a specific value.",
        icon: "fad fa-running",
        categories: ["fun"]
    },
    optionsTemplate: `
        <eos-container header="New Speed">
            <input type="text" class="form-control" ng-model="effect.newSpeed" placeholder="100" replace-variables menu-position="below" />
        </eos-container>
    `,
    optionsController: () => { },
    optionsValidator: (effect) => {
        if (effect.newSpeed < 0) {
            return ["Speed must be greater than or equal to 0."];
        }
        return [];
    },
    onTriggerEvent: async ({ effect }) => {
        await axios.post(`http://localhost:${SERVER_PORT}/setSpeed`, {
            speed: effect.newSpeed
        });
    }
};

const GetSpeedEffect: Firebot.EffectType<{
    variableName: string
}> = {
    definition: {
        id: "tcu:get-speed",
        name: "Get Speed",
        description: "Gets Frisk's current heart speed.",
        icon: "fad fa-running",
        categories: ["fun"]
    },
    optionsTemplate: `
        <eos-container header="Speed Variable">
            <input type="text" class="form-control" ng-model="effect.variableName" placeholder="speed" replace-variables menu-position="below" />
        </eos-container>
    `,
    optionsController: () => { },
    optionsValidator: () => { return []; },
    onTriggerEvent: async ({effect}) => {
        const response = await axios.get(`http://localhost:${SERVER_PORT}/getSpeed`);

        if (effect.variableName) {
            customVarManager.addCustomVariable(effect.variableName, response.data.speed);
        }
    }
};

const GetKillAreaEffect: Firebot.EffectType<{
    variableName: string
}> = {
    definition: {
        id: "tcu:get-kill-area",
        name: "Get Kill Area",
        description: "Gets the current kill area.",
        icon: "fad fa-skull-crossbones",
        categories: ["fun"]
    },
    optionsTemplate: `
        <eos-container header="Kill Area Variable">
            <input type="text" class="form-control" ng-model="effect.variableName" placeholder="killArea" replace-variables menu-position="below" />
        </eos-container>
    `,
    optionsController: () => { },
    optionsValidator: () => { return []; },
    onTriggerEvent: async ({effect}) => {
        const response = await axios.get(`http://localhost:${SERVER_PORT}/getKillArea`);

        if (effect.variableName) {
            customVarManager.addCustomVariable(effect.variableName, response.data.killArea);
        }
    }
};

const GetEquippedWeaponEffect: Firebot.EffectType<{
    variableName: string
}> = {
    definition: {
        id: "tcu:get-equipped-weapon",
        name: "Get Equipped Weapon",
        description: "Gets the weapon currently equipped.",
        icon: "fad fa-sword",
        categories: ["fun"]
    },
    optionsTemplate: `
        <eos-container header="Weapon Variable">
            <input type="text" class="form-control" ng-model="effect.variableName" placeholder="weapon" replace-variables menu-position="below" />
        </eos-container>
    `,
    optionsController: () => { },
    optionsValidator: () => { return []; },
    onTriggerEvent: async ({effect}) => {
        const response = await axios.get(`http://localhost:${SERVER_PORT}/getEquippedWeapon`);

        if (effect.variableName) {
            customVarManager.addCustomVariable(effect.variableName, response.data.weapon);
        }
    }
};

const SetEquippedWeaponEffect: Firebot.EffectType<{
    newWeapon: number
}> = {
    definition: {
        id: "tcu:set-equipped-weapon",
        name: "Set Equipped Weapon",
        description: "Sets the weapon currently equipped.",
        icon: "fad fa-sword",
        categories: ["fun"]
    },
    optionsTemplate: `
        <eos-container header="New Weapon">
            <ui-select ng-model="selectedItem" theme="bootstrap" on-select="itemSelected($item)">
                <ui-select-match placeholder="Select an item...">
                    <div style="height: 21px; display: flex; flex-direction: row; align-items: center;">
                        <div style="font-weight: 100; font-size: 17px;">{{$select.selected.name}}</div>
                    </div>
                </ui-select-match>
                <ui-select-choices minimum-input-length="1" repeat="item in items | filter: $select.search" style="position: relative;">
                    <div style="height: 35px; display: flex; flex-direction: row; align-items: center;">
                        <div style="font-weight: 100; font-size: 17px;">{{item.name}}</div>
                    </div>
                </ui-select-choices>
            </ui-select>
        </eos-container>
    `,
    optionsController: ($scope, $q: any, backendCommunicator: any) => {
        $scope.selectedItem = null;
        $scope.items = [];

        $q.when(backendCommunicator.fireEventAsync("get-undertale-items"))
            .then((items: UndertaleItem[]) => {
                if (items) {
                    $scope.items = items;
                    if ($scope.effect.newWeapon) {
                        $scope.selectedItem = ($scope.items as UndertaleItem[]).find(i => i.id === $scope.effect.newWeapon);
                    }
                }
            });
        
        $scope.itemSelected = (item: UndertaleItem) => {
            if (item) {
                $scope.effect.newWeapon = item.id;
            }
        };
    },
    optionsValidator: () => { return []; },
    onTriggerEvent: async ({effect}) => {
        await axios.post(`http://localhost:${SERVER_PORT}/setEquippedWeapon`, {
            weapon: effect.newWeapon
        });
    }
};

const GetEquippedArmorEffect: Firebot.EffectType<{
    variableName: string
}> = {
    definition: {
        id: "tcu:get-equipped-armor",
        name: "Get Equipped Armor",
        description: "Gets the armor currently equipped.",
        icon: "fad fa-shield",
        categories: ["fun"]
    },
    optionsTemplate: `
        <eos-container header="Armor Variable">
            <input type="text" class="form-control" ng-model="effect.variableName" placeholder="armor" replace-variables menu-position="below" />
        </eos-container>
    `,
    optionsController: () => { },
    optionsValidator: () => { return []; },
    onTriggerEvent: async ({effect}) => {
        const response = await axios.get(`http://localhost:${SERVER_PORT}/getEquippedArmor`);

        if (effect.variableName) {
            customVarManager.addCustomVariable(effect.variableName, response.data.armor);
        }
    }
};

const SetEquippedArmorEffect: Firebot.EffectType<{
    newArmor: number
}> = {
    definition: {
        id: "tcu:set-equipped-armor",
        name: "Set Equipped Armor",
        description: "Sets the armor currently equipped.",
        icon: "fad fa-shield",
        categories: ["fun"]
    },
    optionsTemplate: `
        <eos-container header="New Armor">
            <ui-select ng-model="selectedItem" theme="bootstrap" on-select="itemSelected($item)">
                <ui-select-match placeholder="Select an item...">
                    <div style="height: 21px; display: flex; flex-direction: row; align-items: center;">
                        <div style="font-weight: 100; font-size: 17px;">{{$select.selected.name}}</div>
                    </div>
                </ui-select-match>
                <ui-select-choices minimum-input-length="1" repeat="item in items | filter: $select.search" style="position: relative;">
                    <div style="height: 35px; display: flex; flex-direction: row; align-items: center;">
                        <div style="font-weight: 100; font-size: 17px;">{{item.name}}</div>
                    </div>
                </ui-select-choices>
            </ui-select>
        </eos-container>
    `,
    optionsController: ($scope, $q: any, backendCommunicator: any) => {
        $scope.selectedItem = null;
        $scope.items = [];

        $q.when(backendCommunicator.fireEventAsync("get-undertale-items"))
            .then((items: UndertaleItem[]) => {
                if (items) {
                    $scope.items = items;
                    if ($scope.effect.newArmor) {
                        $scope.selectedItem = ($scope.items as UndertaleItem[]).find(i => i.id === $scope.effect.newArmor);
                    }
                }
            });

        $scope.itemSelected = (item: UndertaleItem) => {
            if (item) {
                $scope.effect.newArmor = item.id;
            }
        };
    },
    optionsValidator: () => { return []; },
    onTriggerEvent: async ({effect}) => {
        await axios.post(`http://localhost:${SERVER_PORT}/setEquippedArmor`, {
            armor: effect.newArmor
        });
    }
};

const GetKillsEffect: Firebot.EffectType<{
    variableName: string
}> = {
    definition: {
        id: "tcu:get-kills",
        name: "Get Kills",
        description: "Gets the number of kills in the current area (if applicable).",
        icon: "fad fa-skull",
        categories: ["fun"]
    },
    optionsTemplate: `
        <eos-container header="Kills Variable">
            <input type="text" class="form-control" ng-model="effect.variableName" placeholder="kills" replace-variables menu-position="below" />
        </eos-container>
    `,
    optionsController: () => { },
    optionsValidator: () => { return []; },
    onTriggerEvent: async ({effect}) => {
        const areaResponse = await axios.get(`http://localhost:${SERVER_PORT}/getKillArea`);
        if (!areaResponse.data || !areaResponse.data.killArea) {
            return;
        }
        const killArea = areaResponse.data.killArea as number;
        let kills: number;
        if (killArea == 202) {
            // ruins
            const response = await axios.get(`http://localhost:${SERVER_PORT}/getKillsRuins`);
            kills = response.data.killsRuins;
        }
        else if (killArea == 203) {
            // snowdin
            const response = await axios.get(`http://localhost:${SERVER_PORT}/getKillsSnowdin`);
            kills = response.data.killsSnowdin;
        }
        else if (killArea == 204) {
            // waterfall
            const response = await axios.get(`http://localhost:${SERVER_PORT}/getKillsWaterfall`);
            kills = response.data.killsWaterfall;
        }
        else if (killArea == 205) {
            // hotland
            const response = await axios.get(`http://localhost:${SERVER_PORT}/getKillsHotland`);
            kills = response.data.killsHotland;
        }
        else {
            kills = 0;
        }

        if (effect.variableName) {
            customVarManager.addCustomVariable(effect.variableName, kills);
        }
    }
};

const SetKillsEffect: Firebot.EffectType<{
    newKills: number
}> = {
    definition: {
        id: "tcu:set-kills",
        name: "Set Kills",
        description: "Sets the number of kills depending on the current area (if applicable).",
        icon: "fad fa-skull",
        categories: ["fun"]
    },
    optionsTemplate: `
        <eos-container header="New Kills">
            <input type="text" class="form-control" ng-model="effect.newKills" placeholder="0" />
        </eos-container>
    `,
    optionsController: () => { },
    optionsValidator: () => { return []; },
    onTriggerEvent: async ({effect}) => {
        const areaResponse = await axios.get(`http://localhost:${SERVER_PORT}/getKillArea`);
        if (!areaResponse.data || !areaResponse.data.killArea) {
            return;
        }
        const killArea = areaResponse.data.killArea as number;

        if (killArea == 202) {
            // ruins
            await axios.post(`http://localhost:${SERVER_PORT}/setKillsRuins`, {
                killsRuins: effect.newKills
            });
        }
        else if (killArea == 203) {
            // snowdin
            await axios.post(`http://localhost:${SERVER_PORT}/setKillsSnowdin`, {
                killsSnowdin: effect.newKills
            });
        }
        else if (killArea == 204) {
            // waterfall
            await axios.post(`http://localhost:${SERVER_PORT}/setKillsWaterfall`, {
                killsWaterfall: effect.newKills
            });
        }
        else if (killArea == 205) {
            // hotland
            await axios.post(`http://localhost:${SERVER_PORT}/setKillsHotland`, {
                killsHotland: effect.newKills
            });
        }
    }
};

const FillInventoryEffect: Firebot.EffectType<{
    item: number,
    overwriteImportantItems: boolean,
    onlyEmptySlots: boolean
}> = {
    definition: {
        id: "tcu:fill-inventory",
        name: "Fill Inventory",
        description: "Fills the inventory with a specific item.",
        icon: "fad fa-box-open",
        categories: ["fun"]
    },
    optionsTemplate: `
        <eos-container header="Item">
            <ui-select ng-model="selectedItem" theme="bootstrap" on-select="itemSelected($item)">
                <ui-select-match placeholder="Select an item...">
                    <div style="height: 21px; display: flex; flex-direction: row; align-items: center;">
                        <div style="font-weight: 100; font-size: 17px;">{{$select.selected.name}}</div>
                    </div>
                </ui-select-match>
                <ui-select-choices minimum-input-length="1" repeat="item in items | filter: $select.search" style="position: relative;">
                    <div style="height: 35px; display: flex; flex-direction: row; align-items: center;">
                        <div style="font-weight: 100; font-size: 17px;">{{item.name}}</div>
                    </div>
                </ui-select-choices>
            </ui-select>
        </eos-container>
        <eos-container header="Filter" pad-top="true">
            <label class="control-fb control--checkbox" style="margin-top: 15px;">
                Overwrite important items
                <input type="checkbox" ng-model="effect.overwriteImportantItems" />
                <div class="control__indicator"></div>
            </label>
            <label class="control-fb control--checkbox" style="margin-top: 15px;">
                Only empty slots
                <input type="checkbox" ng-model="effect.onlyEmptySlots" />
                <div class="control__indicator"></div>
            </label>
        </eos-container>
    `,
    optionsController: ($scope, $q: any, backendCommunicator: any) => {
        $scope.selectedItem = null;
        $scope.items = [];

        $q.when(backendCommunicator.fireEventAsync("get-undertale-items"))
            .then((items: UndertaleItem[]) => {
                if (items) {
                    $scope.items = items;
                    if ($scope.effect.item) {
                        $scope.selectedItem = ($scope.items as UndertaleItem[]).find(i => i.id === $scope.effect.item);
                    }
                }
            });
        
        $scope.itemSelected = (item: UndertaleItem) => {
            if (item) {
                $scope.effect.item = item.id;
            }
        };
    },
    optionsValidator: (effect) => { return []; },
    onTriggerEvent: async ({effect}) => {
        await axios.post(`http://localhost:${SERVER_PORT}/fillInventory`, {
            item: effect.item,
            overwriteImportantItems: effect.overwriteImportantItems,
            onlyEmptySlots: effect.onlyEmptySlots
        });
    },
};

function registerEffects(runRequest: RunRequest, port: number) {
    SERVER_PORT = port;
    customVarManager = runRequest.modules.customVariableManager;

    runRequest.modules.effectManager.registerEffect(SetHealthEffect);
    runRequest.modules.effectManager.registerEffect(GetHealthEffect);
    runRequest.modules.effectManager.registerEffect(GetMaxHealthEffect);
    runRequest.modules.effectManager.registerEffect(SetGoldEffect);
    runRequest.modules.effectManager.registerEffect(GetGoldEffect);
    runRequest.modules.effectManager.registerEffect(GetInventorySlotEffect);
    runRequest.modules.effectManager.registerEffect(FillInventoryEffect);
    runRequest.modules.effectManager.registerEffect(SetEncounterCounterEffect);
    runRequest.modules.effectManager.registerEffect(SetSpeedEffect);
    runRequest.modules.effectManager.registerEffect(GetSpeedEffect);
    runRequest.modules.effectManager.registerEffect(GetKillAreaEffect);
    runRequest.modules.effectManager.registerEffect(GetEquippedWeaponEffect);
    runRequest.modules.effectManager.registerEffect(GetEquippedArmorEffect);
    runRequest.modules.effectManager.registerEffect(SetEquippedWeaponEffect);
    runRequest.modules.effectManager.registerEffect(SetEquippedArmorEffect);
    runRequest.modules.effectManager.registerEffect(SetKillsEffect);
    runRequest.modules.effectManager.registerEffect(GetKillsEffect);
    runRequest.modules.frontendCommunicator.onAsync("get-undertale-items", async () => {
        return items;
    });
}

export {
    registerEffects
};