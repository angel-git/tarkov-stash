<script lang="ts">
  import type { UserPresetItem } from '../../../types';

  import { goto } from '$app/navigation';
  import Modal from './modal.svelte';
  import Slots from '../slots/slots.svelte';
  import type { BsgItem, Item, NewItem } from '../../../types';
  import { calculateBackgroundColor, getDescription } from '../../../helper';
  import { invokeWithLoader } from '../../../helper';
  import { addNewUserPreset } from '../../../store';
  import { getShortName } from '../../../helper';
  import { findNewItemLocation } from '../../../helper';

  export let onClose: () => void;
  export let allItems: Record<string, BsgItem>;
  export let locale: Record<string, string>;
  export let grid: Array<Array<Item | undefined>>;
  export let userPresets: Array<UserPresetItem>;

  let orderedItems: Array<UserPresetItem>;
  let showModal = true;
  let notEnoughSpaceError = false;
  let slots: Array<string> = [];

  const sortByName = (a: UserPresetItem, b: UserPresetItem) => {
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
    orderedItems = userPresets.sort(sortByName);

    // preselect first item
    if (!$addNewUserPreset.item) {
      selectItem(orderedItems[0]);
    }
  }

  $: {
    slots = ($addNewUserPreset.item?.items.map((i) => i.slotId) || []).filter((s) => s);
  }

  function handleConfirm() {
    if (!$addNewUserPreset.item) {
      return;
    }

    const location = findNewItemLocation(
      $addNewUserPreset.item.width,
      $addNewUserPreset.item.height,
      grid,
    );
    if (!location) {
      notEnoughSpaceError = true;
      return;
    }

    invokeWithLoader<NewItem>('add_user_preset', {
      item: {
        id: $addNewUserPreset.item.id,
        locationX: location?.x,
        locationY: location?.y,
        amount: 1,
      },
    })
      .catch((e) => goto(`/error?message=${e}`))
      .finally(() => {
        showModal = false;
      });
  }

  function selectItem(item: UserPresetItem | undefined) {
    if (item?.id === $addNewUserPreset.item?.id) {
      addNewUserPreset.set({ item: undefined, input: $addNewUserPreset.input });
    } else {
      addNewUserPreset.set({ item, input: $addNewUserPreset.input });
    }
  }
</script>

<Modal bind:showModal onConfirm={handleConfirm} fullHeight={true}>
  {#if notEnoughSpaceError}
    <h3>You don't have enough space for this item</h3>
  {/if}
  <h2 slot="header">Add user presets into stash <strong>(BETA!)</strong></h2>

  <div class="modal-content">
    <!-- svelte-ignore a11y-autofocus -->
    <input autofocus placeholder="Filter..." bind:value={$addNewUserPreset.input} />
    <div class="main">
      <div class="left">
        <ul>
          {#each orderedItems as item}
            {#if item.name.toLowerCase().includes($addNewUserPreset.input.toLowerCase())}
              <li class={item.id === $addNewUserPreset.item?.id ? 'selected' : ''}>
                <button on:click={() => selectItem(item)}>{item.name}</button>
              </li>
            {/if}
          {/each}
        </ul>
      </div>
      {#if $addNewUserPreset.item}
        <div class="right" style={`background-color: ${calculateBackgroundColor('black')}`}>
          <div>{getShortName($addNewUserPreset.item.items[0].tpl, locale)}</div>
          <div>{$addNewUserPreset.item.width}x{$addNewUserPreset.item.height}</div>
          <img
            alt="item"
            src={`https://assets.tarkov.dev/${$addNewUserPreset.item.items[0].tpl}-base-image.webp`}
          />
          <Slots
            itemsInSlots={$addNewUserPreset.item?.items}
            bsgItems={allItems}
            {locale}
            {slots}
          />
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
    gap: 0;
    height: 90%;
  }

  input {
    padding: 8px;
    border: 1px solid var(--color-text);
    background-color: var(--color-background);
    color: var(--color-text);
  }

  input:focus {
    outline: none;
    border: 1px solid var(--color-highlight);
  }

  .main {
    margin: 16px 0;
    display: flex;
    overflow-y: auto;
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
</style>
