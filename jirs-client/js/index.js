const getWsHostName = () => process.env.JIRS_SERVER_BIND === "0.0.0.0" ? 'localhost' : process.env.JIRS_SERVER_BIND;
const getProtocol = () => window.location.protocol.replace(/^http/, 'ws');
const wsUrl = () => `${ getProtocol() }//${ getWsHostName() }:${ process.env.JIRS_SERVER_PORT }/ws/`;

import("../pkg/index.js").then(module => {
    let queue = [];
    let ws;

    const buildWebSocket = () => {
        ws = new WebSocket(wsUrl());
        ws.binaryType = 'blob';
        ws.onopen = event => {
            console.log('open', event);
            module.reconnected();
        };
        ws.onerror = event => {
            console.error(event);
        };
        ws.onmessage = async event => {
            const arrayBuffer = await event.data.arrayBuffer();
            const array = new Uint8Array(arrayBuffer);
            module.handle_ws_message(array);
        };
        ws.onclose = () => {
            setTimeout(() => buildWebSocket(), 600);
        };
    };
    buildWebSocket();

    window.send_bin_code = code => queue.push(code);
    window.inspectQueue = () => queue;

    let wsCheckDelay = 100;
    const flush = () => {
        if (queue.length >= 1000) {
            ws.close();
            throw new Error("Message queue overflow");
        }
        // if (queue.length && wsCheckDelay <= 0) console.log(ws.readyState, queue);
        switch (ws.readyState) {
            case 1: {
                const [ code, ...rest ] = queue;
                queue = rest;
                if (code) {
                    // console.log('open', code);
                    ws.send(Uint8Array.from(code).buffer);
                }
                break;
            }
            default:
                break;
        }
        window.requestAnimationFrame(flush);
    };
    window.flush = flush;

    const keepWsOpen = () => {
        if (wsCheckDelay > 0) {
            wsCheckDelay -= 1;
        } else {
            wsCheckDelay = 100;
            switch (ws.readyState) {
                case 1: {
                    // console.log('sending ping');
                    // ws.send(Uint8Array.from([ 0, 0, 0, 0 ]).buffer);
                    break;
                }
                case 0:
                case 2:
                    break;
                case 3:
                    throw new Error('web socket has been closed');
                    buildWebSocket();
                    break;
            }
        }
        window.requestAnimationFrame(keepWsOpen);
    };

    keepWsOpen();
    flush();

    const host_url = `${ location.protocol }//${ process.env.JIRS_SERVER_BIND }:${ process.env.JIRS_SERVER_PORT }`;
    module.set_host_url(host_url);
    module.render();
});
