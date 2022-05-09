window.webBundlr = require("@bundlr-network/client").WebBundlr;
let { keyStores, connect, WalletConnection } = require("near-api-js");
window.nearKeyStores = keyStores;
window.nearConnect = connect;
window.nearWalletConnection = WalletConnection;

import("./pkg").then(module => {
  module.run_app(process.env);
});
