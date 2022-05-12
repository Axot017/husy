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
    window.near_initialize = initialize;
    window.is_logged_in = isLoggedIn;
    window.request_sign_in = requestSignIn;
};

module.exports = {
    init
}
