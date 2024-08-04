<script lang="ts">
  import type { Item, NewItem, RawBsgItem } from '../../../types';
  import { goto } from '$app/navigation';
  import { onMount } from 'svelte';
  import Modal from './modal.svelte';
  import { findNewItemLocation, getName, getShortName, invokeWithLoader } from '../../../helper';

  export let onClose: () => void;
  export let item: Item;
  export let locale: Record<string, string>;
  export let grid: Array<Array<Item | undefined>>;

  let linkedItems: Array<RawBsgItem>;
  let showModal = true;
  let loadingLinkedItems = true;

  $: if (!showModal) onClose();

  onMount(() => {
    invokeWithLoader<Record<string, RawBsgItem>>('linked_search', { item })
      .then((response) => {
        linkedItems = [];
        Object.keys(response).forEach((key) => {
          const linkedItem = response[key];
          if (linkedItem._type !== 'Node' && getShortName(linkedItem._id, locale)) {
            linkedItems.push(linkedItem);
          }
        });
        linkedItems.sort((a, b) => {
          return a._parent.localeCompare(b._parent);
        });
      })
      .catch((e) => goto(`/error?message=${e}`))
      .finally(() => {
        loadingLinkedItems = false;
      });
  });

  function addItem(item: RawBsgItem) {
    const location = findNewItemLocation(item._props.Width, item._props.Height, grid);
    if (!location) {
      // TODO show error
      return;
    }

    invokeWithLoader<NewItem>('add_item', {
      item: {
        id: item._id,
        locationX: location?.x,
        locationY: location?.y,
        amount: item._props.StackMaxSize,
      },
    }).catch((e) => goto(`/error?message=${e}`));
  }
</script>

<Modal bind:showModal withSubmit={false}>
  <h2 slot="header">
    Linked search for item {getShortName(item.tpl, locale)}
  </h2>

  <div>
    {#if loadingLinkedItems}
      loading....
    {/if}
    {#if !loadingLinkedItems}
      {#if linkedItems.length === 0}
        No linked items found
      {/if}
      {#if linkedItems.length > 0}
        <ul>
          {#each linkedItems as linkedItem}
            <li>
              <div>
                <div class="image-container">
                  <img alt="item" src={`https://assets.tarkov.dev/${linkedItem._id}-512.webp`} />
                </div>
                {getName(linkedItem._id, locale)}
              </div>
              <button class="primary" on:click={() => addItem(linkedItem)}>add</button>
            </li>
          {/each}
        </ul>
      {/if}
    {/if}
  </div>
</Modal>

<style>
  ul {
    margin: 0;
    padding: 0;
  }

  ul li {
    list-style: none;
    display: flex;
    margin: 8px;
    border-bottom: 1px solid white;
    justify-content: space-between;
    align-items: center;
  }

  ul li div {
    display: flex;
    gap: 8px;
    align-items: center;
  }

  ul li .image-container {
    width: 64px;
    height: 64px;
  }

  ul li .image-container img {
    max-width: 64px;
    max-height: 64px;
  }
</style>
