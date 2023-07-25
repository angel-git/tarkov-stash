import { component$ } from '@builder.io/qwik';
import { useLocation } from '@builder.io/qwik-city';

export default component$(() => {
    const loc = useLocation();

    return (
        <>
            <div class="container container-center container-spacing-xl">
                <h3>Something went wrong</h3>
            </div>
            <div class="container">
                <p>{loc.url.searchParams.get("message")}</p>
            </div>
        </>
    );
});
