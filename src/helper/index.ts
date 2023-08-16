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
