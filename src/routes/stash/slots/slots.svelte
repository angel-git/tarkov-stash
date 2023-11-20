<script lang="ts">
  import { calculateBackgroundColor, getAttachmentBackground, getShortName } from '../../../helper';
  import type { BsgItem, SlotKind, PresetItemItem, SlotItem } from '../../../types';
  export let locale: Record<string, string>;
  export let slots: Array<string>;
  export let bsgItems: Record<string, BsgItem>;
  export let itemsInSlots: Array<PresetItemItem | SlotItem> | undefined;

  function findItemsInSlot(slotId: string) {
    return itemsInSlots?.filter((slotItem) => slotItem.slotId === slotId);
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
            <div class="slots-grid-item-name">{getShortName(itemInSlot.tpl, locale)}</div>
            <img alt="item" src={`https://assets.tarkov.dev/${itemInSlot.tpl}-base-image.png`} />
          </div>
        {/each}
      {:else}
        <div class="slots-grid-item">
          <div
            class="slots-grid-item-empty"
            style={`background-image: url(${getEmptyAttachmentBackgroundUrl(slotId)}`}
          />
          <div class="slots-grid-item-name">{locale[slotId.toUpperCase()]}</div>
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
    background-image: url($lib/images/empty-slot.png);
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
</style>
