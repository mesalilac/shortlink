import { useParams } from '@solidjs/router';
import type { Component } from 'solid-js';

import styles from './UrlPage.module.css';

type Params = {
    id: string;
};

export const UrlPage: Component = () => {
    const params = useParams<Params>();

    return <div>UrlPage: {params.id}</div>;
};
