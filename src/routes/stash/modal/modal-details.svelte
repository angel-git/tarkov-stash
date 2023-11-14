<script lang="ts">
  import type { BsgItem, Item, SlotKind, Stats } from '../../../types';
  import Modal from './modal.svelte';
  import { calculateBackgroundColor, getAttachmentBackground, getName } from '../../../helper';

  // images
  import ergonomicsLogo from '$lib/images/ergonomics.png';
  import accuracyLogo from '$lib/images/accuracy.png';
  import sightingRangeLogo from '$lib/images/sighting-range.png';
  import recoilLogo from '$lib/images/recoil.png';
  import muzzleVelocityLogo from '$lib/images/muzzle-velocity.png';
  import { getDescription, getShortName } from '../../../helper';

  export let onClose: () => void;
  export let item: Item;
  export let bsgItems: Record<string, BsgItem>;
  export let locale: Record<string, string>;

  let showModal = true;

  $: if (!showModal) onClose();

  ///// accuracy
  function getCenterOfImpactDelta() {
    const acc =
      item.slotItems?.reduce((acc, value) => {
        const slotItem = bsgItems[value.tpl];
        return acc + slotItem.accuracy;
      }, 0) || 0;
    return -(acc / 100);
  }

  function getWeaponBarrel(): BsgItem | undefined {
    const mod_barrel = item.slotItems?.find((item) => item.slotId === 'mod_barrel');
    if (mod_barrel) {
      return bsgItems[mod_barrel.tpl];
    }
  }

  function getBarrelDeviation(): number {
    // TODO item.durability * WeaponSpread
    return calculateDeviation(item.resource || 0) * 1;
  }

  function getCenterOfImpactBase() {
    return getWeaponBarrel()?.centerOfImpact || bsgItems[item.tpl].centerOfImpact;
  }

  function calculateDeviation(durability: number): number {
    const template = bsgItems[item.tpl];
    const deviationCurve = template.deviationCurve;
    const num = 2 * deviationCurve;
    const num2 =
      100 - num == 0
        ? durability / num
        : (0 -
            deviationCurve +
            Math.sqrt((0 - num + 100) * durability + deviationCurve * deviationCurve)) /
          (0 - num + 100);
    const num3 = 1 - num2;
    const num4 = getWeaponBarrel()?.deviationMax || template.deviationMax;
    return num3 * num3 * num4 + 2 * num2 * num3 * deviationCurve + num2 * num2;
  }

  function getAmmoFactor(): number | undefined {
    // TODO ideally we should look for other places with bullets like chamber first
    const cartridges = item.slotItems?.find((item) => item.slotId === 'cartridges');
    if (cartridges) {
      const ammo = bsgItems[cartridges.tpl];
      const ammoAccr = ammo.ammoAccr || 0;
      if (ammoAccr <= 0) {
        return (100 + Math.abs(ammoAccr)) / 100;
      }
      return 100 / (100 + ammoAccr);
    }
  }

  function getTotalCenterOfImpact(includeAmmo = true): number {
    const num = getCenterOfImpactBase() * (1 + getCenterOfImpactDelta());
    if (!includeAmmo) {
      return num;
    }
    return num * (getAmmoFactor() ?? 1);
  }

  function getAccuracy() {
    return (getTotalCenterOfImpact(true) * getBarrelDeviation() * 100) / 2.9089;
  }
  /////

  ///// velocity
  function getVelocityBase(): number {
    // TODO ideally we should look for other places with bullets like chamber first
    const cartridges = item.slotItems?.find((item) => item.slotId === 'cartridges');
    if (cartridges) {
      return bsgItems[cartridges.tpl].initialSpeed || 0;
    }
    return 0;
  }

  function getVelocityDelta() {
    const bsgItem = bsgItems[item.tpl];
    const acc =
      item.slotItems?.reduce((acc, value) => {
        const slotItem = bsgItems[value.tpl];
        return acc + slotItem.velocity;
      }, 0) || 0;
    return (acc + bsgItem.velocity) / 100;
  }

  function getSpeedFactor() {
    return 1 + getVelocityDelta();
  }

  function getVelocity() {
    return getVelocityBase() * getSpeedFactor();
  }

  /////

  function findItemsInSlot(slotId: string) {
    return item.slotItems?.filter((slotItem) => slotItem.slotId === slotId);
  }

  function calculateStats(): Stats | undefined {
    if (!item.slotItems) return undefined;

    const bsgItem = bsgItems[item.tpl];
    const initialStats: Stats = {
      ergonomics: bsgItem.ergonomics,
      accuracy: getAccuracy(),
      sightingRange: bsgItem.sightingRange,
      verticalRecoil: bsgItem.recoilForceUp,
      horizontalRecoil: bsgItem.recoilForceBack,
      velocity: getVelocity(),
      verticalRecoilPercentage: 0,
      horizontalRecoilPercentage: 0,
    };
    const stats = item.slotItems?.reduce((acc, value) => {
      const slotItem = bsgItems[value.tpl];
      acc.ergonomics += slotItem.ergonomics;
      acc.sightingRange =
        acc.sightingRange > slotItem.sightingRange ? acc.sightingRange : slotItem.sightingRange;
      acc.verticalRecoilPercentage += slotItem.recoil;
      acc.horizontalRecoilPercentage += slotItem.recoil;
      return acc;
    }, initialStats) as Stats;

    stats.verticalRecoil =
      bsgItem.recoilForceUp + (bsgItem.recoilForceUp * stats.verticalRecoilPercentage) / 100;
    stats.horizontalRecoil =
      bsgItem.recoilForceBack + (bsgItem.recoilForceBack * stats.horizontalRecoilPercentage) / 100;

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

  function getEmptyAttachmentBackgroundUrl(slotId: string) {
    return getAttachmentBackground(slotId as SlotKind);
  }

  function getImageUrl() {
    return item.presetImageId
      ? `https://assets.tarkov.dev/${item.presetImageId}-base-image.png`
      : `https://assets.tarkov.dev/${item.tpl}-base-image.png`;
  }
</script>

<Modal bind:showModal withSubmit={false}>
  <h2 slot="header">
    {getName(item.tpl, locale)}
  </h2>

  <div>
    <div class="img-details">
      <div class="img">
        <img alt="item" src={getImageUrl()} />
      </div>
      <div class="details">
        {getDescription(item.tpl, locale)}
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
            <div class="graph-line" style={`width: ${stats.ergonomics / 1}%`} />
            <div class="stat-value">{stats.ergonomics}</div>
          </div>
        {/if}
        {#if stats.accuracy}
          <div class="stat-wrapper">
            <div class="stat-name">
              <img alt="ergonomics logo" src={accuracyLogo} />
              <div>ACCURACY</div>
            </div>
            <div class="graph-line" style={`width: ${100 - stats.accuracy / 0.35}%`} />
            <div class="stat-value">{Math.round(stats.accuracy * 100) / 100} MOA</div>
          </div>
        {/if}
        {#if stats.sightingRange}
          <div class="stat-wrapper">
            <div class="stat-name">
              <img alt="ergonomics logo" src={sightingRangeLogo} />
              <div>SIGHTING RANGE</div>
            </div>
            <div class="graph-line" style={`width: ${stats.sightingRange / 50}%`} />
            <div class="stat-value">{stats.sightingRange}</div>
          </div>
        {/if}
        {#if stats.verticalRecoil}
          <div class="stat-wrapper">
            <div class="stat-name">
              <img alt="ergonomics logo" src={recoilLogo} />
              <div>VERTICAL RECOIL</div>
            </div>
            <div class="graph-line" style={`width: ${stats.verticalRecoil / 10}%`} />
            <div class="stat-value">{Math.ceil(stats.verticalRecoil)}</div>
          </div>
        {/if}
        {#if stats.horizontalRecoil}
          <div class="stat-wrapper">
            <div class="stat-name">
              <img alt="ergonomics logo" src={recoilLogo} />
              <div>HORIZONTAL RECOIL</div>
            </div>
            <div class="graph-line" style={`width: ${stats.horizontalRecoil / 10}%`} />
            <div class="stat-value">{Math.ceil(stats.horizontalRecoil)}</div>
          </div>
        {/if}
        {#if stats.velocity}
          <div class="stat-wrapper">
            <div class="stat-name">
              <img alt="ergonomics logo" src={muzzleVelocityLogo} />
              <div>MUZZLE VELOCITY</div>
            </div>
            <div class="graph-line" style={`width: ${stats.velocity / 15}%`} />
            <div class="stat-value">{Math.floor(stats.velocity)} m/s</div>
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
                  <div class="slots-grid-item-name">{getShortName(itemInSlot.tpl, locale)}</div>
                  <img
                    alt="item"
                    src={`https://assets.tarkov.dev/${itemInSlot.tpl}-base-image.png`}
                  />
                </div>
              {/each}
            {:else}
              <div class="slots-grid-item">
                <div
                  class="slots-grid-item-empty"
                  style={`background-image: url(${getEmptyAttachmentBackgroundUrl(slotId)}`}
                />
                <div class="slots-grid-item-name">{locale[slotId.toUpperCase()]}</div>
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

  .slots-grid-item .slots-grid-item-empty {
    height: 64px;
    width: 64px;
    background-repeat: no-repeat;
    background-position: center;
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
