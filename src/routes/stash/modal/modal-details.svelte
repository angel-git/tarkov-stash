<script lang="ts">
  import type { BsgItem, Item, SlotKind } from '../../../types';
  import Modal from './modal.svelte';
  import { calculateBackgroundColor } from '../../../helper';

  export let onClose: () => void;
  export let item: Item;
  export let bsgItems: Record<string, BsgItem>;

  let showModal = true;

  $: if (!showModal) onClose();

  function findItemInSlot(slotName: SlotKind) {
    return item.slotItems?.find((slotItem) => slotItem.slotId === slotName);
  }

  console.log('item', item);

  function handleConfirm() {
    showModal = false;
  }
</script>

<Modal bind:showModal onConfirm={handleConfirm}>
  <h2 slot="header">
    {bsgItems[item.tpl].name}
  </h2>

  <div>
    <div class="img-details">
      <div class="img">
        <img alt="item" src={`https://assets.tarkov.dev/${item.tpl}-base-image.png`} />
      </div>
      <div class="details">
        {bsgItems[item.tpl].description}
      </div>
    </div>
    <div class="slots">
      <div class="slots-grid">
        {#each bsgItems[item.tpl].Slots as slot}
          {@const itemInSlot = findItemInSlot(slot._name)}
          {#if itemInSlot}
            <div
              class="slots-grid-item with-item"
              style={`background-color: ${calculateBackgroundColor(
                bsgItems[itemInSlot.tpl].backgroundColor,
              )}`}
            >
              <div class="slots-grid-item-name">{bsgItems[itemInSlot.tpl].shortName}</div>
              <img alt="item" src={`https://assets.tarkov.dev/${itemInSlot.tpl}-base-image.png`} />
            </div>
          {:else}
            <div class="slots-grid-item">
              <div class="slots-grid-item-name">{slot.name}</div>
            </div>
          {/if}
        {/each}
      </div>
    </div>
  </div>
</Modal>

<style>
  .img-details {
    padding: 16px;
  }

  .img-details .details {
    text-align: justify;
  }

  .slots-grid {
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
