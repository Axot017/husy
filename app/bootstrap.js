window.walletSelector = require('@near-wallet-selector/core').default;
window.setupNearWallet = require("@near-wallet-selector/near-wallet").setupNearWallet;
window.setupSender =  require("@near-wallet-selector/sender").setupSender;
window.setupMathWallet =  require("@near-wallet-selector/math-wallet").setupMathWallet;
window.setupLedger =  require("@near-wallet-selector/ledger").setupLedger;

import("./pkg").then(module => {
  module.run_app();
});
