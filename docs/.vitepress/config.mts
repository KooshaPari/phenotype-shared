import { createPhenotypeConfig } from '../../vendor/phenodocs/packages/docs/src/config/index.ts'

export default createPhenotypeConfig({
  title: 'phenotype-shared',
  description: 'Shared Phenotype infrastructure components',
  githubRepo: 'phenotype-shared',
  nav: [
    { text: 'Guide', link: '/guide/' },
    { text: 'Reference', link: '/reference/' },
  ],
  sidebar: {
    '/guide/': [
      {
        text: 'Guide',
        items: [
          { text: 'Getting Started', link: '/guide/' },
        ],
      },
    ],
    '/reference/': [
      {
        text: 'Reference',
        items: [
          { text: 'FR Tracker', link: '/reference/FR_TRACKER' },
          { text: 'Code Entity Map', link: '/reference/CODE_ENTITY_MAP' },
        ],
      },
    ],
  },
})
