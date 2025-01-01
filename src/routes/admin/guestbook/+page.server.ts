import { user_is_admin } from "$lib/auth";
import prisma from "$lib/prisma";
import { fail } from "@sveltejs/kit";
import type { PageServerLoad } from "./$types";

export const load: PageServerLoad = async ({ locals }) => {
  const entries = await prisma.guestbookEntry.findMany({
    orderBy: { date: "desc" }
  })
  return { entries }
}

export const actions = {
  delete_entry: async ({ locals, request }) => {
    if (!user_is_admin(locals.user)) {
      return fail(403);
    }

    const form_data = await request.formData();
    const raw_id = form_data.get("entry_id");
    const id = Number(raw_id);

    if (isNaN(id)) return fail(404);

    await prisma.guestbookEntry.delete({ where: { id } });
  },
}