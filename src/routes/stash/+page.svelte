<script lang="ts">
  import { goto } from '$app/navigation';
  import { listen } from '@tauri-apps/api/event';
  import { profile } from '../../store';
  import StashTab from './tabs/stash-tab.svelte';
  import CharacterTab from './tabs/character-tab.svelte';

  const supported_version = '3.8';

  let activeTabValue = 1;
  const handleTabChange = (tabValue: number) => () => (activeTabValue = tabValue);

  listen('go_to_main_page', () => {
    goto('/');
  });

  let tabs = [
    {
      label: 'Stash',
      value: 1,
      component: StashTab,
    },
    {
      label: 'Character',
      value: 2,
      component: CharacterTab,
    },
  ];
</script>

<div class="container container-center">
  {#if $profile}
    {#if !$profile.sptVersion.startsWith(supported_version)}
      <h4 style="color: orangered">
        {`Your SPT version ${$profile.sptVersion} might not be compatible with current supported version: ${supported_version}.x`}
      </h4>
    {/if}
    <h3>
      Editing <span class="highlight">{$profile.name}</span>'s stash
    </h3>

    <ul>
      {#each tabs as tab}
        <li class={activeTabValue === tab.value ? 'active' : ''}>
          <span role="button" on:click={handleTabChange(tab.value)}>{tab.label}</span>
        </li>
      {/each}
    </ul>
    <div class="tabs">
      {#each tabs as tab}
        {#if activeTabValue === tab.value}
          <div class="box">
            <svelte:component this={tab.component} />
          </div>
        {/if}
      {/each}
    </div>
  {:else}
    <p>loading profile....</p>
  {/if}
</div>

<style>
  .tabs {
    border: 1px solid var(--color-menu);
    width: 100%;
    padding: 8px;
    min-width: 720px;
  }

  ul {
    display: flex;
    list-style-type: none;
    gap: 8px;
    margin: 0;
    padding-inline-start: 0;
  }

  ul li {
    background-color: var(--color-background);
    color: var(--color-text);
    border: 1px solid var(--color-menu);
    border-bottom: none;
    padding: 4px 16px;
    border-top-right-radius: 16px;
  }

  ul li:hover {
    color: var(--color-highlight);
    cursor: pointer;
  }

  ul li.active {
    border: none;
    background-color: var(--color-text);
    color: var(--color-background);
  }
</style>
