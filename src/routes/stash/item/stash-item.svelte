<script lang="ts">
  import type { Item } from '../../../types';
  import { calculateBackgroundColor } from '../../../helper';
  import { getShortName } from '../../../helper';

  export let item: Item;
  export let locale: Record<string, string>;
  export let handleOpenClick: (item: Item | undefined) => void;

  function calculateSizeStyle(item: Item) {
    return `z-index: 2; position: relative; height: ${item.sizeY * 64}px; width: ${
      item.sizeX * 64
    }px; background-color: ${calculateBackgroundColor(item.backgroundColor)}`;
  }

  function calculateBackgroundStyle(item: Item) {
    const rotation = item.rotation === 'Vertical' ? '90deg' : '0deg';
    const width = item.rotation === 'Vertical' ? item.sizeY * 64 : item.sizeX * 64;
    const height = item.rotation === 'Vertical' ? item.sizeX * 64 : item.sizeY * 64;
    const translateX = (64 - width) / 2;
    const translateY = (64 - height) / 2;

    const translateYAdjustment = (rotation === '90deg' ? width - 64 : height - 64) / 2;
    const translateXAdjustment = (rotation === '90deg' ? height - 64 : width - 64) / 2;

    // Update the translateY value.
    const translateYFinal = translateY + translateYAdjustment;
    const translateXFinal = translateX + translateXAdjustment;
    const transform = {
      rotate: `transform: rotate(${rotation})`,
      translate: rotation === '90deg' ? `translate: ${translateXFinal}px ${translateYFinal}px` : '',
    };
    const backgroundImageUrl = item.presetImageId
      ? `https://assets.tarkov.dev/${item.presetImageId}-base-image.png`
      : `https://assets.tarkov.dev/${item.tpl}-base-image.png`;
    return `${transform.rotate}; ${transform.translate} ; background-image: url(${backgroundImageUrl}); background-repeat: no-repeat; width: ${width}px; height: ${height}px;`;
  }
</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<div
  tabindex="-1"
  role="button"
  class="item-clickable"
  style={calculateSizeStyle(item)}
  on:click={() => handleOpenClick(item)}
>
  <div class="item-image">
    <div style={calculateBackgroundStyle(item)} />
  </div>
  <div class="short-name">{getShortName(item.tpl, locale)}</div>
  <div class="item-data">
    {#if item.isFir}
      <div class="fir" />
    {/if}
    {#if item.isStockable}
      <div class="amount">{item.amount}</div>
    {/if}
    {#if item.maxResource && item.maxResource !== 1}
      <div class="resource">
        {`${item.resource || item.maxResource}/${item.maxResource}`}
      </div>
    {/if}
  </div>
</div>

<style>
  .item-clickable {
    cursor: pointer;
  }

  .item-clickable:hover {
    background-color: rgba(44, 42, 42, 0.7) !important;
  }

  .fir {
    height: 12px;
    width: 12px;
    background-image: url($lib/images/fir.png);
    background-size: 12px 12px;
  }

  .amount {
    font-size: 13px;
    z-index: 2;
  }

  .resource {
    font-size: 13px;
    color: orangered;
    z-index: 2;
  }

  .item-image {
    width: 100%;
    height: 100%;
  }

  .item-data {
    position: absolute;
    top: 0;
    height: 100%;
    width: 100%;
    display: flex;
    flex-direction: column;
    align-items: flex-end;
    justify-content: flex-end;
  }

  .short-name {
    font-size: 11px;
    position: absolute;
    right: 2px;
    top: 2px;
    z-index: 2;
  }
</style>
