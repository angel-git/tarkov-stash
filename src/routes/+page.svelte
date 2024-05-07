<script lang="ts">
  import { listen } from '@tauri-apps/api/event';
  import { goto } from '$app/navigation';
  import { invokeWithLoader } from '../helper';
  import { profile, loading } from '../store';
  import type { Profile, Session } from '../types';
  import Loading from '$lib/images/loading.gif';

  let hostValue = '127.0.0.1';
  let portValue = 6969;

  let sessions: Array<Session>;
  let selectedSession: Session | undefined;
  $: selectedSession = undefined;
  $: connected = false;
  $: isLoading = $loading;
  $: sessions = [];

  listen('profile_loaded', (event) => {
    if (selectedSession && selectedSession.id === event.payload) {
      reloadProfile(selectedSession);
    }
  });
  listen('error', (event) => {
    goto(`/error?message=${event.payload}`);
  });

  function connectToServer() {
    invokeWithLoader<Array<Session>>('connect_to_server', {
      server: { host: hostValue, port: portValue },
    })
      .then((r: Array<Session>) => {
        connected = true;
        sessions = r;
      })
      .catch((errorMessage) => goto(`/error?message=${errorMessage}`));
  }

  function reloadProfile(session: Session) {
    invokeWithLoader<Profile>('refresh_profile_from_spt', { session })
      .then((r) => {
        profile.set(r);
        goto('/stash');
      })
      .catch((errorMessage) => goto(`/error?message=${errorMessage}`));
  }

  function loadProfile(session: Session) {
    selectedSession = session;
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
  {#if isLoading}
    <img class="loading" src={Loading} alt="loading gif" width="40" height="40" />
  {/if}
  <form>
    <input name="host" type="text" bind:value={hostValue} />
    <input name="port" type="number" bind:value={portValue} />
    <button class="primary" type="submit" on:click={connectToServer}>connect</button>
  </form>
  {#if connected}
    <h5>Select profile to load:</h5>

    <div>
      If you don't see your profile here after connecting, make sure you started Tarkov with that
      profile and create the character. If you think your profile should be there, check this mod
      logs on the help menu
    </div>
    {#if sessions.length > 0}
      <div class="sessions">
        {#each sessions as session}
          <div class="session">
            <div>{session.username} ({session.id})</div>
            <button class="primary" on:click={() => loadProfile(session)}>load</button>
          </div>
        {/each}
      </div>
    {/if}
  {/if}
</div>

<style>
  .loading {
    position: fixed;
    top: 12px;
    right: 12px;
    z-index: 10;
  }

  .sessions {
    display: flex;
    flex-direction: column;
    gap: 16px;
  }

  .sessions .session {
    display: flex;
    justify-content: space-between;
  }

  .sessions .session:hover {
    color: var(--color-highlight);
  }
</style>
