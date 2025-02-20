<script lang="ts"></script>

<header>
	<div class="header">
		<div class="header-block">
			<a href="/">
				<p class="header-title">JLI.li (URL 短縮サービス)</p>
			</a>
		</div>
	</div>
</header>

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

	.header {
		border-bottom: solid 1px black;
	}

	.header a {
		color: black;
		text-decoration: none;
	}

	.header .header-block {
		display: inline-block;
		margin: 14px 16px;
	}

	.header .header-block .header-title {
		font-family: "Dela Gothic One", sans-serif;
		font-size: 25px;
	}

	@media screen and (max-width: 390px) {
		.header .header-block .header-title {
			font-size: 20px;
		}
	}

	@media screen and (max-width: 350px) {
		.header .header-block .header-title {
			font-size: 18px;
		}
	}
</style>
