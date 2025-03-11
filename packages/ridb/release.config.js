import Config, { branches, plugins } from '../../release.base.js'
export default {
  ...Config,
  tagFormat: '@trust0/ridb@${version}',
  branches: branches,
  plugins: [
    [
      "@semantic-release/exec",
      {
        "prepareCmd": "sh ../../update.sh"
      }
    ],
    ...plugins
  ],
}