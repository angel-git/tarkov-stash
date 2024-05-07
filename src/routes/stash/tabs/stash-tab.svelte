<script lang="ts">
  import StashGrid from '../stash-grid.svelte';
  import AmountModal from '../modal/modal-amount.svelte';
  import DeleteModal from '../modal/modal-delete.svelte';
  import DetailsModal from '../modal/modal-details.svelte';
  import Loading from '$lib/images/loading.gif';
  import { loading, profile } from '../../../store';
  import type { Item, Option, Profile } from '../../../types';
  import { invokeWithLoader } from '../../../helper';
  import { goto } from '$app/navigation';

  $: isLoading = $loading;
  let selectedOption: Option | undefined;
  let selectedItem: Item | undefined;

  function handleOptionClicked(option: Option, item: Item) {
    switch (option) {
      case 'fir': {
        invokeWithLoader('change_fir', { item })
          .catch((e) => goto(`/error?message=${e}`))
          .finally(() => {
            handleCloseModal();
          });
        break;
      }
      case 'resource': {
        invokeWithLoader('restore_durability', { item })
          .catch((e) => goto(`/error?message=${e}`))
          .finally(() => {
            handleCloseModal();
          });
        break;
      }
      default: {
        selectedOption = option;
        selectedItem = item;
        break;
      }
    }
  }

  function handleCloseModal() {
    selectedOption = undefined;
    selectedItem = undefined;
  }

  function areItemsAreOutsideBounds(p: Profile) {
    return p.items.some((i) => i.y >= p.sizeY || i.x >= p.sizeX);
  }
</script>

{#if areItemsAreOutsideBounds($profile)}
  <h4 class="error">Stash size UNKNOWN some items might be missing</h4>
{:else}
  <h4>
    Your current stash size is {$profile.sizeX}x{$profile.sizeY}
  </h4>
{/if}
{#if isLoading}
  <img class="loading" src={Loading} alt="loading gif" width="40" height="40" />
{/if}
{#if selectedItem && selectedOption && selectedOption === 'details'}
  <DetailsModal
    item={selectedItem}
    bsgItems={$profile.bsgItems}
    locale={$profile.locale}
    onClose={handleCloseModal}
  />
{/if}
{#if selectedItem && selectedOption && selectedOption === 'amount'}
  <AmountModal item={selectedItem} onClose={handleCloseModal} />
{/if}
{#if selectedItem && selectedOption && selectedOption === 'delete'}
  <DeleteModal item={selectedItem} locale={$profile.locale} onClose={handleCloseModal} />
{/if}
<StashGrid profile={$profile} onOptionClicked={handleOptionClicked} />

<style>
  .loading {
    position: fixed;
    top: 12px;
    right: 12px;
    z-index: 10;
  }

  h4.error {
    color: var(--color-destructive);
  }
</style>
