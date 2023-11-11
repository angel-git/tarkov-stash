<script lang="ts">
  import type { Item, Profile, Option } from '../../types';
  import Grid from './grid/grid.svelte';
  import { getName, getShortName } from '../../helper';

  export let profile: Profile;
  export let onOptionClicked: (option: Option, item: Item) => void;
  let filteredItems: Array<Item | undefined>;
  $: filteredItems = [];
  let searchTerm = '';

  $: {
    if (profile) {
      filteredItems = profile.items.filter((item) => {
        return (
          getName(item.tpl, profile.locale).toLowerCase().includes(searchTerm.toLowerCase()) ||
          getShortName(item.tpl, profile.locale).toLowerCase().includes(searchTerm.toLowerCase())
        );
      });
    }
  }
</script>

<div class="filter">
  <input placeholder="Type to filter" bind:value={searchTerm} />
</div>

<Grid
  locale={profile.locale}
  nestedLevel={1}
  bsgItems={profile.bsgItems}
  items={filteredItems}
  sizeX={profile.sizeX}
  sizeY={profile.sizeY}
  {onOptionClicked}
/>

<style>
  .filter {
    margin: 12px 0;
  }

  .filter input {
    padding: 8px;
    background-color: var(--color-background);
    color: var(--color-text);
  }

  .filter input:focus {
    outline: none;
    border: 2px solid var(--color-highlight);
  }
</style>
