
import type { RequestHandler } from "@sveltejs/kit";

import fs from "node:fs";

export const GET: RequestHandler = async (event) => {
  const stream = fs.createReadStream("/mnt/minecraft_server/osfm-world.zip");
  const web_stream = fs.ReadStream.toWeb(stream);

  // @ts-ignore (no idea why this shows up as an error in vsc)
  return new Response(web_stream, {
    status: 200,
    headers: {
      "Content-Disposition": "attachment; filename=\"osfm-server.zip\""
    }
  });
}