const { keyStores, connect, WalletConnection } = require("near-api-js");

const initialize = async (config) => {
    if (window._near && window._wallet) {
        return;
    }

    const nearConfig = {
        ...config,
        keyStore: new keyStores.BrowserLocalStorageKeyStore(), 
    };
    window._near = await connect(nearConfig);
    window._wallet = new WalletConnection(window._near);
};

const requestSignIn = () => {
    return window._wallet.requestSignIn();
};

const isLoggedIn = () => {
    return window._wallet.isSignedIn();
};

const init = () => {
    window.nearInitialize = initialize;
    window.isLoggedIn = isLoggedIn;
    window.requestSignIn = requestSignIn;
};

module.exports = {
    init
}
