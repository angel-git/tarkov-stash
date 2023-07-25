import { component$, useStore, useVisibleTask$ } from '@builder.io/qwik';
import { listen } from '@tauri-apps/api/event';
import type { IProfile } from '~/types';
import { Stash } from '~/components/stash/stash';
import { useNavigate } from '@builder.io/qwik-city';

export default component$(() => {
    const nav = useNavigate();
    const p = useStore<IProfile>({ profile: undefined });

    useVisibleTask$(() => {
        listen('profile_loaded', () => {
            nav(`/stash`);
        });
        listen('error', (event) => {
            nav(`/error?message=${event.payload}`);
        });
    });

    if (p.profile) {
        return <Stash profile={p.profile} />;
    }

    // TODO verify server is not running
    return (
        <>
            <div class="container container-center container-spacing-xl">
                <h3>
                    Welcome to <span class="highlight">tarkov-stash</span> mod
                </h3>
                <h4>Select your profile from the menu to start editing it</h4>
            </div>
        </>
    );
});
