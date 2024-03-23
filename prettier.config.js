/** @type {import("prettier").Config} */
const config = {
  trailingComma: 'es5',
  semi: true,
  tabWidth: 2,
  plugins: [
    'prettier-plugin-svelte',
    'prettier-plugin-organize-imports',
    'prettier-plugin-tailwindcss',
  ],
};

export default config;
