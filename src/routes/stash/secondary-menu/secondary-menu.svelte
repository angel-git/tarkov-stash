<script lang="ts">
  import { getName } from '../../../helper';
  import InspectIcon from '$lib/images/inspect.png';
  import OpenIcon from '$lib/images/open.png';
  import AmountIcon from '$lib/images/amount.png';
  import FirIcon from '$lib/images/fir.png';
  import RepairIcon from '$lib/images/repair.png';
  import DiscardIcon from '$lib/images/discard.png';
  import type { Item, Option } from '../../../types';
  import { contextMenu } from '../../../store';

  export let locale: Record<string, string>;
  export let onOptionClicked: (option: Option, item: Item) => void;

  let item = $contextMenu;

  function handleOptionClicked(option: Option, item: Item | undefined) {
    if (!item) return;
    contextMenu.set(undefined);
    onOptionClicked(option, item);
  }

  function clickOutside(element: any) {
    function onClick(event: any) {
      if (!element.contains(event.target)) {
        contextMenu.set(undefined);
      }
    }

    document.body.addEventListener('click', onClick);

    return {
      destroy() {
        document.body.removeEventListener('click', onClick);
      },
    };
  }
</script>

{#if item}
  <div class="options" use:clickOutside>
    <div class="title">{getName(item.tpl, locale)}</div>
    <!-- svelte-ignore a11y-click-events-have-key-events -->
    <div
      class="option"
      tabindex="-1"
      role="button"
      on:click={() => handleOptionClicked('details', item)}
    >
      <img alt="inspect logo" src={InspectIcon} />
      <div>Inspect</div>
    </div>
    {#if item.isContainer}
      <!-- svelte-ignore a11y-click-events-have-key-events -->
      <div
        class="option"
        tabindex="-1"
        role="button"
        on:click={() => handleOptionClicked('open', item)}
      >
        <img alt="open logo" src={OpenIcon} />
        <div>Open</div>
      </div>
    {/if}
    {#if item.isStockable}
      <!-- svelte-ignore a11y-click-events-have-key-events -->
      <div
        class="option"
        tabindex="-1"
        role="button"
        on:click={() => handleOptionClicked('amount', item)}
      >
        <img alt="amount logo" src={AmountIcon} />
        <div>Change amount</div>
      </div>
    {/if}
    {#if !item.isFir}
      <!-- svelte-ignore a11y-click-events-have-key-events -->
      <div
        class="option"
        tabindex="-1"
        role="button"
        on:click={() => handleOptionClicked('fir', item)}
      >
        <img alt="fir logo" src={FirIcon} />
        <div>Set fir</div>
      </div>
    {/if}
    {#if item.maxResource && item.maxResource !== 1 && item.resource !== item.maxResource}
      <!-- svelte-ignore a11y-click-events-have-key-events -->
      <div
        class="option"
        tabindex="-1"
        role="button"
        on:click={() => handleOptionClicked('resource', item)}
      >
        <img alt="repair logo" src={RepairIcon} />
        <div>Restore durability</div>
      </div>
    {/if}
    {#if !item.isContainer}
      <!-- svelte-ignore a11y-click-events-have-key-events -->
      <div
        class="option destructive"
        tabindex="-1"
        role="button"
        on:click={() => handleOptionClicked('delete', item)}
      >
        <img alt="discard logo" src={DiscardIcon} />
        <div>Discard</div>
      </div>
    {/if}
  </div>
{/if}

<style>
  .options {
    position: absolute;
    top: 10px;
    left: 10px;
    background-color: var(--color-background);
    border: 1px solid var(--color-background);
    font-size: 12px;
    z-index: 5;
    min-width: 170px;
  }

  .options .option {
    font-size: 11px;
    padding: 2px 10px;
    margin: 2px 0;
    border-top-left-radius: 6px;
    border-bottom-right-radius: 6px;
    text-transform: uppercase;
    background-color: var(--color-menu);
    display: flex;
    gap: 16px;
    justify-items: center;
    align-self: center;
    align-content: center;
    align-items: center;
  }

  .options .option img {
    max-height: 14px;
    max-width: 14px;
  }

  .options .option:hover img {
    filter: invert();
  }

  .options .option.destructive {
    color: var(--color-destructive);
  }

  .options .option.destructive:hover {
    color: var(--color-background);
    background-color: var(--color-destructive);
  }

  .options .title {
    font-weight: bold;
    border-bottom: 1px solid var(--color-menu);
  }

  .options .option:hover {
    cursor: pointer;
    background-color: var(--color-text);
    color: var(--color-background);
  }
</style>
