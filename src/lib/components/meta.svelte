<script lang="ts">
	type Props = {
		title?: string;
		description?: string;
	};

	const props: Props = $props();
	const title = props.title ?? "URL短縮サービス";
	const description =
		props.description ?? "URL短縮サービスです。どんなに長いURLでも小さくまとめることが出来ます。";
</script>

<svelte:head>
	<meta name="description" content={description} />
	<meta name="keywords" content="URL短縮サービス, jli.li, jli, URL短縮, 短縮" />
	<meta name="theme-color" content="#008b8b" />
	<title>{title} / jli.li</title>

	<meta property="og:url" content="https://jli.li" />
	<meta property="og:type" content="website" />
	<meta property="og:title" content="{title} / jli.li" />
	<meta property="og:description" content={description} />
	<meta property="og:site_name" content="URL短縮サービス / jli.li" />

	<meta name="format-detection" content="telephone=no,email=no,address=no" />

	<meta name="twitter:card" content="summary_large_image" />
	<meta name="twitter:site" content="@thunlights" />
	<meta name="twitter:creator" content="@thunlights" />
</svelte:head>

<style></style>
