import { invoke } from '@tauri-apps/api';
import { loading } from '../store';
import type { Item, SlotKind } from '../types';

import empty from '$lib/images/empty-slot.png';
import patron_in_weapon from '$lib/images/attachments/patron_in_weapon.png';
import tactical from '$lib/images/attachments/tactical.png';
import scope from '$lib/images/attachments/scope.png';
import sight from '$lib/images/attachments/sight.png';
import mount from '$lib/images/attachments/mount.png';
import charge from '$lib/images/attachments/charge.png';
import barrel from '$lib/images/attachments/barrel.png';
import stock from '$lib/images/attachments/stock.png';
import handguard from '$lib/images/attachments/handguard.png';
import pistol_grip from '$lib/images/attachments/pistol-grip.png';
import receiver from '$lib/images/attachments/receiver.png';
import muzzle from '$lib/images/attachments/muzzle.png';
import foregrip from '$lib/images/attachments/foregrip.png';
import gas_block from '$lib/images/attachments/gas-block.png';
import launcher from '$lib/images/attachments/launcher.png';

type InvokeArgs = Record<string, unknown>;

export async function invokeWithLoader<T>(cmd: string, args?: InvokeArgs): Promise<T> {
  loading.set(true);
  try {
    return await invoke<T>(cmd, args);
  } finally {
    loading.set(false);
  }
}

export const calculateBackgroundColor = (backgroundColor: string) => {
  switch (backgroundColor) {
    case 'black':
      return `rgba(0, 0, 0, 0.3)`;
    case 'blue':
      return `rgba(28, 65, 86, 0.3)`;
    case 'green':
      return `rgba(21, 45, 0, 0.3)`;
    case 'grey':
      return `rgba(29, 29, 29, 0.3)`;
    case 'orange':
      return `rgba(60, 25, 0, 0.3)`;
    case 'red':
      return `rgba(109, 36, 24, 0.3)`;
    case 'violet':
      return `rgba(76, 42, 85, 0.3)`;
    case 'yellow':
      return `rgba(104, 102, 40, 0.3)`;
    default:
      return `rgba(127, 127, 127, 0.0)`;
  }
};

export const getShortName = (id: string, locale: Record<string, string>): string => {
  return locale[`${id} ShortName`];
};

export const getName = (id: string, locale: Record<string, string>): string => {
  return locale[`${id} Name`];
};

export const getDescription = (id: string, locale: Record<string, string>): string => {
  return locale[`${id} Description`];
};

export const getAttachmentBackground = (slotId: SlotKind) => {
  let image = empty;
  switch (slotId) {
    case 'mod_barrel':
      image = barrel;
      break;
    case 'mod_bipod':
      break;
    case 'mod_catch':
      break;
    case 'mod_charge':
      image = charge;
      break;
    case 'mod_equipment':
      break;
    case 'mod_equipment_000':
      break;
    case 'mod_equipment_001':
      break;
    case 'mod_equipment_002':
      break;
    case 'mod_flashlight':
      image = tactical;
      break;
    case 'mod_foregrip':
      image = foregrip;
      break;
    case 'mod_gas_block':
      image = gas_block;
      break;
    case 'mod_hammer':
      break;
    case 'mod_handguard':
      image = handguard;
      break;
    case 'mod_launcher':
      image = launcher;
      break;
    case 'mod_magazine':
      image = patron_in_weapon;
      break;
    case 'mod_mount':
      image = mount;
      break;
    case 'mod_mount_000':
      image = mount;
      break;
    case 'mod_mount_001':
      image = mount;
      break;
    case 'mod_mount_002':
      image = mount;
      break;
    case 'mod_mount_003':
      image = mount;
      break;
    case 'mod_mount_004':
      image = mount;
      break;
    case 'mod_mount_005':
      image = mount;
      break;
    case 'mod_mount_006':
      image = mount;
      break;
    case 'mod_muzzle':
      image = muzzle;
      break;
    case 'mod_muzzle_000':
      image = muzzle;
      break;
    case 'mod_muzzle_001':
      image = muzzle;
      break;
    case 'mod_nvg':
      break;
    case 'mod_pistol_grip':
      image = pistol_grip;
      break;
    case 'mod_pistol_grip_akms':
      image = pistol_grip;
      break;
    case 'mod_pistolgrip':
      image = pistol_grip;
      break;
    case 'mod_pistolgrip_000':
      image = pistol_grip;
      break;
    case 'mod_reciever':
      image = receiver;
      break;
    case 'mod_scope':
      image = scope;
      break;
    case 'mod_scope_000':
      image = scope;
      break;
    case 'mod_scope_001':
      image = scope;
      break;
    case 'mod_scope_002':
      image = scope;
      break;
    case 'mod_scope_003':
      image = scope;
      break;
    case 'mod_sight_front':
      image = sight;
      break;
    case 'mod_sight_rear':
      image = sight;
      break;
    case 'mod_stock':
      image = stock;
      break;
    case 'mod_stock_000':
      image = stock;
      break;
    case 'mod_stock_001':
      image = stock;
      break;
    case 'mod_stock_002':
      image = stock;
      break;
    case 'mod_stock_akms':
      image = stock;
      break;
    case 'mod_stock_axis':
      image = stock;
      break;
    case 'mod_tactical':
      image = tactical;
      break;
    case 'mod_tactical001':
      image = tactical;
      break;
    case 'mod_tactical002':
      image = tactical;
      break;
    case 'mod_tactical_000':
      image = tactical;
      break;
    case 'mod_tactical_001':
      image = tactical;
      break;
    case 'mod_tactical_002':
      image = tactical;
      break;
    case 'mod_tactical_003':
      image = tactical;
      break;
    case 'mod_tactical_004':
      image = tactical;
      break;
    case 'mod_tactical_2':
      image = tactical;
      break;
    case 'mod_trigger':
      break;
    case 'patron_in_weapon':
      image = patron_in_weapon;
      break;
    default:
      return empty;
  }

  return image;
};

export const findNewItemLocation = (
  width: number,
  height: number,
  grid: Array<Array<Item | undefined>>,
) => {
  const sizeY = grid.length;
  const sizeX = grid[0].length;

  for (let row = 0; row <= sizeY - height; row++) {
    for (let col = 0; col <= sizeX - width; col++) {
      let hasSpace = true;
      for (let i = 0; i < height && hasSpace; i++) {
        for (let j = 0; j < width; j++) {
          if (grid[row + i][col + j]) {
            hasSpace = false;
            break;
          }
        }
      }

      if (hasSpace) {
        return { x: col, y: row };
      }
    }
  }
  return null;
};
