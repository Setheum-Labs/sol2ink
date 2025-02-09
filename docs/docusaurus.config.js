// @ts-check
// Note: type annotations allow type checking and IDEs autocompletion

const lightCodeTheme = require('prism-react-renderer/themes/github');
const darkCodeTheme = require('prism-react-renderer/themes/dracula');

/** @type {import('@docusaurus/types').Config} */
const config = {
  title: 'Sol2Ink',
  tagline: 'Sol2Ink documentation',
  url: 'https://Brushfam.github.io/sol2ink/',
  baseUrl: '/sol2ink/',
  onBrokenLinks: 'throw',
  onBrokenMarkdownLinks: 'warn',
  favicon: 'img/favicon.ico',
  organizationName: 'Brushfam', 
  projectName: 'sol2ink',
  deploymentBranch: 'gh-pages',

  themeConfig: {
    colorMode: {
      defaultMode: 'dark'
    },
    navbar: {
      logo: {
        alt: 'Sol2Ink',
        src: 'img/logo.svg',
        srcDark: 'img/logo-dark.svg'
      },
      items: [
        {
          href: 'https://twitter.com/727_ventures',
          className: 'header-twitter-link',
          position: 'right'
        },
        {
          href: 'https://github.com/Brushfam/sol2ink',
          className: 'header-github-link',
          position: 'right'
        }
      ]
    },
    footer: {
      copyright: `Copyright © ${new Date().getFullYear()} Sol2Ink, Supercolony.net.`
    },
    prism: {
      theme: lightCodeTheme,
      darkTheme: darkCodeTheme,
      additionalLanguages: ['toml', 'rust']
    }
  },
  plugins: ['docusaurus-plugin-sass'],
  presets: [
    [
      '@docusaurus/preset-classic',
      {
        docs: {
          routeBasePath: '/',
          sidebarPath: require.resolve('./sidebars.js'),
          editUrl: 'https://github.com/Brushfam/sol2ink/tree/main/docs'
        },
        theme: {
          customCss: [require.resolve('./src/css/custom.scss')]
        }
      }
    ]
  ]
};

module.exports = config;
