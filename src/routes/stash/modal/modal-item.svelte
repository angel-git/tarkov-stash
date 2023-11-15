<script lang="ts">
  interface BsgItemWithParent extends BsgItem {
    parent: string;
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

  type OrderType = 'alpha' | 'parent';

  let showModal = true;
  let parsedItems: Array<BsgItemWithParent>;
  let parsedNodes: Record<string, BsgItem>;
  const categories: Set<string> = new Set<string>();
  let notEnoughSpaceError = false;
  let order: OrderType = 'alpha';

  const sortByName = (a: BsgItemWithParent, b: BsgItemWithParent) => {
    if (a.parent < b.parent) {
      return -1;
    }
    if (a.parent > b.parent) {
      return 1;
    }
    return 0;
  };

  $: if (!showModal) onClose();

  $: {
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
          categories.add(getParentNode(i));
        });

      console.log('categories', categories);

      parsedItems = Object.keys(allItems)
        .map((i) => allItems[i])
        .filter((i) => i.type === 'Item')
        .filter((i) => !i.unbuyable)
        .filter((i) => getName(i.id, locale))
        .filter((i) => !getName(i.id, locale).includes('!!!DO_NOT_USE!!'))
        .map((i) => {
          if (order === 'alpha') {
            return { ...i, parent: `${getName(i.id, locale)} - [${getParentNode(i)}]` };
          } else {
            return { ...i, parent: `[${getParentNode(i)}] - ${getName(i.id, locale)}` };
          }
        })
        .filter(
          (i) =>
            i.parent.toLowerCase().includes($addNewItem.input.toLowerCase()) ||
            getShortName(i.id, locale).toLowerCase().includes($addNewItem.input.toLowerCase()) ||
            $addNewItem.item?.id === i.id,
        )
        .sort(sortByName);

      // preselet first item
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
    return getName(parsedNodes[item.parentId]?.id, locale) || '??';
  }

  function onOrderChange(event: { currentTarget: HTMLInputElement }) {
    order = event.currentTarget.value as OrderType;
  }
</script>

<Modal bind:showModal onConfirm={handleConfirm} fullHeight={true}>
  {#if notEnoughSpaceError}
    <h3>You don't have enough space for this item</h3>
  {/if}
  <h2 slot="header">Add item into stash <strong>(BETA!)</strong></h2>

  <div class="modal-content">
    <fieldset>
      <input
        type="radio"
        id="alpha"
        name="alpha"
        value="alpha"
        checked={order === 'alpha'}
        on:change={onOrderChange}
      />
      <label for="alpha">Alpha ordering</label>
      <input
        type="radio"
        id="parent"
        name="parent"
        value="parent"
        checked={order === 'parent'}
        on:change={onOrderChange}
      />
      <label for="parent">Parent ordering</label>
    </fieldset>
    <!-- svelte-ignore a11y-autofocus -->
    <input autofocus placeholder="Filter..." bind:value={$addNewItem.input} />
    <div class="main">
      <ul>
        {#each parsedItems as item}
          <li class={item.id === $addNewItem.item?.id ? 'selected' : ''}>
            <button on:click={() => selectItem(item)}>{item.parent}</button>
          </li>
        {/each}
      </ul>
      {#if $addNewItem.item}
        <div
          class="selected-item"
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

  .main ul {
    overflow-y: auto;
    flex: 1 0 50vh;
  }

  .main .selected-item {
    flex: 1 1 50vh;
    padding: 8px;
  }

  .main .selected-item .details {
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
  }

  li button:hover {
    color: var(--color-highlight);
  }

  fieldset {
    border: none;
  }
</style>
