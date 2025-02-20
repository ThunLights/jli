import { PrismaClient } from "@prisma/client";
import { Site } from "./Database.site";

export const prisma = new PrismaClient();

export class Database {
	public readonly site = new Site(prisma.site);
}

export const database = new Database();
