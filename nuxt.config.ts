
export default defineNuxtConfig({
	compatibilityDate: "2024-11-01",
	devtools: { enabled: true },
	runtimeConfig: {
		public: {
			idSize: 6
		}
	},
	components: [
		{
			path: '~/components',
			pathPrefix: false,
		},
	],
})
