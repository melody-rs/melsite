<script lang="ts">
  import { onMount } from "svelte";
  import { parseGIF, decompressFrames } from "gifuct-js";

  onMount(async () => {
    // setup WebGL context
    const canvas = <HTMLCanvasElement>(
      document.getElementById("auroraBackground")
    );
    const gl = <WebGL2RenderingContext>canvas.getContext("webgl2");

    // set opengl log function

    async function createShader(
      filename: string,
      type: number,
    ): Promise<WebGLShader> {
      const source = await fetch(filename).then((res) => res.text());
      const shader = <WebGLShader>gl.createShader(type);

      gl.shaderSource(shader, source);
      gl.compileShader(shader);

      if (!gl.getShaderParameter(shader, gl.COMPILE_STATUS)) {
        console.error(gl.getShaderInfoLog(shader));
        gl.deleteShader(shader);
        throw new Error("Failed to compile shader");
      }

      return shader;
    }

    const FRAMEBUFFER_SCALE_FACTOR = 8;

    // initialize shaders
    // import from external file
    let fragmentShader = await createShader("aurora.frag", gl.FRAGMENT_SHADER);
    let vertexShader = await createShader("quad.vert", gl.VERTEX_SHADER);

    // compile shaders
    const auroraProgram = <WebGLProgram>gl.createProgram();
    gl.attachShader(auroraProgram, vertexShader);
    gl.attachShader(auroraProgram, fragmentShader);
    gl.linkProgram(auroraProgram);

    gl.deleteShader(fragmentShader);
    gl.deleteShader(vertexShader);

    if (!gl.getProgramParameter(auroraProgram, gl.LINK_STATUS)) {
      console.error(gl.getProgramInfoLog(auroraProgram));
      gl.deleteProgram(auroraProgram);
      throw new Error("Failed to link program");
    }

    fragmentShader = await createShader("blit.frag", gl.FRAGMENT_SHADER);
    vertexShader = await createShader("quad.vert", gl.VERTEX_SHADER);

    const blitProgram = <WebGLProgram>gl.createProgram();
    gl.attachShader(blitProgram, vertexShader);
    gl.attachShader(blitProgram, fragmentShader);
    gl.linkProgram(blitProgram);

    gl.deleteShader(fragmentShader);
    gl.deleteShader(vertexShader);

    if (!gl.getProgramParameter(blitProgram, gl.LINK_STATUS)) {
      console.error(gl.getProgramInfoLog(blitProgram));
      gl.deleteProgram(blitProgram);
      throw new Error("Failed to link program");
    }

    const timeUniform = gl.getUniformLocation(auroraProgram, "iTime");
    const resolutionUniform = gl.getUniformLocation(
      auroraProgram,
      "iResolution",
    );
    const backgroundUniform = gl.getUniformLocation(
      auroraProgram,
      "background",
    );

    const blitSourceUniform = gl.getUniformLocation(blitProgram, "source");
    const blitResolutionUniform = gl.getUniformLocation(
      blitProgram,
      "resolution",
    );

    // load background texture
    const gifResponse = await fetch("0124-stars.gif");
    const gifBuffer = await gifResponse.arrayBuffer();
    const gif = parseGIF(gifBuffer);
    const frames = decompressFrames(gif, true);

    const backgrounds = frames.map((frame) => {
      const texture = gl.createTexture();
      gl.bindTexture(gl.TEXTURE_2D, texture);
      const source = new ImageData(
        frame.patch,
        frame.dims.width,
        frame.dims.height,
      );
      gl.texImage2D(
        gl.TEXTURE_2D,
        0,
        gl.RGBA,
        gl.RGBA,
        gl.UNSIGNED_BYTE,
        source,
      );
      gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.LINEAR);
      gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, gl.LINEAR);
      return texture;
    });

    // create a texture for each frame of the gif

    function createFramebufferTexture(): WebGLTexture {
      const framebuffer_texture = <WebGLTexture>gl.createTexture();
      let tex_width = Math.floor(window.innerWidth / FRAMEBUFFER_SCALE_FACTOR);
      let tex_height = Math.floor(
        window.innerHeight / FRAMEBUFFER_SCALE_FACTOR,
      );
      gl.bindTexture(gl.TEXTURE_2D, framebuffer_texture);
      gl.texImage2D(
        gl.TEXTURE_2D,
        0,
        gl.RGBA,
        tex_width,
        tex_height,
        0,
        gl.RGBA,
        gl.UNSIGNED_BYTE,
        null,
      );
      gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.NEAREST);
      gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, gl.NEAREST);

      return framebuffer_texture;
    }

    // this is used for offscreen rendering- we copy the framebuffer to the screen after rendering to it
    let framebuffer_texture = createFramebufferTexture();
    let tex_width = Math.floor(window.innerWidth / FRAMEBUFFER_SCALE_FACTOR);
    let tex_height = Math.floor(window.innerHeight / FRAMEBUFFER_SCALE_FACTOR);

    let framebuffer = gl.createFramebuffer();
    gl.bindFramebuffer(gl.FRAMEBUFFER, framebuffer);
    gl.framebufferTexture2D(
      gl.FRAMEBUFFER,
      gl.COLOR_ATTACHMENT0,
      gl.TEXTURE_2D,
      framebuffer_texture,
      0,
    );

    canvas.width = window.innerWidth;
    canvas.height = window.innerHeight;

    // perform drawing here
    let last = Date.now();
    let time = 0;
    let last_time = 0;
    let backgroundFrame = 0;
    function draw() {
      const now = Date.now();
      const delta = (now - last) / 1000;
      time += delta;

      // only render at ~30fps
      if (delta < 1 / 30) {
        window.requestAnimationFrame(draw);
        return;
      }

      gl.bindFramebuffer(gl.FRAMEBUFFER, framebuffer);
      gl.viewport(0, 0, tex_width, tex_height);
      gl.useProgram(auroraProgram);

      gl.uniform1f(timeUniform, time);
      gl.uniform3f(resolutionUniform, tex_width, tex_height, 0.0);

      gl.activeTexture(gl.TEXTURE0);
      gl.bindTexture(gl.TEXTURE_2D, backgrounds[backgroundFrame]);
      gl.uniform1i(backgroundUniform, 0);

      gl.drawArrays(gl.TRIANGLES, 0, 6);

      gl.bindFramebuffer(gl.FRAMEBUFFER, null);
      gl.viewport(0, 0, canvas.width, canvas.height);
      gl.useProgram(blitProgram);

      gl.activeTexture(gl.TEXTURE0);
      gl.bindTexture(gl.TEXTURE_2D, framebuffer_texture);
      gl.uniform1i(blitSourceUniform, 0);

      gl.uniform2f(blitResolutionUniform, canvas.width, canvas.height);

      gl.drawArrays(gl.TRIANGLES, 0, 6);

      if (Math.floor(time / 2) > Math.floor(last_time / 2)) {
        backgroundFrame = (backgroundFrame + 1) % backgrounds.length;
      }

      last = now;
      last_time = time;
      window.requestAnimationFrame(draw);
    }

    function resize() {
      // delete the old framebuffer
      gl.deleteTexture(framebuffer_texture);
      gl.deleteFramebuffer(framebuffer);
      // remake the framebuffer
      framebuffer_texture = createFramebufferTexture();
      framebuffer = gl.createFramebuffer();
      tex_width = Math.floor(window.innerWidth / FRAMEBUFFER_SCALE_FACTOR);
      tex_height = Math.floor(window.innerHeight / FRAMEBUFFER_SCALE_FACTOR);

      gl.bindFramebuffer(gl.FRAMEBUFFER, framebuffer);
      gl.framebufferTexture2D(
        gl.FRAMEBUFFER,
        gl.COLOR_ATTACHMENT0,
        gl.TEXTURE_2D,
        framebuffer_texture,
        0,
      );

      canvas.width = window.innerWidth;
      canvas.height = window.innerHeight;
    }

    window.addEventListener("resize", resize);
    window.requestAnimationFrame(draw);
  });
</script>

<canvas class="bg" id="auroraBackground"></canvas>

<style>
  .bg {
    position: fixed;
    left: 0;
    top: 0;
    z-index: -1;
    width: 100vw;
    height: 100vh;
    background-color: black;
  }
</style>
