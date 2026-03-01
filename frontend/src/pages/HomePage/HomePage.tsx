import { useParams } from '@solidjs/router';
import type { Component } from 'solid-js';

import styles from './HomePage.module.css';

type Params = {
    id: string;
};

export const HomePage: Component = () => {
    const params = useParams<Params>();

    return <div>HomePage: {params.id}</div>;
};
