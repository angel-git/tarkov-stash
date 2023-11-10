import { invoke } from '@tauri-apps/api';
import { loading } from '../store';

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
