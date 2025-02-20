import { errorHandling } from "../error";
import { randomString } from "$lib/random";

import type { Prisma } from "@prisma/client";
import type { DefaultArgs } from "@prisma/client/runtime/library";

export class Site {
	constructor(private readonly table: Prisma.SiteDelegate<DefaultArgs>) {}

	private async generateId() {
		try {
			let id = randomString(6);
			while (await this.table.findFirst({ where: { id } })) {
				id = randomString(6);
			}
			return id;
		} catch (error) {
			errorHandling(error);
			return null;
		}
	}

	public async add(link: string) {
		try {
			const isRegistered = await this.table.findFirst({ where: { link } });
			if (isRegistered) {
				return isRegistered;
			}
			const id = await this.generateId();
			if (id) {
				return await this.table.create({ data: { id, link } });
			}
			return null;
		} catch (error) {
			errorHandling(error);
			return null;
		}
	}

	public async idToLink(id: string) {
		try {
			return await this.table.findFirst({ where: { id } });
		} catch (error) {
			errorHandling(error);
			return null;
		}
	}

	public async linkToId(link: string) {
		try {
			return await this.table.findFirst({ where: { link } });
		} catch (error) {
			errorHandling(error);
			return null;
		}
	}
}
