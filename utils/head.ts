
export type Props = {
	title?: string
	description?: string
}

export function setHead(props: Props) {
	const title = `${props.title ?? "URL短縮サービス"} / jli.li`;
    const description = props.description ?? "URL短縮サービスです。どんなに長いURLでも小さくまとめることが出来ます。";
	useHead({
        title,
        meta: [
            { charset: "UTF-8" },
            { name: "viewport", content: "width=device-width, initial-scale=1.0" },
            { name: "description", content: description },
            { name: "keywords", content: "URL短縮サービス, jli.li, jli, URL短縮, 短縮" },
            { name: "theme-color", content: "#008b8b" },

            { property: "og:url", content: "https://jli.li" },
            { property: "og:type", content: "website" },
            { property: "og:title", content: title },
            { property: "og:description", content: description },
            { property: "og:site:name", content: "URL短縮サービス / jli.li" },

            { name: "format-detection", content: "telephone=no,email=no,address=no" },

            { name: "twitter:card", content: "summary_large_image" },
            { name: "twitter:site", content: "@thunlights" },
            { name: "twitter:creator", content: "@thunlights" },
        ],
    });
}
