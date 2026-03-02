import { useParams } from '@solidjs/router';
import {
    type Component,
    createMemo,
    createResource,
    Match,
    Show,
    Switch,
} from 'solid-js';
import type { GetUrlInfoResponse } from '@/consts';

import styles from './UrlInfoPage.module.css';

type Params = {
    id: string;
};

export const UrlInfoPage: Component = () => {
    const params = useParams<Params>();
    const [info] = createResource<GetUrlInfoResponse>(async () => {
        const res = await fetch(`/api/url/${params.id}/info`);

        if (!res.ok) {
            throw new Error(await res.text());
        }

        const data: GetUrlInfoResponse = await res.json();

        return data;
    });

    return (
        <div>
            <Show when={info.loading}>
                <p>Loading...</p>
            </Show>
            <Switch>
                <Match when={info.error}>
                    <span>Error: {info.error.message}</span>
                </Match>
                <Match when={info()}>
                    {(info) => {
                        const createdAt = createMemo(() =>
                            new Date(info().createdAt).toString(),
                        );
                        const lastClickedAt = createMemo(() => {
                            const lastClickedAt = info().lastClickedAt;
                            if (lastClickedAt !== undefined)
                                return new Date(lastClickedAt).toString();

                            return undefined;
                        });

                        return (
                            <>
                                <h1>Url info</h1>
                                <div class='flex-row'>
                                    <span>id:</span>
                                    <span class={styles.label_value}>
                                        {params.id}
                                    </span>
                                </div>
                                <div class='flex-row'>
                                    <span>Short Url:</span>
                                    <a href={info().shortUrl}>
                                        {info().shortUrl}
                                    </a>
                                </div>
                                <div class='flex-row'>
                                    <span>Long Url:</span>
                                    <a href={info().longUrl}>
                                        {info().longUrl}
                                    </a>
                                </div>
                                <div class='flex-row'>
                                    <span>Clicks:</span>
                                    <span class={styles.label_value}>
                                        {info().clicks}
                                    </span>
                                </div>
                                <div class='flex-row'>
                                    <span>Expires At:</span>
                                    <span class={styles.label_value}>
                                        {info().expiresAt}
                                    </span>
                                </div>
                                <div class='flex-row'>
                                    <span>Max Clicks:</span>
                                    <span class={styles.label_value}>
                                        {info().maxClicks}
                                    </span>
                                </div>
                                <div class='flex-row'>
                                    <span>Disabled:</span>
                                    <span class={styles.label_value}>
                                        {info().disabled.toString()}
                                    </span>
                                </div>
                                <div class='flex-row'>
                                    <span>Last Clicked At:</span>
                                    <span class={styles.label_value}>
                                        {lastClickedAt()}
                                    </span>
                                </div>
                                <div class='flex-row'>
                                    <span>Created At:</span>
                                    <span class={styles.label_value}>
                                        {createdAt()}
                                    </span>
                                </div>
                            </>
                        );
                    }}
                </Match>
            </Switch>
        </div>
    );
};
