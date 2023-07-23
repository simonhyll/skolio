// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  ssr: false,
  srcDir: 'src',
  devtools: { enabled: false },
  css: ["~/assets/scss/main.scss", '@fortawesome/fontawesome-svg-core/styles.css']
})
