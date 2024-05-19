<script lang="ts">
  import { profile } from '../../../store';
  import BodyItem from '../item/body-item.svelte';
  import type { Item } from '../../../types';

  function handleDbClickOnItem(item: Item) {}

  function getPocketItem(pocketId: string) {
    return $profile?.bodyItems.pockets?.gridItems?.find((i) => i.name === pocketId)?.items[0];
  }

  function getSpecialSlotItem(slotId: string) {
    return $profile?.bodyItems.pockets?.slotItems?.find((i) => i.slotId === slotId);
  }
</script>

<div class="grid">
  <div class="tacticalVest">
    <BodyItem
      title="tactical rig"
      item={$profile?.bodyItems.tacticalVest}
      onDblClick={handleDbClickOnItem}
    />
  </div>
  <div class="pocket1">
    <BodyItem title="" item={getPocketItem('pocket1')} onDblClick={handleDbClickOnItem} />
  </div>
  <div class="pocket2">
    <BodyItem title="" item={getPocketItem('pocket2')} onDblClick={handleDbClickOnItem} />
  </div>
  <div class="pocket3">
    <BodyItem title="" item={getPocketItem('pocket3')} onDblClick={handleDbClickOnItem} />
  </div>
  <div class="pocket4">
    <BodyItem title="" item={getPocketItem('pocket4')} onDblClick={handleDbClickOnItem} />
  </div>

  <div class="special1">
    <BodyItem title="" item={getSpecialSlotItem('SpecialSlot1')} onDblClick={handleDbClickOnItem} />
  </div>
  <div class="special2">
    <BodyItem title="" item={getSpecialSlotItem('SpecialSlot2')} onDblClick={handleDbClickOnItem} />
  </div>
  <div class="special3">
    <BodyItem title="" item={getSpecialSlotItem('SpecialSlot3')} onDblClick={handleDbClickOnItem} />
  </div>

  <div class="backpack">
    <BodyItem
      title="backpack"
      item={$profile?.bodyItems.backpack}
      onDblClick={handleDbClickOnItem}
    />
  </div>

  <div class="securedContainer">
    <BodyItem
      title="pouch"
      item={$profile?.bodyItems.securedContainer}
      onDblClick={handleDbClickOnItem}
    />
  </div>
</div>

<style>
  .grid {
    display: grid;
    grid-template-columns: repeat(4, 64px);
    grid-template-rows: repeat(8, 64px);
    grid-column-gap: 8px;
    grid-row-gap: 16px;
  }

  .tacticalVest {
    grid-area: 1 / 1 / 3 / 3;
  }

  .pocket1 {
    grid-area: 3 / 1 / 4 / 2;
  }

  .pocket2 {
    grid-area: 3 / 2 / 4 / 3;
  }

  .pocket3 {
    grid-area: 3 / 3 / 4 / 4;
  }

  .pocket4 {
    grid-area: 3 / 4 / 4 / 5;
  }

  .special1 {
    grid-area: 4 / 1 / 5 / 2;
  }
  .special2 {
    grid-area: 4 / 2 / 5 / 3;
  }
  .special3 {
    grid-area: 4 / 3 / 5 / 4;
  }

  .backpack {
    grid-area: 5 / 1 / 7 / 3;
  }

  .securedContainer {
    grid-area: 7 / 1 / 9 / 3;
  }
</style>
