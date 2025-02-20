<script lang="ts">
	import Header from "$lib/components/header.svelte";

    let compressUrl = $state("");
    let decompressId = $state("");
	let compressResult = $state("");
	let decompressResult = $state("");

	async function compress() {}
	async function decompress() {}
</script>

<Header />
<main>
	<div class="main">
		<div class="contents-block">
			<div class="search">
				<input
					type="url"
					spellcheck="false"
					placeholder="ここにURLを入力"
					autocomplete="off"
                    bind:value={compressUrl}
				/><button onclick={compress}>短縮</button>
				<div>
					<p>{compressResult}</p>
				</div>
			</div>
		</div>
		<div class="contents-block">
			<div class="search search-decompress">
				<p class="contents-title">短縮したURLを解凍することも出来ます。</p>
				<input
					type="url"
					spellcheck="false"
					placeholder="ここに短縮したURLのIDを入力"
					autocomplete="off"
                    bind:value={decompressId}
				/><button onclick={decompress}>解凍</button><br />
				<small>https://jli.li/rdNwqj の rdNwqj部分がIDです。</small>
				<div>
					<p>{decompressResult}</p>
				</div>
			</div>
		</div>
		<div class="contents-block">
			<div class="about">
				<div class="about-title"><p><b>なぜURLを短縮するのですか？</b></p></div>
				<div class="about-description">
					<p>
						SNSでURLを貼る時にURLが長いせいで文字数制限のせいで投稿できないといった経験はありませんか？
					</p>
					<p>そんな時に使えるのがURL短縮サービスです。</p>
					<p>以下のようにurlを短縮することが出来ます。</p>
					<p>https://www.thunlights.com/ → https://jli.li/rdNwqj</p>
					<p>これにより、文字数制限のあるSNSでの投稿の幅がより広がります。</p>
					<small>※元々短いURLに関しては元のURLの方が短くなる可能性があります。</small>
				</div>
			</div>
		</div>
		<div class="contents-block">
			<div class="about">
				<div class="about-title"><p><b>Q & A</b></p></div>
				<div class="about-description">
					<b><p>Q.本当に安全ですか？</p></b>
					<p>本サイトはオープンソースです。 GitHubにて全てのコードを公開しています。</p>
					<p>詳しくは<a href="https://github.com/thunlights/jli">こちら</a>をご覧ください</p>
					<b><p>Q.作成したリンクは永久に機能しますか？</p></b>
					<p>はい。このサービスが終わらない限り永久に機能します。</p>
					<p>悪意のあるURLでない限り削除されることもありません</p>
					<b><p>Q.短縮URLの削除申請はどのようにしたらいいですか？</p></b>
					<p><a href="https://x.com/thunlights">公式Twitter</a>のDMにて可能です。</p>
					<p>リンク先が悪意のある場合にのみ削除対応可能です。</p>
				</div>
			</div>
		</div>
	</div>

	<div class="footer">
		<p>
			This project is publicly available in Github <a href="https://github.com/thunlights/jli"
				>code</a
			>
		</p>
		<p>Supported By <a href="https://www.thunlights.com">Team ThunLights</a></p>
	</div>
</main>

<style>
	@font-face {
		font-family: "Dela Gothic One";
		font-display: swap;
		src: url("/fonts/DelaGothicOne-Regular.ttf") format("truetype");
	}

	@font-face {
		font-family: "M PLUS 1p";
		font-display: swap;
		src: url("/fonts/MPLUS1p-Bold.ttf") format("truetype");
	}

	@font-face {
		font-family: "Noto Serif JP";
		font-display: swap;
		src: url("/fonts/NotoSerifJP-Regular.ttf") format("truetype");
	}

	.main {
		display: block;
	}

	.main .contents-block {
		width: 100%;
	}

	.main .contents-block .contents-title {
		font-family: "M PLUS 1p";
		text-align: center;
	}

	.main .contents-block .search {
		text-align: center;
		margin: 30px auto;
	}

	.main .contents-block .search input,
	.main .contents-block .search button {
		background-color: #f7f7f7;
		border: 1px solid hsla(0, 0%, 78%, 0.5);
		border-radius: 0;
		color: #333;
		padding: 5px 15px;
		margin: 35px auto;
		box-sizing: border-box;
		font-family: inherit;
		font-size: inherit;
		height: 40px !important;
	}

	.main .contents-block .search-decompress input,
	.main .contents-block .search-decompress button {
		margin: 0 auto;
	}

	.main .contents-block .search input {
		width: 450px;
		cursor: text;
	}

	.main .contents-block .search button {
		cursor: pointer;
	}

	.main .contents-block .search button:hover {
		background-color: #dbdbdb;
	}

	.main .contents-block .about {
		width: 85%;
		margin: 5px auto;
	}

	.main .contents-block .about .about-title {
		font-family: "M PLUS 1p";
		font-size: 20px;
		margin-top: 5px;
	}

	.main .contents-block .about .about-description {
		font-family: "Noto Serif JP";
		font-size: 16px;
	}

	.main .contents-block .about .about-description b p {
		margin-top: 10px;
	}

	.main .contents-block .about .about-description p {
		margin: 6px 0;
	}

	.main .contents-block .about .about-description a {
		color: black;
	}

	.footer {
		color: #9ca3af;
		background: #1d1e22;
		margin-top: 50px;
	}

	.footer p {
		padding: 5px 8px;
	}

	.footer a {
		color: #c1c6ce;
	}

	@media screen and (max-width: 550px) {
		.main .contents-block .search input {
			width: 380px;
		}
	}

	@media screen and (max-width: 480px) {
		.main .contents-block .search input {
			width: 300px;
		}
	}

	@media screen and (max-width: 390px) {
		.main .contents-block .search input {
			width: 250px;
		}
	}

	@media screen and (max-width: 350px) {
		.main .contents-block .search input {
			width: 200px;
		}
	}
</style>
