const getWsHostName = () => process.env.JIRS_SERVER_BIND === "0.0.0.0" ? 'localhost' : process.env.JIRS_SERVER_BIND;
const getProtocol = () => window.location.protocol.replace(/^http/, 'ws');
const wsUrl = () => `${getProtocol()}//${getWsHostName()}:${process.env.JIRS_SERVER_PORT}/ws/`;

import("/jirs.js").then(async module => {
    // window.module = module;
    await module.default();
    const host_url = `${location.protocol}//${process.env.JIRS_SERVER_BIND}:${process.env.JIRS_SERVER_PORT}`;
    module.render(host_url, wsUrl());
    document.querySelector('main').className = '';
    document.querySelector('.spinner').remove();
});

