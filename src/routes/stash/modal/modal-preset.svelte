<script lang="ts">
  import type { PresetItem } from '../../../types';

  interface PresetItemWithParent extends PresetItem {
    category: string;
    name: string;
  }

  import { goto } from '$app/navigation';
  import Modal from './modal.svelte';
  import type { BsgItem, Item, NewItem } from '../../../types';
  import { calculateBackgroundColor, getDescription, getName } from '../../../helper';
  import { invokeWithLoader } from '../../../helper';
  import { addNewPreset } from '../../../store';
  import { getShortName } from '../../../helper';
  import { findNewItemLocation } from '../../../helper';

  export let onClose: () => void;
  export let allItems: Record<string, BsgItem>;
  export let locale: Record<string, string>;
  export let grid: Array<Array<Item | undefined>>;
  export let presetItems: Array<PresetItem>;

  let showModal = true;
  let parsedItems: Array<PresetItemWithParent>;
  let parsedNodes: Record<string, BsgItem>;
  let categories: Array<string>;
  let notEnoughSpaceError = false;
  let slots: Array<string> = [];

  const sortByName = (a: PresetItemWithParent, b: PresetItemWithParent) => {
    if (a.name < b.name) {
      return -1;
    }
    if (a.name > b.name) {
      return 1;
    }
    return 0;
  };

  $: if (!showModal) onClose();

  $: {
    const categoriesSet: Set<string> = new Set<string>();

    parsedNodes = Object.keys(allItems)
      .map((i) => allItems[i])
      .filter((i) => i.type === 'Node')
      .reduce((acc, i) => {
        acc[i.id] = i;
        return acc;
      }, {} as Record<string, BsgItem>);

    if (allItems) {
      Object.keys(allItems)
        .map((i) => allItems[i])
        .forEach((i) => {
          categoriesSet.add(getParentNode(i.parentId));
        });

      categories = Array.from(categoriesSet).sort();

      parsedItems = presetItems
        .filter((i) => getName(i.encyclopedia, locale))
        .map((i) => ({
          ...i,
          category: getPresetFromBsgItems(i.encyclopedia),
          name: getName(i.encyclopedia, locale),
        }))
        .filter(
          (i) =>
            i.category.toLowerCase().includes($addNewPreset.input.toLowerCase()) ||
            i.name.toLowerCase().includes($addNewPreset.input.toLowerCase()) ||
            $addNewPreset.item?.id === i.id,
        )
        .sort(sortByName);

      // preselect first item
      if (!$addNewPreset.item) {
        selectItem(parsedItems[0]);
      }
    }
  }

  $: {
    slots = $addNewPreset.item?.items.map((i) => i.slotId) || [];
  }

  function handleConfirm() {
    if (!$addNewPreset.item) {
      return;
    }

    const location = findNewItemLocation($addNewPreset.item.width, $addNewPreset.item.height, grid);
    if (!location) {
      notEnoughSpaceError = true;
      return;
    }

    invokeWithLoader<NewItem>('add_preset', {
      item: {
        id: $addNewPreset.item.id,
        locationX: location?.x,
        locationY: location?.y,
      },
    })
      .catch((e) => goto(`/error?message=${e}`))
      .finally(() => {
        showModal = false;
      });
  }

  function selectItem(item: PresetItem) {
    if (item.id === $addNewPreset.item?.id) {
      addNewPreset.set({ item: undefined, input: $addNewPreset.input });
    } else {
      addNewPreset.set({ item, input: $addNewPreset.input });
    }
  }

  function getParentNode(id: string) {
    return (
      getName(parsedNodes[id]?.id, locale) || getName(parsedNodes[id]?.parentId, locale) || '??'
    );
  }

  function getPresetFromBsgItems(encyclopedia: string) {
    return getParentNode(allItems[encyclopedia].parentId);
  }

  function isAnyItemInCategory(cat: string) {
    return parsedItems.some((item) => item.category === cat);
  }

  function findItemsInSlot(slotId: string) {
    return $addNewPreset.item?.items.filter((slotItem) => slotItem.slotId === slotId);
  }
</script>

<Modal bind:showModal onConfirm={handleConfirm} fullHeight={true}>
  {#if notEnoughSpaceError}
    <h3>You don't have enough space for this item</h3>
  {/if}
  <h2 slot="header">Add preset weapon into stash <strong>(BETA!)</strong></h2>

  <div class="modal-content">
    <!-- svelte-ignore a11y-autofocus -->
    <input autofocus placeholder="Filter..." bind:value={$addNewPreset.input} />
    <div class="main">
      <div class="left">
        {#each categories as cat}
          {#if isAnyItemInCategory(cat)}
            <section>
              <div class="section-heading">
                <h2>{cat}</h2>
              </div>
              <ul>
                {#each parsedItems as item}
                  {#if item.category === cat}
                    <li class={item.id === $addNewPreset.item?.id ? 'selected' : ''}>
                      <button on:click={() => selectItem(item)}>{item.name}</button>
                    </li>
                  {/if}
                {/each}
              </ul>
            </section>
          {/if}
        {/each}
      </div>
      {#if $addNewPreset.item}
        <div class="right" style={`background-color: ${calculateBackgroundColor('black')}`}>
          <div>{getShortName($addNewPreset.item.encyclopedia, locale)}</div>
          <div>{$addNewPreset.item.width}x{$addNewPreset.item.height}</div>
          <img
            alt="item"
            src={`https://assets.tarkov.dev/${$addNewPreset.item.id}-base-image.png`}
          />
          <div class="details">
            {getDescription($addNewPreset.item.encyclopedia, locale)}
          </div>
          <div class="slots">
            <div class="slots-grid">
              {#if slots}
                {#each slots as slotId}
                  {@const itemsInSlot = findItemsInSlot(slotId)}
                  {#if itemsInSlot}
                    {#each itemsInSlot as itemInSlot}
                      {#if itemInSlot._tpl !== $addNewPreset.item.encyclopedia}
                        <div
                          id={itemInSlot._tpl}
                          class="slots-grid-item with-item"
                          style={`background-color: ${calculateBackgroundColor(
                            allItems[itemInSlot._tpl].backgroundColor,
                          )}`}
                        >
                          <div class="slots-grid-item-name">
                            {getShortName(itemInSlot._tpl, locale)}
                          </div>
                          <img
                            alt="item"
                            src={`https://assets.tarkov.dev/${itemInSlot._tpl}-base-image.png`}
                          />
                        </div>
                      {/if}
                    {/each}
                  {/if}
                {/each}
              {/if}
            </div>
          </div>
        </div>
      {/if}
    </div>
  </div>
</Modal>

<style>
  h3 {
    color: orangered;
  }

  .modal-content {
    display: flex;
    flex-direction: column;
    gap: 8px;
    height: 90%;
  }

  input {
    padding: 8px;
    background-color: var(--color-background);
    color: var(--color-text);
  }

  input:focus {
    outline: none;
    border: 2px solid var(--color-highlight);
  }

  .main {
    display: flex;
    overflow-y: auto;
  }

  .main section {
    display: flex;
    flex-direction: column;
  }

  .main .section-heading {
    flex: 0 0 33%;
    position: sticky;
    text-align: right;
    top: 0;
    background-color: var(--color-background);
  }

  .main section h2 {
    border-bottom: 1px solid var(--color-text);
    font-size: 1.2em;
    text-align: left;
  }

  .main .left {
    overflow-y: auto;
    flex: 1 0 50vw;
  }

  .main .right {
    overflow-y: auto;
    flex: 1 1 50vw;
    padding: 8px;
  }

  .main .right .details {
    text-align: justify;
  }

  .main .right img {
    max-width: 100%;
  }

  ul {
    padding: 0;
    margin: 0;
  }

  li {
    list-style-type: none;
    cursor: pointer;
    margin: 8px 0;
    text-align: left;
  }

  li.selected button {
    color: var(--color-highlight);
  }

  li button {
    border: none;
    text-align: left;
  }

  li button:hover {
    color: var(--color-highlight);
  }

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
