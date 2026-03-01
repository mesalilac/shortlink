import { Route, Router } from '@solidjs/router';
import type { Component } from 'solid-js';
import { HomePage, UrlInfoPage, UrlPage } from '@/pages';

import './App.css';
import './utils.css';

const App: Component = () => {
    return (
        <Router>
            <Route component={HomePage} path='/' />
            <Route component={UrlPage} path='/:id' />
            <Route component={UrlInfoPage} path='/:id/info' />
        </Router>
    );
};

export default App;
