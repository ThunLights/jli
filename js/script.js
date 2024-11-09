
async function compresser() {
	document.getElementById("compress-result").innerHTML = ""

	const originalUrl = document.getElementById("compress-url").value
	const url = new URL(originalUrl);
	if (url.hostname === "jli.li") {
		let element = document.createElement("p")
		element.style.color = "red"
		element.textContent = "ドメインがjli.liのURLは短縮することが出来ません"
		document.getElementById("compress-result").appendChild(element)
		return;
	};
	const response = await fetch(
		`/api/compress`,
		{
			method: "POST",
			headers: {
				"Content-Type": "application/json"
			},
			body: JSON.stringify({
				link: originalUrl,
			})
		}
	)

	if (response.status === 200) {
		const json = await response.json()
		const compressId = json.id
		const compressUrl = `https://jli.li/${compressId}`
		const resultText = originalUrl.length < compressUrl.length ? `元URLのほうがサイズが小さいので元URLを使うのをおすすめします。` : `圧縮に成功 ${originalUrl.length}文字 → ${compressUrl.length}文字 に圧縮しました`

		let title = document.createElement("p")
		title.classList.add("result-title")
		title.textContent = originalUrl.length === compressUrl.length ? `元URLとサイズは変わりませんでした。(両方: ${compressUrl.length}文字)` : resultText

		let result = document.createElement("a")
		result.href = compressUrl
		result.classList.add("result-compress")
		result.textContent = compressUrl

		document.getElementById("compress-result").appendChild(title)
		document.getElementById("compress-result").appendChild(result)
	} else {
		let element = document.createElement("p")
		element.style.color = "red"
		element.textContent = "エラーが発生し短縮に失敗しました。"
		document.getElementById("compress-result").appendChild(element)
	}
}

async function decompresser() {
	document.getElementById("decompress-result").innerHTML = ""

	let id = document.getElementById("decompress-url").value

	if (id.match(/https:\/\/jli\.li\/(.+)/)) {
		id = id.match(/https:\/\/jli\.li\/(.+)/)[1]
	}

	const response = await fetch(
		`/api/decompress`,
		{
			method: "POST",
			headers: {
				"Content-Type": "application/json"
			},
			body: JSON.stringify({
				id: id,
			})
		}
	)

	if (response.status === 200) {
		const json = await response.json()

		let result = document.createElement("p")
		result.classList.add("result-compress")
		result.textContent = `元URL: ${json.link}`

		document.getElementById("decompress-result").appendChild(result)
	} else {
		let element = document.createElement("p")
		element.style.color = "red"
		element.textContent = "解凍に失敗しました。登録されていないURLの可能性があります。"
		document.getElementById("decompress-result").appendChild(element)
	}
}

document.getElementById("compress").onclick = compresser
document.getElementById("compress-url").addEventListener("keyup", async (e) => {
	if (e.key === "Enter") {
		await compresser()
	}
})
document.getElementById("decompress").onclick = decompresser
document.getElementById("decompress-url").addEventListener("keyup", async (e) => {
	if (e.key === "Enter") {
		await decompresser()
	}
})
