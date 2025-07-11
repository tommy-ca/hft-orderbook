const { join } = require('path');
const { loadBinding } = require('@node-rs/helper');

module.exports = loadBinding(join(__dirname, '../../rust_core'), 'lob_rs', 'lob_rs');
