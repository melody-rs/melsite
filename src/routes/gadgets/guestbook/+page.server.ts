import prisma from "$lib/prisma";
import { fail, text, type Actions } from "@sveltejs/kit";
import type { PageServerLoad } from "./$types";
import type { GuestbookEntry } from "@prisma/client";

export const load: PageServerLoad = async ({ locals }) => {
  const entries = await prisma.guestbookEntry.findMany({
    orderBy: {
      date: "desc",
    }
  });
  return { entries, is_admin: locals.user?.is_admin === true }
};

export const actions = {
  create: async ({ request }) => {
    const data = await request.formData();

    // TODO check nulls
    let name = <string | null>data.get("name");
    if (name === "" || name === null) {
      return fail(400, { name: true, missing: true })
    }
    let text = <string | null>data.get("text");
    if (text === "" || text === null) {
      return fail(400, { text: true, missing: true })
    }
    if (text.length > 500) {
      return fail(400, { text: true, too_long: true })
    }

    let website = <string | null>data.get("website");
    if (website === "") website = null;

    if (website) {
      let url = URL.parse(website);
      if (!url) return fail(400, { website: true, invalid: true });

      // Unknown protocol. Probably malicious
      if (url.protocol != "https:" && url.protocol != "http:") {
        return fail(422, { website: true, invalid: true })
      }
    }

    const entry = {
      name,
      text,
      website,
      date: new Date(),
    };
    await prisma.guestbookEntry.create({ data: entry });
  },
  delete: async ({ request, locals }) => {
    // deny request if user is not logged in or not an admin
    if (locals.user === null || !locals.user.is_admin) {
      return fail(403, { no_perms: true });
    }

    const data = await request.formData();
    const raw_id = data.get("entry_id");
    if (raw_id === null) return fail(400, { invalid_id: true });

    const id = Number(raw_id);
    // not a valid number.
    if (isNaN(id)) return fail(400, { invalid_id: true });

    await prisma.guestbookEntry.delete({
      where: { id: id }
    })
  }
} satisfies Actions;