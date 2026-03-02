import { type Component, createSignal, For, Show } from 'solid-js';
import type { CreateShortUrlResponse } from '@/consts';

import styles from './HomePage.module.css';

const Url = (props: { url: CreateShortUrlResponse }) => {
    return (
        <div class='flex-row'>
            <div class='flex-row'>
                <a
                    class={styles.url_info_href}
                    href={`${props.url.shortUrl}/info`}
                >
                    (INFO)
                </a>
                <a class={styles.url_href} href={props.url.shortUrl}>
                    {props.url.shortUrl}
                </a>
            </div>
            {'->'}
            <a class={styles.url_href} href={props.url.longUrl}>
                {props.url.longUrl}
            </a>
        </div>
    );
};

export const HomePage: Component = () => {
    const [urls, setUrls] = createSignal<CreateShortUrlResponse[]>([]);
    const [longUrl, setLongUrl] = createSignal<string>('');
    const [maxClicks, setMaxClicks] = createSignal<number>();
    const [expiresAt, setExpiresAt] = createSignal<number>();
    const [error, setError] = createSignal<string>();
    const [loading, setLoading] = createSignal<boolean>(false);

    const createShortUrl = async () => {
        if (!longUrl()) return;

        setLoading(true);
        setError(undefined);

        try {
            const res = await fetch('/api/shorten', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({
                    url: longUrl(),
                    maxClicks: maxClicks(),
                    expiresAt: expiresAt(),
                }),
            });

            if (!res.ok) {
                const text = await res.text();
                console.error(text);
                setError(text);
                setLoading(false);
                return;
            }

            const data: CreateShortUrlResponse = await res.json();

            setUrls([...urls(), data]);

            setLoading(false);
        } catch (err) {
            console.error(err);
            setError((err as Error).message);
            setLoading(false);
        }
    };

    return (
        <div>
            <h1>URL Shortener</h1>
            <div
                style={{
                    height: '2px',
                    width: '100%',
                    'background-color': 'gray',
                }}
            />

            <br />

            <Show when={error()}>
                <p>Error: {error()}</p>
                <br />
            </Show>

            <div class='flex-row'>
                <input
                    onInput={(e) => setLongUrl(e.currentTarget.value)}
                    placeholder='https://example.com'
                    type='text'
                    value={longUrl()}
                />
                <button
                    disabled={loading()}
                    onClick={createShortUrl}
                    type='button'
                >
                    {loading() ? 'Loading...' : 'Shorten URL'}
                </button>
            </div>
            <br />
            <div class='flex-row'>
                <span>Max Clicks</span>
                <input
                    onInput={(e) => setMaxClicks(e.currentTarget.valueAsNumber)}
                    placeholder='Max Clicks'
                    type='number'
                    value={maxClicks()}
                />
            </div>
            <br />
            <div class='flex-row'>
                <span>Expires At</span>
                <input
                    onChange={(e) =>
                        setExpiresAt(new Date(e.currentTarget.value).getTime())
                    }
                    type='date'
                />
            </div>
            <br />

            <For each={urls()}>{(url) => <Url url={url} />}</For>
        </div>
    );
};
