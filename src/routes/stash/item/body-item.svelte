<script lang="ts">
  import type { Item } from '../../../types';
  import { profile } from '../../../store';
  import { getShortName } from '../../../helper';
  export let title: string;
  export let item: Item | undefined;
  export let onDblClick: (item: Item) => void;

  function calculateBackgroundStyle(item: Item) {
    const backgroundImageUrl = item.presetImageId
      ? `https://assets.tarkov.dev/${item.presetImageId}-base-image.png`
      : `https://assets.tarkov.dev/${item.tpl}-base-image.png`;

    if (item.cacheImage) {
      return item.cacheImage;
    } else {
      return backgroundImageUrl;
    }
  }
</script>

<div tabindex="-1" role="button" class="body-item" on:dblclick={() => item && onDblClick(item)}>
  {#if title}
    <div class="title">
      {title}
    </div>
  {/if}
  <div class="content">
    {#if item && $profile?.locale}
      <div class="item-image">
        <img alt="item" src={calculateBackgroundStyle(item)} />
      </div>
      <div class="short-name">{getShortName(item.tpl, $profile.locale)}</div>
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
    {/if}
  </div>
</div>

<style>
  .body-item {
    position: relative;
    background-image: url($lib/images/background.png);
    background-position: center;
    height: 100%;
    width: 100%;
    display: flex;
    flex-direction: column;
    border: 1px solid black;
    cursor: pointer;
  }

  .body-item:hover .item-image img {
    filter: brightness(150%);
  }

  .body-item:focus {
    outline: none;
  }

  .title {
    text-transform: uppercase;
    border-bottom: 1px solid black;
    padding: 4px;
    text-align: left;
    font-weight: bold;
    background: rgb(52, 52, 52);
    background: linear-gradient(180deg, rgba(52, 52, 52, 1) 0%, rgba(19, 19, 19, 1) 45%);
  }

  .content {
    position: relative;
    width: 100%;
    height: 100%;
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

  .item-image img {
    max-width: 100%;
    max-height: 100%;
    position: absolute;

    top: 50%;
    left: 50%;
    transform: translateX(-50%) translateY(-50%);
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
