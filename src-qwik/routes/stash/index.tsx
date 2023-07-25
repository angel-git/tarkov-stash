import {component$, useStore, useVisibleTask$} from '@builder.io/qwik';
import {useNavigate} from '@builder.io/qwik-city';
import {invoke} from '@tauri-apps/api';
import {Stash} from "~/components/stash/stash";
import type {IProfile, Profile} from "~/types";

export default component$(() => {
    // In order to access the `routeLoader$` data within a Qwik Component, you need to call the hook.
    const nav = useNavigate();
    const p = useStore<IProfile>({ profile: undefined });


    useVisibleTask$(() => {
        invoke('load_profile_file', {})
            .then((result) => {
                p.profile = result as Profile;
            })
            .catch((e) => {
                nav(`/error?message=${e}`)
            });
    }, {strategy: "document-ready"})

    if (!p.profile) {
        return <p>loading</p>
    }

    return <Stash profile={p.profile} />;
});
