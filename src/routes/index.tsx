import { component$ } from '@builder.io/qwik';

export default component$(() => {
    return (
        <>
            <div class="container container-center container-spacing-xl">
                <h3>
                    Welcome to <span class="highlight">tarkov-stash</span> mod
                </h3>
                <h4>Select your profile file to start editing it</h4>

                <input type="file" id="profile" name="profile" accept="application/json" />
            </div>

            <div class="container container-flex"></div>
        </>
    );
});
