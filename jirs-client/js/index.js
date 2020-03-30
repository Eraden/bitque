import "./styles.css";

import("../pkg/index.js").then(module => {
    const host_url = `${location.protocol}//${process.env.JIRS_SERVER_BIND}:${process.env.JIRS_SERVER_PORT}`;
    module.set_host_url(host_url);
    module.render();
});
