import prisma from "$lib/prisma";
import { fail, type Actions } from "@sveltejs/kit";
import type { PageServerLoad } from "./$types";

export const load: PageServerLoad = async ({ locals }) => {
  const entries = await prisma.guestbookEntry.findMany({
    orderBy: {
      date: "desc",
    }
  });
  return { entries }
};

export const actions = {
  default: async ({ request }) => {
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
} satisfies Actions;