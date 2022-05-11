const near = require('./js/near');
const bundlr = require('./js/bundlr');

near.init();
bundlr.init();

import("./pkg").then(module => {
  module.run_app(process.env);
});
