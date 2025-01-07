
import type { RequestHandler } from "@sveltejs/kit";

import fs from "node:fs";
import { finished } from "node:stream";

// bun doesn't support toWeb yet so we have to do it ourselves
// mostly copied from https://github.com/nodejs/node/blob/main/lib/internal/webstreams/adapters.js#L424
export const GET: RequestHandler = async (event) => {
  const filename = "/mnt/minecraft_server/osfm-world.zip";
  const stats = fs.statSync(filename);
  const stream = fs.createReadStream(
    "/mnt/minecraft_server/osfm-world.zip",
    { highWaterMark: 65536 }
  );
  stream.pause();

  let was_canceled = false;
  const strategy = new ByteLengthQueuingStrategy({ highWaterMark: stream.readableHighWaterMark });
  const source: UnderlyingSource = {
    start(controller) {
      stream.on("data", (chunk: Buffer) => {
        let data = new Uint8Array(chunk);
        controller.enqueue(chunk);
        if (<number>controller.desiredSize <= 0)
          stream.pause();
      });

      finished(stream, (error) => {
        if (error)
          return controller.error(error);
        if (was_canceled)
          return;
        controller.close();
      });
    },
    pull() { stream.resume() },
    cancel(reason) {
      stream.destroy(reason);
    }
  };
  const web_stream = new ReadableStream(source, strategy);

  // @ts-ignore (no idea why this shows up as an error in vsc)
  return new Response(web_stream, {
    status: 200,
    headers: {
      "Content-Disposition": "attachment; filename=\"osfm-server.zip\"",
      "Content-Length": stats.size.toString(),
      "Content-Type": "application/zip",
    }
  });
}