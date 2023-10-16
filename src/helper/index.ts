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
			return `rgba(63, 63, 147, 0.1)`;
		case 'green':
			return `rgba(55, 119, 55, 0.1)`;
		case 'grey':
			return `rgba(50, 50, 50, 0.1)`;
		case 'orange':
			return `rgba(255, 182, 115, 0.1)`;
		case 'red':
			return `rgba(255, 0, 0, 0.1)`;
		case 'violet':
			return `rgba(155, 38, 182, 0.1)`;
		case 'yellow':
			return `rgba(246, 235, 97, 0.1)`;
		default:
			return `rgba(0, 0, 0, 0.0)`;
	}
};
