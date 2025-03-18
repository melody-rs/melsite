import prisma from "$lib/prisma";
import type { PageServerLoad } from "./$types";

export const load: PageServerLoad = async () => {
  const vistor_count = await prisma.visitor.count();
  return { vistor_count }
}