<script lang="ts">
  import {
    calculateBackgroundColor,
    getAttachmentBackground,
    getShortName,
    getName,
  } from '../../../helper';
  import type { BsgItem, SlotKind, PresetItemItem, SlotItem } from '../../../types';

  export let locale: Record<string, string>;
  export let slots: Array<string>;
  export let bsgItems: Record<string, BsgItem>;
  export let itemsInSlots: Array<PresetItemItem | SlotItem> | undefined;

  function findItemsInSlot(slotId: string) {
    return itemsInSlots?.filter((slotItem) => slotItem.slotId === slotId);
  }

  function findSlotName(slotId: string) {
    return locale[slotId.toUpperCase()] ?? locale[slotId.toLowerCase()] ?? slotId;
  }

  function getEmptyAttachmentBackgroundUrl(slotId: string) {
    return getAttachmentBackground(slotId as SlotKind);
  }
</script>

<div class="slots-grid">
  {#if slots}
    {#each slots as slotId}
      {@const itemsInSlot = findItemsInSlot(slotId)}
      {#if itemsInSlot && itemsInSlot.length}
        {#each itemsInSlot as itemInSlot}
          <div
            id={itemInSlot.tpl}
            class="slots-grid-item with-item"
            style={`background-color: ${calculateBackgroundColor(
              bsgItems[itemInSlot.tpl].backgroundColor,
            )}`}
          >
            <div class="slots-grid-item-name">
              {getShortName(itemInSlot.tpl, locale) || getName(itemInSlot.tpl, locale)}
            </div>
            {#if itemInSlot.upd?.Repairable}
              <div class="resource">
                {`${
                  itemInSlot.upd.Repairable.Durability || itemInSlot.upd.Repairable.MaxDurability
                }/${itemInSlot.upd.Repairable.MaxDurability}`}
              </div>
            {/if}
            <img alt="item" src={`https://assets.tarkov.dev/${itemInSlot.tpl}-base-image.webp`} />
          </div>
        {/each}
      {:else}
        <div class="slots-grid-item">
          <div
            class="slots-grid-item-empty"
            style={`background-image: url(${getEmptyAttachmentBackgroundUrl(slotId)}`}
          />
          <div class="slots-grid-item-name">{findSlotName(slotId)}</div>
        </div>
      {/if}
    {/each}
  {/if}
</div>

<style>
  .slots-grid {
    padding: 16px;
    display: flex;
    gap: 8px;
    flex-wrap: wrap;
    justify-content: center;
  }

  .slots-grid-item {
    background-image: url($lib/images/grid_cell.png);
    height: 64px;
    width: 64px;
    display: flex;
    justify-content: center;
    position: relative;
    border: 1px solid #575b5e;
  }

  .slots-grid-item .slots-grid-item-empty {
    height: 64px;
    width: 64px;
    background-repeat: no-repeat;
    background-position: center;
  }

  .slots-grid-item.with-item {
    background-image: none;
  }

  .slots-grid-item-name {
    position: absolute;
    font-size: 10px;
    right: 0;
  }

  .slots-grid-item img {
    max-height: 100%;
    max-width: 100%;
    margin: 8px;
  }

  .resource {
    font-size: 13px;
    color: orangered;
    z-index: 2;
    position: absolute;
    bottom: 0;
    right: 0;
  }
</style>
