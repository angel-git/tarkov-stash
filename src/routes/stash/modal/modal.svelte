<script lang="ts">
  export let showModal: boolean;
  export let onConfirm: () => void;
  export let withSubmit = true;
  export let fullHeight = false;

  let dialog: HTMLDialogElement;

  $: if (dialog && showModal) dialog.showModal();
  $: if (dialog && !showModal) dialog.close();
</script>

<!-- svelte-ignore a11y-click-events-have-key-events a11y-no-noninteractive-element-interactions -->
<dialog
  bind:this={dialog}
  on:close={() => (showModal = false)}
  on:click|self={() => (showModal = false)}
  class={fullHeight ? 'full' : ''}
>
  <!-- svelte-ignore a11y-no-static-element-interactions -->
  <div on:click|stopPropagation>
    <slot name="header" />
    <slot />
    <div class="controls">
      <button class="secondary" on:click={() => (showModal = false)}>Close</button>
      {#if withSubmit}
        <button class="primary" on:click={onConfirm}>Confirm</button>
      {/if}
    </div>
  </div>
</dialog>

<style>
  dialog {
    color: var(--color-text);
    background-color: var(--color-background);
    width: 90%;
    border-radius: 0.2em;
    border: none;
    padding: 1em;
  }

  dialog.full {
    height: 90%;
  }

  dialog::backdrop {
    background: rgba(0, 0, 0, 0.5);
  }

  dialog > div {
    display: flex;
    flex-direction: column;
  }

  dialog.full > div {
    height: 100%;
  }

  dialog[open] {
    animation: zoom 0.3s cubic-bezier(0.34, 1.56, 0.64, 1);
  }

  @keyframes zoom {
    from {
      transform: scale(0.8);
    }
    to {
      transform: scale(1);
    }
  }

  dialog[open]::backdrop {
    animation: fade 0.2s ease-out;
  }

  @keyframes fade {
    from {
      opacity: 0;
    }
    to {
      opacity: 1;
    }
  }

  .controls {
    display: flex;
    justify-content: space-between;
    margin-top: auto;
  }
</style>
