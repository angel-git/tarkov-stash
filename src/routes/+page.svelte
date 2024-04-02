<script lang="ts">
  import { listen } from '@tauri-apps/api/event';
  import { goto } from '$app/navigation';

  listen('profile_loaded', () => {
    goto('/stash');
  });
  listen('error', (event) => {
    goto(`/error?message=${event.payload}`);
  });
</script>

<svelte:head>
  <title>Home</title>
  <meta name="description" content="Tarkov Stash" />
</svelte:head>

<div class="container container-center">
  <h1>
    Welcome to <span class="highlight">tarkov-stash</span> mod
  </h1>
  <h4>Select your profile from the menu to start editing it</h4>
</div>
