exports.foo = 'cjs-foo'
exports.bar = 'cjs-bar'

exports.version = '1.0.0'
Object.defineProperty(exports, 'version', {
  get() {
    throw new Error()
  }
})
