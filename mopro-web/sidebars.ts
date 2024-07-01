import type { SidebarsConfig } from '@docusaurus/plugin-content-docs';

const sidebars: SidebarsConfig = {
  docsSidebar: [
    {
      type: 'doc',
      label: 'Introduction',
      id: 'intro',
    },
    {
      type: 'doc',
      label: 'Prerequisites',
      id: 'prerequisites',
    },
    {
      type: 'doc',
      label: 'Getting Started',
      id: 'getting-started',
    },
    {
      type: 'doc',
      label: 'Mopro Core',
      id: 'mopro-core',
    },
    {
      type: 'doc',
      label: 'Mopro FFI',
      id: 'mopro-ffi',
    },
    {
      type: 'doc',
      label: 'Mopro CLI',
      id: 'mopro-cli',
    },
    {
      type: 'category',
      label: 'Circom',
      items: [
        {
          type: 'doc',
          label: 'configuration',
          id: 'circom/configuration',
        },
        {
          type: 'doc',
          label: 'Core API',
          id: 'circom/core-api',
        },
        {
          type: 'doc',
          label: 'iOS',
          id: 'circom/ios',
        },
        {
          type: 'doc',
          label: 'Android',
          id: 'circom/android',
        },
      ],
    },
    {
      type: 'category',
      label: 'Halo2',
      items: [
        {
          type: 'doc',
          label: 'configuration',
          id: 'halo2/configuration',
        },
        {
          type: 'doc',
          label: 'introduction',
          id: 'halo2/introduction',
        },
      ],
    },
    {
      type: 'doc',
      label: 'Performance and Benchmarks',
      id: 'performance',
    },
    {
      type: 'doc',
      label: 'Community and Talks',
      id: 'community',
    },
    {
      type: 'doc',
      label: 'FAQ',
      id: 'FAQ',
    },
  ],
};

export default sidebars;