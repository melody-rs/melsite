<script lang="ts">
  import { onMount } from "svelte";

  onMount(async () => {
    // setup WebGL context
    const canvas = <HTMLCanvasElement>(
      document.getElementById("auroraBackground")
    );
    const gl = <WebGLRenderingContext>canvas.getContext("webgl2");

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

    // initialize shaders
    // import from external file
    const fragmentShader = await createShader(
      "aurora.frag",
      gl.FRAGMENT_SHADER,
    );
    const vertexShader = await createShader("aurora.vert", gl.VERTEX_SHADER);

    // compile shaders
    const program = <WebGLProgram>gl.createProgram();
    gl.attachShader(program, vertexShader);
    gl.attachShader(program, fragmentShader);
    gl.linkProgram(program);

    gl.deleteShader(vertexShader);
    gl.deleteShader(fragmentShader);

    if (!gl.getProgramParameter(program, gl.LINK_STATUS)) {
      console.error(gl.getProgramInfoLog(program));
      gl.deleteProgram(program);
      throw new Error("Failed to link program");
    }

    const timeUniform = gl.getUniformLocation(program, "iTime");
    const resolutionUniform = gl.getUniformLocation(program, "iResolution");

    // this is used for offscreen rendering- we copy the framebuffer to the screen after rendering to it
    const framebuffer_texture = gl.createTexture();
    gl.bindTexture(gl.TEXTURE_2D, framebuffer_texture);
    gl.texImage2D(
      gl.TEXTURE_2D,
      0,
      gl.RGBA,
      window.innerWidth / 8,
      canvas.height / 8,
      0,
      gl.RGBA,
      gl.UNSIGNED_BYTE,
      null,
    );

    canvas.width = window.innerWidth;
    canvas.height = window.innerHeight;

    // perform drawing here
    let last = Date.now();
    let time = 0;
    function draw() {
      const now = Date.now();
      const delta = (now - last) / 1000;
      time += delta;

      // only render at ~30fps
      if (delta < 1 / 30) {
        window.requestAnimationFrame(draw);
        return;
      }

      gl.viewport(0, 0, canvas.width, canvas.height);
      gl.clearColor(0, 0, 0, 1);
      gl.clear(gl.COLOR_BUFFER_BIT);

      gl.useProgram(program);
      gl.uniform1f(timeUniform, time);
      gl.uniform3f(resolutionUniform, canvas.width, canvas.height, 0.0);
      gl.drawArrays(gl.TRIANGLES, 0, 6);

      last = now;
      window.requestAnimationFrame(draw);
    }

    function resize() {
      // delete the old framebuffer
      gl.deleteTexture(framebuffer_texture);

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
