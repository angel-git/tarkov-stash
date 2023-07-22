import { component$ } from '@builder.io/qwik';
import { invoke } from '@tauri-apps/api/tauri';

const loadFile = (file?: File) => {
    if (!file) {
        // TODO handle exception on UI
        console.error('no file??');
        return;
    }
    const reader = new FileReader();
    reader.addEventListener('loadend', () => {
        invoke('load_profile_file', { content: reader.result })
            .then((result) => {
                console.info('okkkk', result);
            })
            .catch((e) => {
                console.error('eeeee', e);
            });
    });
    reader.readAsText(file, 'UTF-8');
};

export default component$(() => {
    return (
        <>
            <div class="container container-center container-spacing-xl">
                <h3>
                    Welcome to <span class="highlight">tarkov-stash</span> mod
                </h3>
                <h4>Select your profile file to start editing it</h4>

                <input type="file" id="profile" name="profile" accept="application/json" onChange$={(e) => loadFile(e.target.files?.[0])} />
            </div>

            <div class="container container-flex"></div>
        </>
    );
});
