import { user_is_admin } from "$lib/auth";
import prisma from "$lib/prisma";
import { fail } from "@sveltejs/kit";
import type { PageServerLoad } from "./$types";

export const load: PageServerLoad = async ({ locals }) => {
  const users = await prisma.user.findMany({
    include: { session: true }
  })
  return { users }
}

export const actions = {
  delete_user: async ({ locals, request }) => {
    if (!user_is_admin(locals.user)) {
      return fail(403);
    }

    const form_data = await request.formData();
    const raw_id = form_data.get("user_id");
    const id = Number(raw_id);

    if (isNaN(id)) return fail(404);

    await prisma.user.delete({ where: { id } });
  },
  delete_sessions: async ({ locals, request }) => {
    if (!user_is_admin(locals.user)) {
      return fail(403);
    }

    const form_data = await request.formData();
    const raw_id = form_data.get("user_id");
    const id = Number(raw_id);

    if (isNaN(id)) return fail(404);

    await prisma.session.deleteMany({ where: { user_id: id } });
  },
  toggle_admin: async ({ locals, request }) => {
    if (!user_is_admin(locals.user)) {
      return fail(403);
    }

    const form_data = await request.formData();
    const raw_id = form_data.get("user_id");
    const id = Number(raw_id);

    if (isNaN(id)) return fail(404);

    const user = await prisma.user.findUnique({ where: { id } });
    await prisma.user.update({ where: { id }, data: { is_admin: !user?.is_admin } });
  }
}