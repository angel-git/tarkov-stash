<script lang="ts">
  interface BsgItemWithParent extends BsgItem {
    category: string;
    name: string;
  }

  import { goto } from '$app/navigation';
  import Modal from './modal.svelte';
  import type { BsgItem, Item, NewItem } from '../../../types';
  import { calculateBackgroundColor, getDescription, getName } from '../../../helper';
  import { invokeWithLoader } from '../../../helper';
  import { addNewItem } from '../../../store';
  import { getShortName } from '../../../helper';

  export let onClose: () => void;
  export let allItems: Record<string, BsgItem>;
  export let locale: Record<string, string>;
  export let grid: Array<Array<Item | undefined>>;

  let showModal = true;
  let parsedItems: Array<BsgItemWithParent>;
  let parsedNodes: Record<string, BsgItem>;
  let categories: Array<string>;
  let notEnoughSpaceError = false;

  const sortByName = (a: BsgItemWithParent, b: BsgItemWithParent) => {
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
        .filter((i) => i.type === 'Item')
        .filter((i) => !i.unbuyable)
        .filter((i) => getName(i.id, locale))
        .filter((i) => !getName(i.id, locale).includes('!!!DO_NOT_USE!!'))
        .forEach((i) => {
          categoriesSet.add(getParentNode(i));
        });

      // remove wrong categories
      categories = Array.from(categoriesSet)
        .filter((c) => c !== 'inventory')
        .sort();

      parsedItems = Object.keys(allItems)
        .map((i) => allItems[i])
        .filter((i) => i.type === 'Item')
        .filter((i) => !i.unbuyable)
        .filter((i) => getName(i.id, locale))
        .filter((i) => !getName(i.id, locale).includes('!!!DO_NOT_USE!!'))
        .map((i) => ({ ...i, category: getParentNode(i), name: getName(i.id, locale) }))
        .filter(
          (i) =>
            i.category.toLowerCase().includes($addNewItem.input.toLowerCase()) ||
            i.name.toLowerCase().includes($addNewItem.input.toLowerCase()) ||
            $addNewItem.item?.id === i.id,
        )
        .sort(sortByName);

      // preselect first item
      if (!$addNewItem.item) {
        selectItem(parsedItems[0]);
      }
    }
  }

  function handleConfirm() {
    if (!$addNewItem.item) {
      return;
    }

    const location = findNewItemLocation($addNewItem.item);
    if (!location) {
      notEnoughSpaceError = true;
      return;
    }

    invokeWithLoader<NewItem>('add_item', {
      item: {
        id: $addNewItem.item.id,
        locationX: location?.x,
        locationY: location?.y,
      },
    })
      .catch((e) => goto(`/error?message=${e}`))
      .finally(() => {
        showModal = false;
      });
  }

  function selectItem(item: BsgItem) {
    if (item.id === $addNewItem.item?.id) {
      addNewItem.set({ item: undefined, input: $addNewItem.input });
    } else {
      addNewItem.set({ item, input: $addNewItem.input });
    }
  }

  function findNewItemLocation(item: BsgItem) {
    const { width, height } = item;

    const sizeY = grid.length;
    const sizeX = grid[0].length;

    for (let row = 0; row <= sizeY - height; row++) {
      for (let col = 0; col <= sizeX - width; col++) {
        let hasSpace = true;
        for (let i = 0; i < height && hasSpace; i++) {
          for (let j = 0; j < width; j++) {
            if (grid[row + i][col + j]) {
              hasSpace = false;
              break;
            }
          }
        }

        if (hasSpace) {
          return { x: col, y: row };
        }
      }
    }
    return null;
  }

  function getParentNode(item: BsgItem) {
    return (
      getName(parsedNodes[item.parentId]?.id, locale) ||
      getName(parsedNodes[item.parentId]?.parentId, locale) ||
      '??'
    );
  }
</script>

<Modal bind:showModal onConfirm={handleConfirm} fullHeight={true}>
  {#if notEnoughSpaceError}
    <h3>You don't have enough space for this item</h3>
  {/if}
  <h2 slot="header">Add item into stash <strong>(BETA!)</strong></h2>

  <div class="modal-content">
    <!-- svelte-ignore a11y-autofocus -->
    <input autofocus placeholder="Filter..." bind:value={$addNewItem.input} />
    <div class="main">
      <div class="left">
        {#each categories as cat}
          <section>
            <div class="section-heading">
              <h2>{cat}</h2>
            </div>
            <ul>
              {#each parsedItems as item}
                {#if item.category === cat}
                  <li class={item.id === $addNewItem.item?.id ? 'selected' : ''}>
                    <button on:click={() => selectItem(item)}>{item.name}</button>
                  </li>
                {/if}
              {/each}
            </ul>
          </section>
        {/each}
      </div>
      {#if $addNewItem.item}
        <div
          class="right"
          style={`background-color: ${calculateBackgroundColor($addNewItem.item.backgroundColor)}`}
        >
          <div>{getShortName($addNewItem.item.id, locale)}</div>
          <div>{$addNewItem.item.width}X{$addNewItem.item.height}</div>
          <img alt="item" src={`https://assets.tarkov.dev/${$addNewItem.item.id}-base-image.png`} />
          <div class="details">
            {getDescription($addNewItem.item.id, locale)}
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
    flex: 1 1 50vw;
    padding: 8px;
  }

  .main .right .details {
    text-align: justify;
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
</style>
