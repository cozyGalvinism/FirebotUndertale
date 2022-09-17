import { Integration, IntegrationController, IntegrationData, IntegrationDefinition, IntegrationManager, LinkData } from '@crowbartools/firebot-custom-scripts-types/types/modules/integration-manager';
import { EventManager, EventSource } from '@crowbartools/firebot-custom-scripts-types/types/modules/event-manager';
import { EventFilter } from '@crowbartools/firebot-custom-scripts-types/types/modules/event-filter-manager';
import { EventEmitter } from 'events';
import axios from "axios";
import { eventManager, jsonDb } from './firebot'; 
import { RunRequest } from '@crowbartools/firebot-custom-scripts-types';
import { Params } from './main';
import { Logger } from '@crowbartools/firebot-custom-scripts-types/types/modules/logger';
import { Effects } from '@crowbartools/firebot-custom-scripts-types/types/effects';
import { GameSettings } from '@crowbartools/firebot-custom-scripts-types/types/modules/game-manager';

// Config(filename: string, saveOnPush?: boolean, humanReadable?: boolean, separator?: string)
let integrationManager: IntegrationManager;
let db: any;
let logger: Logger;

const TILTIFY_BASE_URL = "https://tiltify.com/api/v3/";
// const TILTIFY_BASE_URL = "http://127.0.0.1:3000/api/v3/";

const EVENT_SOURCE_ID = "tiltify";
const EventId = {
    DONATION: "donation",
};

const eventSourceDefinition: EventSource = {
    id: EVENT_SOURCE_ID,
    name: "Tiltify",
    events: [
        {
            id: EventId.DONATION,
            name: "Donation",
            description: "When someone donates to you via Tiltify.",
            cached: false,
            manualMetadata: {
                from: "Tiltify",
                donationAmount: 4.2,
                rewardId: null,
            },
            isIntegration: true,
            queued: true,
            activityFeed: {
                icon: "fad fa-money-bill",
                getMessage: (eventData: any) => {
                    return `**${eventData.from}** donated **$${eventData.donationAmount}** to Tiltify${eventData.rewardId && eventData.rewardId != -1 ? ` with reward *${eventData.rewardId}*` : ''}`;
                }
            }
        }
    ]
};

const integrationDefinition: IntegrationDefinition = {
    id: "tiltify",
    name: "Tiltify",
    description: "Tiltify donation events",
    connectionToggle: true,
    configurable: true,
    settingCategories: {
        integrationSettings: {
            title: "Integration Settings",
            settings: {
                pollInterval: {
                    title: "Poll Interval",
                    type: "number",
                    default: 5,
                    description: "How often to poll Tiltify for new donations (in seconds).",
                }
            }
        },
        campaignSettings: {
            title: "Campaign Settings",
            settings: {
                campaignId: {
                    title: "Campaign ID",
                    type: "string",
                    description: "ID of the running campaign to fetch donations for.",
                    default: "",
                }
            },
        }
    },
    linkType: "id",
    idDetails: {
        steps: 
`1. Log in to [Tiltify](https://dashboard.tiltify.com/)

2. Go to your \`My account\` and then to the \`Connected accounts\` tab

3. Click \`Your applications\` and then \`create application\`

4. In the form, enter a \`Firebot\` as the name and enter \`http://localhost\` as the redirect URL

5. Copy the access token and paste it into the field below`
    }
};

class TiltifyIntegration extends EventEmitter implements IntegrationController {
    timeout: NodeJS.Timeout;
    connected: boolean;

    constructor() {
        super();
        this.timeout = null;
        this.connected = false;
    }

    init() {}

    link() {}
    unlink() {}

    connect(integrationData: IntegrationData) {
        const { accountId } = integrationData;

        if (accountId == null || accountId === "") {
            this.emit("disconnected", integrationDefinition.id);
            return;
        }

        if (integrationData.userSettings == null || integrationData.userSettings.campaignSettings == null) {
            this.emit("connected", integrationDefinition.id);
            this.connected = true;
            return;
        }

        const { campaignId } = integrationData.userSettings.campaignSettings;
        if (campaignId == null || campaignId === "") {
            this.emit("connected", integrationDefinition.id);
            this.connected = true;
            return;
        }

        this.timeout = setInterval(async () => {
            var lastId: number;
            try {
                lastId = db.getData(`/tiltify/${campaignId}/lastId`);
                logger.debug("load: lastId", lastId);
            } catch (e) {
                lastId = -1;
            }

            let ids: any[] = [];
            try {
                ids = db.getData(`/tiltify/${campaignId}/ids`);
            } catch (e) {
                db.push(`/tiltify/${campaignId}/ids`, []);
            }
            logger.debug("load: ids", ids);

            if (lastId == -1) {
                var response = await axios.get(TILTIFY_BASE_URL + "campaigns/" + campaignId + "/donations", {
                    headers: {
                        Authorization: "Bearer " + accountId,
                    }
                });
            } else {
                var response = await axios.get(TILTIFY_BASE_URL + "campaigns/" + campaignId + "/donations?after=" + lastId, {
                    headers: {
                        Authorization: "Bearer " + accountId,
                    }
                });
            }
            
            if (response.status != 200) {
                console.log("Error fetching donations: " + response.status);
                return;
            }

            const { data } = response;
            // sort by ascending completedAt
            var reversed = data.data.sort((a: any, b: any) => a.completedAt - b.completedAt);

            reversed.forEach((donation: { id: number; amount: number; name: string; comment: string; completedAt: number; rewardId?: number; }) => {
                if (db.getData(`/tiltify/${campaignId}/ids`).includes(donation.id)) {
                    return;
                }
                
                lastId = donation.id;

                logger.info(`Donation from ${donation.name} for $${donation.amount}. Reward: ${donation.rewardId}`);
                eventManager.triggerEvent(EVENT_SOURCE_ID, EventId.DONATION, {
                    from: donation.name,
                    donationAmount: donation.amount,
                    rewardId: donation.rewardId,
                }, false);

                ids.push(donation.id);
                db.push(`/tiltify/${campaignId}/ids`, ids);
            });

            logger.debug("save: lastId", lastId);
            db.push(`/tiltify/${campaignId}/lastId`, lastId);
            
        }, (integrationData.userSettings.integrationSettings.pollInterval as number) * 1000);

        this.emit("connected", integrationDefinition.id);
        this.connected = true;
    }

    disconnect() {
        if (this.timeout) {
            clearInterval(this.timeout);
        }
        this.connected = false;
        this.emit("disconnected", integrationDefinition.id);
    }

    onUserSettingsUpdate(integrationData: IntegrationData) {
        if (this.connected) {
            this.disconnect();
        }
        this.connect(integrationData);
    }
}

const integration: Integration = {
    definition: integrationDefinition,
    integration: new TiltifyIntegration(),
};

async function fetchRewards(accountId: string, campaignId: string) {
    try {
        const response = await axios.get(TILTIFY_BASE_URL + "campaigns/" + campaignId + "/rewards", {
            headers: {
                Authorization: "Bearer " + accountId,
            }
        });
        return response.data.data;
    } catch (e) {
        console.log(e);
        return [];
    }
}

async function fetchCampaigns(accountId: string) {
    try {
        const userInfo = await axios.get(TILTIFY_BASE_URL + "user", {
            headers: {
                Authorization: "Bearer " + accountId,
            }
        });
        const userId = userInfo.data.data.id;

        const response = await axios.get(TILTIFY_BASE_URL + "users/" + userId + "/campaigns", {
            headers: {
                Authorization: "Bearer " + accountId,
            }
        });
        return response.data.data;
    } catch (e) {
        console.log(e);
        return [];
    }
}

const RewardFilter: EventFilter = {
    id: "tcu:reward-id",
    name: "Tiltify Reward",
    description: "Filter by the Tiltify reward.",
    events: [
        { eventSourceId: EVENT_SOURCE_ID, eventId: EventId.DONATION },
    ],
    comparisonTypes: [
        "is",
        "is not"
    ],
    valueType: "preset",
    predicate: (filterSettings, eventData) => {
        const rewardId = eventData.eventMeta.rewardId;

        switch (filterSettings.comparisonType) {
            case "is": {
                return Promise.resolve(rewardId != null && rewardId == filterSettings.value);
            }
            case "is not": {
                return Promise.resolve(rewardId != filterSettings.value);
            }
            default: {
                return Promise.resolve(false);
            }
        }
    },
    presetValues: (backendCommunicator) => {
        return backendCommunicator.fireEventAsync("get-tiltify-rewards").then((rewards: any) => {
            return rewards.map((r: any) => ({value: r.id, display: r.name}));
        });
    },
};

function register(runRequest: RunRequest) {
    db = new jsonDb("tiltify.json", true, false, "/");
    logger = runRequest.modules.logger;

    runRequest.modules.integrationManager.registerIntegration(integration);
    runRequest.modules.eventManager.registerEventSource(eventSourceDefinition);
    runRequest.modules.eventFilterManager.registerFilter(RewardFilter);
    runRequest.modules.frontendCommunicator.fireEventAsync("integrationsUpdated", {});

    runRequest.modules.replaceVariableManager.registerReplaceVariable({
        definition: {
            handle: 'tiltifyDonationFrom',
            description: 'The name of who sent a Tiltify donation',
            triggers: {
                "event": [
                    "tiltify:donation"
                ],
                "manual": true
            },
            possibleDataOutput: ["text"]
        },
        evaluator: function (trigger: Effects.Trigger, ...args: any[]) {
            const from = (trigger.metadata.eventData && trigger.metadata.eventData.from) || "Unknown User";

            return from;
        }
    });
    runRequest.modules.replaceVariableManager.registerReplaceVariable({
        definition: {
            handle: 'tiltifyDonationAmount',
            description: 'The amount of a donation from Tiltify',
            triggers: {
                "event": [
                    "tiltify:donation"
                ],
                "manual": true
            },
            possibleDataOutput: ["number"]
        },
        evaluator: function (trigger: Effects.Trigger, ...args: any[]) {
            const donationAmount = (trigger.metadata.eventData && trigger.metadata.eventData.donationAmount) || 0;

            return donationAmount;
        }
    });
    runRequest.modules.replaceVariableManager.registerReplaceVariable({
        definition: {
            handle: 'tiltifyDonationRewardId',
            description: 'The reward ID of a donation from Tiltify',
            triggers: {
                "event": [
                    "tiltify:donation"
                ],
                "manual": true
            },
            possibleDataOutput: ["number"]
        },
        evaluator: function (trigger: Effects.Trigger, ...args: any[]) {
            const rewardId = (trigger.metadata.eventData && trigger.metadata.eventData.rewardId) || -1;

            return rewardId;
        }
    });

    runRequest.modules.frontendCommunicator.onAsync("get-tiltify-rewards", () => {
        let integration = runRequest.modules.integrationManager.getIntegrationDefinitionById("tiltify");
        if (integration == null || integration.userSettings == null || integration.userSettings.campaignSettings == null || integration.userSettings.campaignSettings.campaignId == null || integration.userSettings.campaignSettings.campaignId === "") {
            return Promise.reject("Tiltify integration not found or not configured");
        }
        let accountId = integration.accountId;
        let campaignId = integration.userSettings.campaignSettings.campaignId;

        return fetchRewards(accountId, campaignId);
    });
    runRequest.modules.frontendCommunicator.onAsync("get-tiltify-campaigns", () => {
        let integration = runRequest.modules.integrationManager.getIntegrationDefinitionById("tiltify");
        if (integration == null || integration.accountId == null || integration.accountId === "") {
            return Promise.reject("Tiltify integration not found or not configured");
        }
        let accountId = integration.accountId;

        return fetchCampaigns(accountId);
    });

    integrationManager = runRequest.modules.integrationManager;
}

export {
    register
};