import { useParams } from '@solidjs/router';
import type { Component } from 'solid-js';

import styles from './UrlInfoPage.module.css';

type Params = {
    id: string;
};

export const UrlInfoPage: Component = () => {
    const params = useParams<Params>();

    return <div>UrlInfoPage: {params.id}</div>;
};
