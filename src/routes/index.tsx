import { $, component$, useStore } from '@builder.io/qwik';
import { invoke } from '@tauri-apps/api/tauri';
import type {IProfile, Profile} from '~/types';
import { Stash } from '~/components/stash/stash';
import {useNavigate} from "@builder.io/qwik-city";



export default component$(() => {
    const nav = useNavigate();
    const p = useStore<IProfile>({profile: undefined});


    const loadFile = $((file?: File) => {
        if (!file) {
            console.error('no file??');
            return;
        }
        const reader = new FileReader();
        reader.addEventListener('loadend', () => {
            invoke('load_profile_file', { content: reader.result })
                .then((result) => {
                    p.profile = result as Profile;
                })
                .catch((e) => {
                    console.error(e);
                    nav('/error')
                });
        });
        reader.readAsText(file, 'UTF-8');
    });

    if (p.profile) {
        return <Stash profile={p.profile} />
    }

    // TODO verify server is not running
    return (
        <>
            <div class="container container-center container-spacing-xl">
                <h3>
                    Welcome to <span class="highlight">tarkov-stash</span> mod
                </h3>
                <h4>Select your profile file to start editing it</h4>

                <input type="file" id="profile" name="profile" accept="application/json" onChange$={(e) => loadFile(e.target.files?.[0])} />
            </div>
        </>
    );
});
