<script lang="ts">
  import BodyItem from '../item/body-item.svelte';
  import { profile } from '../../../store';
  import type { Item } from '../../../types';
  import DetailsModal from '../modal/modal-details.svelte';

  let selectedDetailsItem: Item | undefined = undefined;

  function handleDbClickOnItem(item: Item | undefined) {
    selectedDetailsItem = item;
  }

  function handleCloseModal() {
    selectedDetailsItem = undefined;
  }
</script>

<div class="inventory">
  {#if selectedDetailsItem}
    <DetailsModal
      item={selectedDetailsItem}
      bsgItems={$profile.bsgItems}
      locale={$profile.locale}
      onClose={handleCloseModal}
    />
  {/if}
  <div class="grid">
    <BodyItem
      title="earpierce"
      item={$profile?.bodyItems.earpierce}
      onDblClick={handleDbClickOnItem}
    />
    <BodyItem
      title="headwear"
      item={$profile?.bodyItems.headwear}
      onDblClick={handleDbClickOnItem}
    />
    <BodyItem
      title="face cover"
      item={$profile?.bodyItems.faceCover}
      onDblClick={handleDbClickOnItem}
    />
    <BodyItem title="armband" item={$profile?.bodyItems.armBand} onDblClick={handleDbClickOnItem} />
    <BodyItem
      title="body armor"
      item={$profile?.bodyItems.armorVest}
      onDblClick={handleDbClickOnItem}
    />
    <BodyItem title="eyewear" item={$profile?.bodyItems.eyewear} onDblClick={handleDbClickOnItem} />
    <div class="weapon-sling">
      <BodyItem
        title="on sling"
        item={$profile?.bodyItems.primaryWeapon}
        onDblClick={handleDbClickOnItem}
      />
    </div>
    <div class="weapon-back">
      <BodyItem
        title="on back"
        item={$profile?.bodyItems.secondaryWeapon}
        onDblClick={handleDbClickOnItem}
      />
    </div>
    <BodyItem title="holster" item={$profile?.bodyItems.holster} onDblClick={handleDbClickOnItem} />
    <BodyItem title="sheath" item={$profile?.bodyItems.sheath} onDblClick={handleDbClickOnItem} />
  </div>
</div>

<style>
  .inventory {
    display: flex;
    justify-content: center;
  }

  .grid {
    display: grid;
    grid-template-columns: repeat(3, 150px);
    grid-template-rows: repeat(4, 178px);
    grid-column-gap: 32px;
    grid-row-gap: 16px;
  }

  .weapon-back {
    grid-area: 4 / 1 / 5 / 3;
  }

  .weapon-sling {
    grid-area: 3 / 1 / 4 / 3;
  }
</style>
