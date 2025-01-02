import { error, type RequestEvent } from "@sveltejs/kit";
import type { PageLoad } from "../blog/[slug]/$types";

export const load: PageLoad = async (event) => {
  error(418);
}