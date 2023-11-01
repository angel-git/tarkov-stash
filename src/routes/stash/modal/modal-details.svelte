<script lang="ts">
  import type { BsgItem, Item, SlotKind, Stats } from '../../../types';
  import Modal from './modal.svelte';
  import { calculateBackgroundColor } from '../../../helper';

  // images
  import ergonomicsLogo from '$lib/images/ergonomics.png';
  import accuracyLogo from '$lib/images/accuracy.png';
  import sightingRangeLogo from '$lib/images/sighting-range.png';
  import recoilLogo from '$lib/images/recoil.png';
  import muzzleVelocityLogo from '$lib/images/muzzle-velocity.png';

  export let onClose: () => void;
  export let item: Item;
  export let bsgItems: Record<string, BsgItem>;

  let showModal = true;

  $: if (!showModal) onClose();

  function findItemInSlot(slotName: SlotKind) {
    return item.slotItems?.find((slotItem) => slotItem.slotId === slotName);
  }

  function calculateStats(): Stats | undefined {
    const bsgItem = bsgItems[item.tpl];
    if (!bsgItem) return;
    const initialStats: Stats = {
      ergonomics: bsgItem.ergonomics || 0,
      accuracy: bsgItem.deviationMax || 0,
      sightingRange: bsgItem.sightingRange || 0,
      verticalRecoil: bsgItem.recoilForceUp || 0,
      horizontalRecoil: bsgItem.recoilForceBack || 0,
      muzzleVelocity: bsgItem.muzzleVelocity || 0,
    };
    return item.slotItems?.reduce((acc, value) => {
      const slotItem = bsgItems[value.tpl];
      if (!slotItem) return acc;
      acc.ergonomics += slotItem.ergonomics;
      acc.accuracy += slotItem.accuracy;
      acc.sightingRange =
        acc.sightingRange > slotItem.sightingRange ? acc.sightingRange : slotItem.sightingRange;
      acc.verticalRecoil += slotItem.recoil; // TODO test with real profile
      acc.horizontalRecoil += slotItem.recoil; // TODO test with real profile
      acc.muzzleVelocity += slotItem.velocity;
      return acc;
    }, initialStats) as Stats;
  }

  const stats = calculateStats();

  console.log('item', item);

  function handleConfirm() {
    showModal = false;
  }
</script>

<Modal bind:showModal onConfirm={handleConfirm}>
  <h2 slot="header">
    {bsgItems[item.tpl].name}
  </h2>

  <div>
    <div class="img-details">
      <div class="img">
        <img alt="item" src={`https://assets.tarkov.dev/${item.tpl}-base-image.png`} />
      </div>
      <div class="details">
        {bsgItems[item.tpl].description}
      </div>
    </div>
    {#if stats}
      <div class="stats">
        {#if stats.ergonomics}
          <div class="stat-wrapper">
            <div class="stat-name">
              <img alt="ergonomics logo" src={ergonomicsLogo} />
              <div>ERGONOMICS</div>
            </div>
            <div class="graph-line" style={`width: ${stats.ergonomics / 1.5}%`} />
            <div class="stat-value">{stats.ergonomics}</div>
          </div>
        {/if}
        {#if stats.accuracy}
          <div class="stat-wrapper">
            <div class="stat-name">
              <img alt="ergonomics logo" src={accuracyLogo} />
              <div>ACCURACY</div>
            </div>
            <div class="graph-line" style={`width: ${stats.accuracy / 0.35}%`} />
            <div class="stat-value">{stats.accuracy} MOA</div>
          </div>
        {/if}
        {#if stats.sightingRange}
          <div class="stat-wrapper">
            <div class="stat-name">
              <img alt="ergonomics logo" src={sightingRangeLogo} />
              <div>SIGHTING RANGE</div>
            </div>
            <div class="graph-line" style={`width: ${stats.sightingRange / 20}%`} />
            <div class="stat-value">{stats.sightingRange}</div>
          </div>
        {/if}
        {#if stats.verticalRecoil}
          <div class="stat-wrapper">
            <div class="stat-name">
              <img alt="ergonomics logo" src={recoilLogo} />
              <div>VERTICAL RECOIL</div>
            </div>
            <div class="graph-line" style={`width: ${stats.verticalRecoil / 7}%`} />
            <div class="stat-value">{stats.verticalRecoil}</div>
          </div>
        {/if}
        {#if stats.horizontalRecoil}
          <div class="stat-wrapper">
            <div class="stat-name">
              <img alt="ergonomics logo" src={recoilLogo} />
              <div>HORIZONTAL RECOIL</div>
            </div>
            <div class="graph-line" style={`width: ${stats.horizontalRecoil / 12}%`} />
            <div class="stat-value">{stats.horizontalRecoil}</div>
          </div>
        {/if}
        {#if stats.muzzleVelocity}
          <div class="stat-wrapper">
            <div class="stat-name">
              <img alt="ergonomics logo" src={muzzleVelocityLogo} />
              <div>MUZZLE VELOCITY</div>
            </div>
            <div class="graph-line" style={`width: ${stats.muzzleVelocity / 13}%`} />
            <div class="stat-value">{Math.floor(stats.muzzleVelocity)} m/s</div>
          </div>
        {/if}
      </div>
    {/if}
    <div class="slots">
      <div class="slots-grid">
        {#each bsgItems[item.tpl].Slots as slot}
          {@const itemInSlot = findItemInSlot(slot._name)}
          {#if itemInSlot}
            <div
              class="slots-grid-item with-item"
              style={`background-color: ${calculateBackgroundColor(
                bsgItems[itemInSlot.tpl].backgroundColor,
              )}`}
            >
              <div class="slots-grid-item-name">{bsgItems[itemInSlot.tpl].shortName}</div>
              <img alt="item" src={`https://assets.tarkov.dev/${itemInSlot.tpl}-base-image.png`} />
            </div>
          {:else}
            <div class="slots-grid-item">
              <div class="slots-grid-item-name">{slot.name}</div>
            </div>
          {/if}
        {/each}
      </div>
    </div>
  </div>
</Modal>

<style>
  .img-details {
    padding: 16px;
  }

  .img-details .details {
    text-align: justify;
  }

  .stats {
    padding: 16px;
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .stats .stat-wrapper {
    border: 1px solid var(--color-background);
    outline: 1px solid #575b5e;
    position: relative;
    display: flex;
    justify-content: space-between;
  }

  .stats .stat-wrapper .stat-name {
    display: flex;
    gap: 8px;
  }

  .stats .stat-wrapper .stat-name img {
    width: 18px;
    height: 18px;
  }

  .stats .stat-wrapper .graph-line {
    background: linear-gradient(180deg, #575b5e 0%, #3a3f41 100%);
    position: absolute;
    top: 0;
    height: 100%;
    z-index: -1;
  }

  .slots-grid {
    display: flex;
    gap: 8px;
    flex-wrap: wrap;
    justify-content: center;
  }

  .slots-grid-item {
    background-image: url($lib/images/empty-slot.png);
    height: 64px;
    width: 64px;
    display: flex;
    justify-content: center;
    position: relative;
  }

  .slots-grid-item.with-item {
    background-image: none;
  }

  .slots-grid-item-name {
    position: absolute;
    font-size: 10px;
    right: 0;
  }

  .slots-grid-item img {
    max-height: 100%;
    max-width: 100%;
    margin: 8px;
  }
</style>
