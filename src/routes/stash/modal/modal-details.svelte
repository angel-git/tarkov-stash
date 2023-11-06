<script lang="ts">
  import type { BsgItem, Item, Stats } from '../../../types';
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

  function findItemsInSlot(slotId: string) {
    return item.slotItems?.filter((slotItem) => slotItem.slotId === slotId);
  }

  function calculateStats(): Stats | undefined {
    if (!item.slotItems) return undefined;

    const bsgItem = bsgItems[item.tpl];
    const initialStats: Stats = {
      ergonomics: bsgItem.ergonomics,
      accuracy: bsgItem.deviationMax,
      sightingRange: bsgItem.sightingRange,
      verticalRecoil: bsgItem.recoilForceUp,
      horizontalRecoil: bsgItem.recoilForceBack,
      singleFireRate: bsgItem.singleFireRate,
      verticalRecoilPercentage: 0,
      horizontalRecoilPercentage: 0,
      velocityPercentage: 0,
      centerOfImpactPercentage: 0,
    };
    const stats = item.slotItems?.reduce((acc, value) => {
      const slotItem = bsgItems[value.tpl];
      acc.ergonomics += slotItem.ergonomics;
      acc.centerOfImpactPercentage += slotItem.centerOfImpact;
      acc.sightingRange =
        acc.sightingRange > slotItem.sightingRange ? acc.sightingRange : slotItem.sightingRange;
      acc.verticalRecoilPercentage += slotItem.recoil;
      acc.horizontalRecoilPercentage += slotItem.recoil;
      acc.velocityPercentage += slotItem.velocity;
      return acc;
    }, initialStats) as Stats;

    stats.verticalRecoil =
      bsgItem.recoilForceUp + (bsgItem.recoilForceUp * stats.verticalRecoilPercentage) / 100;
    stats.horizontalRecoil =
      bsgItem.recoilForceBack + (bsgItem.recoilForceBack * stats.horizontalRecoilPercentage) / 100;
    stats.singleFireRate =
      bsgItem.singleFireRate + (bsgItem.singleFireRate * stats.velocityPercentage) / 100;
    stats.accuracy = 34.38 * stats.centerOfImpactPercentage;

    return stats;
  }

  function mergeSlots() {
    const bsgItemSlots = bsgItems[item.tpl].Slots || [];
    const itemSlots = item.slotItems || [];
    const bsgNames = bsgItemSlots.map((s) => s._name);
    const itemNames = itemSlots.map((s) => s.slotId);
    return new Set([...bsgNames, ...itemNames]);
  }

  const stats = calculateStats();
  const slots = mergeSlots();

  console.log('item', item); // TODO delete me

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
            <div class="graph-line" style={`width: ${stats.ergonomics}%`} />
            <div class="stat-value">{stats.ergonomics}</div>
          </div>
        {/if}
        {#if stats.accuracy}
          <div class="stat-wrapper">
            <div class="stat-name">
              <img alt="ergonomics logo" src={accuracyLogo} />
              <div>ACCURACY (aprox)</div>
            </div>
            <div class="graph-line" style={`width: ${100 - stats.accuracy / 0.35}%`} />
            <div class="stat-value">{Math.round(stats.accuracy)} MOA</div>
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
            <div class="stat-value">{Math.ceil(stats.verticalRecoil)}</div>
          </div>
        {/if}
        {#if stats.horizontalRecoil}
          <div class="stat-wrapper">
            <div class="stat-name">
              <img alt="ergonomics logo" src={recoilLogo} />
              <div>HORIZONTAL RECOIL</div>
            </div>
            <div class="graph-line" style={`width: ${stats.horizontalRecoil / 12}%`} />
            <div class="stat-value">{Math.ceil(stats.horizontalRecoil)}</div>
          </div>
        {/if}
        {#if stats.singleFireRate}
          <div class="stat-wrapper">
            <div class="stat-name">
              <img alt="ergonomics logo" src={muzzleVelocityLogo} />
              <div>MUZZLE VELOCITY (aprox)</div>
            </div>
            <div class="graph-line" style={`width: ${stats.singleFireRate / 13}%`} />
            <div class="stat-value">{Math.floor(stats.singleFireRate)} m/s</div>
          </div>
        {/if}
      </div>
    {/if}
    <div class="slots">
      <div class="slots-grid">
        {#if slots}
          {#each slots as slotId}
            {@const itemsInSlot = findItemsInSlot(slotId)}
            {#if itemsInSlot && itemsInSlot.length}
              {#each itemsInSlot as itemInSlot}
                <div
                  id={itemInSlot.tpl}
                  class="slots-grid-item with-item"
                  style={`background-color: ${calculateBackgroundColor(
                    bsgItems[itemInSlot.tpl].backgroundColor,
                  )}`}
                >
                  <div class="slots-grid-item-name">{bsgItems[itemInSlot.tpl].shortName}</div>
                  <img
                    alt="item"
                    src={`https://assets.tarkov.dev/${itemInSlot.tpl}-base-image.png`}
                  />
                </div>
              {/each}
            {:else}
              <div class="slots-grid-item">
                <div class="slots-grid-item-name">{slotId}</div>
              </div>
            {/if}
          {/each}
        {/if}
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
    padding: 16px;
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
    border: 1px solid #575b5e;
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
