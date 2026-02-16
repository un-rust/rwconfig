import { defineConfig } from 'vitepress'

// https://vitepress.dev/reference/site-config
export default defineConfig({
  base: '/rwconfig/',
  title: 'rwconfig',
  description: 'Read/write config files with get/set and dirty-tracking; save() writes all changes at once.',

  themeConfig: {
    nav: [
      { text: 'Home', link: '/' },
      { text: 'Guide', link: '/guide' },
      { text: 'API', link: '/api' },
      { text: 'Formats', link: '/formats' },
    ],

    sidebar: [
      {
        text: 'Getting started',
        items: [
          { text: 'Guide', link: '/guide' },
          { text: 'Supported formats', link: '/formats' },
        ],
      },
      {
        text: 'Reference',
        items: [
          { text: 'API Reference', link: '/api' },
        ],
      },
    ],

    socialLinks: [
      { icon: 'github', link: 'https://github.com/betterhyq/rwconfig' },
    ],
  },
})
