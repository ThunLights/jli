import JavaScriptObfuscator from "javascript-obfuscator"

import path from "path"
import fs from "fs"

function Obfuscator(content:string):string {
    let obfuscationResult = JavaScriptObfuscator.obfuscate(content,
        {
            compact: true,
            controlFlowFlattening: true,
            controlFlowFlatteningThreshold: 1
        }
    )
    return obfuscationResult.getObfuscatedCode()
}

async function ExchangeJS(dir: string, output: string) {
	const files = fs.readdirSync(dir)
	for (const file of files) {
		if (fs.statSync(path.join(dir, file)).isDirectory()) {
			await ExchangeJS(path.join(dir, file), path.join(output, file))
		} else {
			const contents = Obfuscator(fs.readFileSync(path.join(dir, file), "utf-8"))
			fs.writeFileSync(path.join(output, file), contents)
		}
	}
}

(async () => {
	await ExchangeJS(path.join(__dirname, "./js/"), path.join(__dirname, "./public/"))
})()
