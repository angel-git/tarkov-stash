<script lang="ts">
  import { listen } from '@tauri-apps/api/event';
  import { goto } from '$app/navigation';
  import { invokeWithLoader } from '../helper';
  import { profile } from '../store';
  import type { Profile, Session } from '../types';

  let hostValue = '127.0.0.1';
  let portValue = 6969;

  let sessions: Array<Session>;
  $: sessions = [];

  listen('profile_loaded', () => {
    // TODO get the session
    reloadProfile({ id: '65de135d00017a100e8b0215', username: 'a' });
  });
  listen('error', (event) => {
    goto(`/error?message=${event.payload}`);
  });

  function connectToServer() {
    invokeWithLoader<Array<Session>>('connect_to_server', {
      server: { host: hostValue, port: portValue },
    })
      .then((r: Array<Session>) => {
        sessions = r;
      })
      .catch((errorMessage) => goto(`/error?message=${errorMessage}`));
  }

  function reloadProfile(session: Session) {
    invokeWithLoader<Profile>('refresh_profile_from_spt', { session })
      .then((r) => {
        console.log('got new profile freshed!', r);
        profile.set(r);
        goto('/stash');
      })
      .catch((errorMessage) => goto(`/error?message=${errorMessage}`));
  }

  function loadProfile(session: Session) {
    invokeWithLoader<Profile>('load_profile_from_spt', { session })
      .then((r) => {
        profile.set(r);
        goto('/stash');
      })
      .catch((errorMessage) => goto(`/error?message=${errorMessage}`));
  }
</script>

<svelte:head>
  <title>Home</title>
  <meta name="description" content="Tarkov Stash" />
</svelte:head>

<div class="container container-center">
  <h1>
    Welcome to <span class="highlight">tarkov-stash</span> mod
  </h1>
  <h4>Your SPT server must be running, enter the connection details:</h4>
  <form>
    <input name="host" type="text" bind:value={hostValue} />
    <input name="port" type="number" bind:value={portValue} />
    <button type="submit" on:click={connectToServer}>connect</button>
  </form>
  {#if sessions.length > 0}
    <h5>Select profile to load:</h5>
    {#each sessions as session}
      <div class="session">
        <div>{session.username} ({session.id})</div>
        <button on:click={() => loadProfile(session)}>load</button>
      </div>
    {/each}
  {/if}
</div>

<style>
  .session {
    display: flex;
    justify-content: space-between;
    gap: 16px;
  }
</style>
