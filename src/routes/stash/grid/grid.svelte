<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import type { BsgItem, Item, Option, PresetItem, UserPresetItem } from '../../../types';
  import StashItem from '../item/stash-item.svelte';
  import Grid from './grid.svelte';
  import NewItemModal from '../modal/modal-item.svelte';
  import NewPresetModal from '../modal/modal-preset.svelte';
  import UserPresetModal from '../modal/modal-user-preset.svelte';
  import { getName } from '../../../helper';
  import WeaponIcon from '$lib/images/icon_weapons.png';
  import InfoIcon from '$lib/images/icon_info.png';
  import FirIcon from '$lib/images/fir.png';
  import InspectIcon from '$lib/images/inspect.png';
  import LinkedSearchIcon from '$lib/images/linked-search.png';
  import OpenIcon from '$lib/images/open.png';
  import RepairIcon from '$lib/images/repair.png';
  import DiscardIcon from '$lib/images/discard.png';
  import AmountIcon from '$lib/images/amount.png';

  export let items: Array<Item>;
  export let locale: Record<string, string>;
  export let sizeY: number;
  export let sizeX: number;
  export let nestedLevel: number;
  export let bsgItems: Record<string, BsgItem>;
  export let presetItems: Array<PresetItem>;
  export let userPresets: Array<UserPresetItem>;
  export let onOptionClicked: (
    option: Option,
    item: Item,
    grid?: Array<Array<Item | undefined>>,
  ) => void;

  onMount(() => {
    window.addEventListener('scroll', onScroll);
  });

  onDestroy(() => {
    window.removeEventListener('scroll', onScroll);
  });

  const onScroll = () => {
    localStorage.setItem('scrollY', window.scrollY.toString());
  };

  let isNewItemModalOpen = false;
  let isPresetItemModalOpen = false;
  let isUserBuildPresetItemModalOpen = false;
  let secondaryItemMenuId = '-1';
  let containerOpenId = '-1';
  let orderedItems: Array<Item | undefined>;
  $: orderedItems = [];
  let grid: Array<Array<Item | undefined>>;
  $: grid = [];

  $: {
    if (items) {
      grid = Array.from({ length: sizeY }, () => Array(sizeX).fill(null));
      const addedItems = new Set();
      const tempItems: Array<Item | undefined> = [];

      items.forEach((item) => {
        const rotatedItem =
          item.rotation === 'Vertical'
            ? {
                ...item,
                sizeX: item.sizeY,
                sizeY: item.sizeX,
              }
            : item;
        for (let col = rotatedItem.y; col < rotatedItem.y + rotatedItem.sizeY; col++) {
          for (let row = rotatedItem.x; row < rotatedItem.x + rotatedItem.sizeX; row++) {
            if (col < sizeY && row < sizeX) {
              grid[col][row] = rotatedItem;
            }
          }
        }
      });

      for (let col = 0; col < sizeY; col++) {
        for (let row = 0; row < sizeX; row++) {
          const item = grid[col][row];
          if (item) {
            if (!addedItems.has(item.id)) {
              tempItems.push(item);
              addedItems.add(item.id);
            } else {
              tempItems.push(undefined);
            }
          } else {
            tempItems.push(undefined);
          }
        }
      }

      orderedItems = [...tempItems];
      // restore scroll position after items have been reloaded
      if (localStorage.getItem('scrollY')) {
        window.scrollTo(0, Number(localStorage.getItem('scrollY')));
      }
    }
  }

  function handleOpenClick(item: Item | undefined) {
    if (!item) {
      secondaryItemMenuId = '-1';
      return;
    }
    if (secondaryItemMenuId === item.id) {
      secondaryItemMenuId = '-1';
    } else {
      secondaryItemMenuId = item.id;
    }
  }

  function handleOpenDetails(item: Item | undefined) {
    if (!item) return;
    onOptionClicked('details', item);
  }

  function handleOptionClicked(option: Option, item: Item | undefined) {
    if (!item) return;
    secondaryItemMenuId = '-1';
    if (option === 'open') {
      containerOpenId = item.id;
    } else {
      onOptionClicked(option, item, grid);
    }
  }

  function openNewItemModal() {
    secondaryItemMenuId = '-1';
    isNewItemModalOpen = true;
  }

  function openPresetItemModal() {
    secondaryItemMenuId = '-1';
    isPresetItemModalOpen = true;
  }

  function openUserBuildPresetItemModal() {
    secondaryItemMenuId = '-1';
    isUserBuildPresetItemModalOpen = true;
  }
</script>

{#if nestedLevel === 1}
  <div class="button-container">
    <button class="primary" on:click={openNewItemModal}>Add item</button>
    <button class="primary" on:click={openPresetItemModal}
      ><img alt="weapon logo" src={InfoIcon} />Add preset
    </button>
    <button
      class="primary"
      disabled={userPresets.length === 0}
      on:click={openUserBuildPresetItemModal}
      ><img alt="weapon logo" src={WeaponIcon} />Add user weapon preset
    </button>
  </div>
{/if}
{#if isNewItemModalOpen}
  <NewItemModal {grid} allItems={bsgItems} {locale} onClose={() => (isNewItemModalOpen = false)} />
{/if}
{#if isPresetItemModalOpen}
  <NewPresetModal
    {presetItems}
    {grid}
    allItems={bsgItems}
    {locale}
    onClose={() => (isPresetItemModalOpen = false)}
  />
{/if}
{#if isUserBuildPresetItemModalOpen}
  <UserPresetModal
    {userPresets}
    {grid}
    allItems={bsgItems}
    {locale}
    onClose={() => (isUserBuildPresetItemModalOpen = false)}
  />
{/if}
<div
  class="grid"
  style={`grid-template-columns: repeat(${sizeX}, 64px); width: ${sizeX * 64 + 20}px;`}
>
  {#each orderedItems as item}
    <div class="grid-item">
      {#if item}
        <StashItem {locale} {handleOpenClick} {handleOpenDetails} {item} />
      {:else}
        <div class="empty" />
      {/if}
      {#if item?.id === secondaryItemMenuId}
        <div class="options">
          <div class="title">{getName(item.tpl, locale)}</div>
          <!-- svelte-ignore a11y-click-events-have-key-events -->
          <div
            class="option"
            tabindex="-1"
            role="button"
            on:click={() => handleOptionClicked('details', item)}
          >
            <img alt="inspect logo" src={InspectIcon} />
            <div>Inspect</div>
          </div>
          <!-- svelte-ignore a11y-click-events-have-key-events -->
          <div
            class="option"
            tabindex="-1"
            role="button"
            on:click={() => handleOptionClicked('linked-search', item)}
          >
            <img alt="inspect logo" src={LinkedSearchIcon} />
            <div>Linked search</div>
          </div>
          {#if item.isContainer}
            <!-- svelte-ignore a11y-click-events-have-key-events -->
            <div
              class="option"
              tabindex="-1"
              role="button"
              on:click={() => handleOptionClicked('open', item)}
            >
              <img alt="open logo" src={OpenIcon} />
              <div>Open</div>
            </div>
          {/if}
          {#if item.isStockable}
            <!-- svelte-ignore a11y-click-events-have-key-events -->
            <div
              class="option"
              tabindex="-1"
              role="button"
              on:click={() => handleOptionClicked('amount', item)}
            >
              <img alt="amount logo" src={AmountIcon} />
              <div>Change amount</div>
            </div>
          {/if}
          {#if !item.isFir}
            <!-- svelte-ignore a11y-click-events-have-key-events -->
            <div
              class="option"
              tabindex="-1"
              role="button"
              on:click={() => handleOptionClicked('fir', item)}
            >
              <img alt="fir logo" src={FirIcon} />
              <div>Set fir</div>
            </div>
          {/if}
          {#if item.maxResource && item.maxResource !== 1 && item.resource !== item.maxResource}
            <!-- svelte-ignore a11y-click-events-have-key-events -->
            <div
              class="option"
              tabindex="-1"
              role="button"
              on:click={() => handleOptionClicked('resource', item)}
            >
              <img alt="repair logo" src={RepairIcon} />
              <div>Restore durability</div>
            </div>
          {/if}
          {#if !item.isContainer}
            <!-- svelte-ignore a11y-click-events-have-key-events -->
            <div
              class="option destructive"
              tabindex="-1"
              role="button"
              on:click={() => handleOptionClicked('delete', item)}
            >
              <img alt="discard logo" src={DiscardIcon} />
              <div>Discard</div>
            </div>
          {/if}
        </div>
      {/if}
      {#if item?.id === containerOpenId}
        <div class="nested-grid" style={`z-index: ${nestedLevel + 10}`}>
          <div class="nested-grid-header">
            <div>{getName(item.tpl, locale)}</div>
            <button on:click={() => (containerOpenId = '-1')}>close</button>
          </div>
          {#each item.gridItems as gridItem}
            <Grid
              {locale}
              nestedLevel={nestedLevel + 1}
              {bsgItems}
              items={gridItem.items}
              sizeX={gridItem.cellsH}
              sizeY={gridItem.cellsV}
              {onOptionClicked}
            />
          {/each}
        </div>
      {/if}
    </div>
  {/each}
</div>

<style>
  .grid {
    display: grid;
    margin: 16px auto;
    max-height: calc(100vh - 300px);
    overflow-y: auto;
  }

  .grid-item {
    height: 64px;
    width: 64px;
    position: relative;
    background-image: url($lib/images/grid_cell.png);
    background-size: cover;
  }

  .nested-grid {
    border: 2px solid var(--color-background);
    background-color: var(--color-background);
    position: absolute;
    top: 0;
  }

  .nested-grid-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    background-color: rgba(44, 42, 42, 0.7);
  }

  .empty {
    height: 64px;
    width: 64px;
  }

  .options {
    position: absolute;
    top: 10px;
    left: 10px;
    background-color: var(--color-background);
    border: 1px solid var(--color-background);
    font-size: 12px;
    z-index: 5;
    min-width: 170px;
  }

  .options .option {
    font-size: 11px;
    padding: 2px 10px;
    margin: 2px 0;
    border-top-left-radius: 6px;
    border-bottom-right-radius: 6px;
    text-transform: uppercase;
    background-color: var(--color-menu);
    display: flex;
    gap: 16px;
    justify-items: center;
    align-self: center;
    align-content: center;
    align-items: center;
  }

  .options .option img {
    max-height: 14px;
    max-width: 14px;
  }

  .options .option:hover img {
    filter: invert();
  }

  .options .option.destructive {
    color: var(--color-destructive);
  }

  .options .option.destructive:hover {
    color: var(--color-background);
    background-color: var(--color-destructive);
  }

  .options .title {
    font-weight: bold;
    border-bottom: 1px solid var(--color-menu);
  }

  .options .option:hover {
    cursor: pointer;
    background-color: var(--color-text);
    color: var(--color-background);
  }

  .button-container {
    display: flex;
    gap: 8px;
    justify-content: center;
  }

  .button-container button {
    display: flex;
    align-items: center;
  }
  .button-container button img {
    max-height: 15px;
  }
</style>
