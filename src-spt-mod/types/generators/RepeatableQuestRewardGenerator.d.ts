import { HandbookHelper } from '@spt/helpers/HandbookHelper';
import { ItemHelper } from '@spt/helpers/ItemHelper';
import { PresetHelper } from '@spt/helpers/PresetHelper';
import { Item } from '@spt/models/eft/common/tables/IItem';
import { IQuestReward, IQuestRewards } from '@spt/models/eft/common/tables/IQuest';
import { ITemplateItem } from '@spt/models/eft/common/tables/ITemplateItem';
import {
  IBaseQuestConfig,
  IQuestConfig,
  IRepeatableQuestConfig,
} from '@spt/models/spt/config/IQuestConfig';
import { ILogger } from '@spt/models/spt/utils/ILogger';
import { ConfigServer } from '@spt/servers/ConfigServer';
import { DatabaseService } from '@spt/services/DatabaseService';
import { ItemFilterService } from '@spt/services/ItemFilterService';
import { LocalisationService } from '@spt/services/LocalisationService';
import { SeasonalEventService } from '@spt/services/SeasonalEventService';
import { ICloner } from '@spt/utils/cloners/ICloner';
import { MathUtil } from '@spt/utils/MathUtil';
import { ObjectId } from '@spt/utils/ObjectId';
import { RandomUtil } from '@spt/utils/RandomUtil';
export declare class RepeatableQuestRewardGenerator {
  protected logger: ILogger;
  protected randomUtil: RandomUtil;
  protected mathUtil: MathUtil;
  protected databaseService: DatabaseService;
  protected itemHelper: ItemHelper;
  protected presetHelper: PresetHelper;
  protected handbookHelper: HandbookHelper;
  protected localisationService: LocalisationService;
  protected objectId: ObjectId;
  protected itemFilterService: ItemFilterService;
  protected seasonalEventService: SeasonalEventService;
  protected configServer: ConfigServer;
  protected cloner: ICloner;
  protected questConfig: IQuestConfig;
  constructor(
    logger: ILogger,
    randomUtil: RandomUtil,
    mathUtil: MathUtil,
    databaseService: DatabaseService,
    itemHelper: ItemHelper,
    presetHelper: PresetHelper,
    handbookHelper: HandbookHelper,
    localisationService: LocalisationService,
    objectId: ObjectId,
    itemFilterService: ItemFilterService,
    seasonalEventService: SeasonalEventService,
    configServer: ConfigServer,
    cloner: ICloner,
  );
  /**
   * Generate the reward for a mission. A reward can consist of
   * - Experience
   * - Money
   * - Items
   * - Trader Reputation
   *
   * The reward is dependent on the player level as given by the wiki. The exact mapping of pmcLevel to
   * experience / money / items / trader reputation can be defined in QuestConfig.js
   *
   * There's also a random variation of the reward the spread of which can be also defined in the config.
   *
   * Additionally, a scaling factor w.r.t. quest difficulty going from 0.2...1 can be used
   *
   * @param   {integer}   pmcLevel            player's level
   * @param   {number}    difficulty          a reward scaling factor from 0.2 to 1
   * @param   {string}    traderId            the trader for reputation gain (and possible in the future filtering of reward item type based on trader)
   * @param   {object}    repeatableConfig    The configuration for the repeatable kind (daily, weekly) as configured in QuestConfig for the requested quest
   * @returns {object}                        object of "Reward"-type that can be given for a repeatable mission
   */
  generateReward(
    pmcLevel: number,
    difficulty: number,
    traderId: string,
    repeatableConfig: IRepeatableQuestConfig,
    questConfig: IBaseQuestConfig,
  ): IQuestRewards;
  /**
   * @param rewardItems List of reward items to filter
   * @param roublesBudget The budget remaining for rewards
   * @param minPrice The minimum priced item to include
   * @returns True if any items remain in `rewardItems`, false otherwise
   */
  protected filterRewardPoolWithinBudget(
    rewardItems: ITemplateItem[],
    roublesBudget: number,
    minPrice: number,
  ): ITemplateItem[];
  /**
   * Get a randomised number a reward items stack size should be based on its handbook price
   * @param item Reward item to get stack size for
   * @returns Stack size value
   */
  protected getRandomisedRewardItemStackSizeByPrice(item: ITemplateItem): number;
  /**
   * Should reward item have stack size increased (25% chance)
   * @param item Item to possibly increase stack size of
   * @param maxRoublePriceToStack Maximum rouble price an item can be to still be chosen for stacking
   * @returns True if it should
   */
  protected canIncreaseRewardItemStackSize(
    item: ITemplateItem,
    maxRoublePriceToStack: number,
  ): boolean;
  protected calculateAmmoStackSizeThatFitsBudget(
    itemSelected: ITemplateItem,
    roublesBudget: number,
    rewardNumItems: number,
  ): number;
  /**
   * Select a number of items that have a colelctive value of the passed in parameter
   * @param repeatableConfig Config
   * @param roublesBudget Total value of items to return
   * @returns Array of reward items that fit budget
   */
  protected chooseRewardItemsWithinBudget(
    repeatableConfig: IRepeatableQuestConfig,
    roublesBudget: number,
    traderId: string,
  ): ITemplateItem[];
  /**
   * Helper to create a reward item structured as required by the client
   *
   * @param   {string}    tpl             ItemId of the rewarded item
   * @param   {integer}   value           Amount of items to give
   * @param   {integer}   index           All rewards will be appended to a list, for unknown reasons the client wants the index
   * @returns {object}                    Object of "Reward"-item-type
   */
  protected generateRewardItem(
    tpl: string,
    value: number,
    index: number,
    preset?: Item[],
  ): IQuestReward;
  /**
   * Picks rewardable items from items.json. This means they need to fit into the inventory and they shouldn't be keys (debatable)
   * @param repeatableQuestConfig Config file
   * @returns List of rewardable items [[_tpl, itemTemplate],...]
   */
  getRewardableItems(
    repeatableQuestConfig: IRepeatableQuestConfig,
    traderId: string,
  ): [string, ITemplateItem][];
  /**
   * Checks if an id is a valid item. Valid meaning that it's an item that may be a reward
   * or content of bot loot. Items that are tested as valid may be in a player backpack or stash.
   * @param {string} tpl template id of item to check
   * @returns True if item is valid reward
   */
  protected isValidRewardItem(
    tpl: string,
    repeatableQuestConfig: IRepeatableQuestConfig,
    itemBaseWhitelist: string[],
  ): boolean;
  protected addMoneyReward(
    traderId: string,
    rewards: IQuestRewards,
    rewardRoubles: number,
    rewardIndex: number,
  ): void;
}
