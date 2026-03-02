import { useParams } from '@solidjs/router';
import { type Component, createResource, Match, Show, Switch } from 'solid-js';
import type { GetUrlResponse } from '@/consts';
import styles from './UrlPage.module.css';

type Params = {
    id: string;
};

export const UrlPage: Component = () => {
    const params = useParams<Params>();
    const [redirect] = createResource<GetUrlResponse>(async () => {
        const res = await fetch(`/api/url/${params.id}`);

        if (!res.ok) {
            throw new Error(await res.text());
        }

        const data: GetUrlResponse = await res.json();

        window.location.href = data.longUrl;

        return data;
    });

    return (
        <div>
            <Show when={redirect.loading}>
                <span>Loading...</span>
            </Show>
            <Switch>
                <Match when={redirect.error}>
                    <span>Error: {redirect.error.message}</span>
                </Match>
                <Match when={redirect()}>
                    <span>Redirecting...</span>
                </Match>
            </Switch>
        </div>
    );
};
