const WebBundlr = require("@bundlr-network/client").WebBundlr;

const initialize = async (node, currency) => {
    if (window._bundlr) {
        return;
    }

    window._bundlr = new WebBundlr(node, currency, window._wallet);
    await window._bundlr.ready()
};

const init = () => {
    window.bundlr_initialize = initialize;
};

module.exports = {
    init
}
