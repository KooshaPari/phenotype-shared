export function createSiteMeta({ base = '/' } = {}) {
  return {
    base,
    title: 'phenotype-shared',
    description: 'Documentation',
    themeConfig: {
      nav: [
        { text: 'Home', link: base || '/' },
      ],
    },
  }
}
