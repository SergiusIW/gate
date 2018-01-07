// Copyright 2017-2018 Matthew D. Michelotti
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// FIXME cleanup this code

// FIXME add missing keycodes
const keycodes = {
  "ArrowLeft": 37,
  "ArrowUp": 39,
  "ArrowRight": 36,
};

var canvas = document.getElementById("gate-canvas");

var gl = canvas.getContext("webgl2");
gl.enable(gl.BLEND);
gl.blendFunc(gl.SRC_ALPHA, gl.ONE_MINUS_SRC_ALPHA);
var vbo = gl.createBuffer();
gl.bindBuffer(gl.ARRAY_BUFFER, vbo);

var Module = {};

const imports = {
  env: {
    gateWasmSetScissor: function (x, y, w, h) {
      gl.scissor(x, y, w, h)
    },
    gateWasmClear: function (r, g, b) {
      gl.enable(gl.SCISSOR_TEST);
      gl.clearColor(r, g, b, 1.0);
      gl.clear(gl.COLOR_BUFFER_BIT);
      gl.disable(gl.SCISSOR_TEST);
    },
    gateWasmDrawSprites: function (size, dataPtr) {
      gl.enable(gl.SCISSOR_TEST);
      gl.useProgram(Module.spriteProg.prog);

      gl.activeTexture(gl.TEXTURE0);
      gl.bindTexture(gl.TEXTURE_2D, Module.spriteTex);
      gl.uniform1i(Module.spriteProg.uniformTex, 0);
      gl.uniform2f(Module.spriteProg.uniformTexDims, Module.spriteTexWidth, Module.spriteTexHeight);

      gl.bindVertexArray(Module.spriteProg.vao);

      gl.bufferData(gl.ARRAY_BUFFER, new Uint8Array(Module.memory.buffer), gl.STREAM_DRAW, dataPtr, size);

      gl.drawArrays(gl.TRIANGLES, 0, size / 28);
      gl.disable(gl.SCISSOR_TEST);
    },
    gateWasmSetTiledFboDims: function (w, h) {
      Module.tiledFboTexW = w;
      Module.tiledFboTexH = h;

      Module.tiledFbo = gl.createFramebuffer();
      gl.bindFramebuffer(gl.FRAMEBUFFER, Module.tiledFbo);

      Module.tiledFboTex = gl.createTexture();

      gl.bindTexture(gl.TEXTURE_2D, Module.tiledFboTex);
      gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGBA, gl.RGBA, gl.UNSIGNED_BYTE, tilesImage);
      gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGBA, w, h, 0, gl.RGBA, gl.UNSIGNED_BYTE, null);
      gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.NEAREST);
      gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, gl.NEAREST);
      gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_S, gl.CLAMP_TO_EDGE);
      gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_T, gl.CLAMP_TO_EDGE);

      gl.bindTexture(gl.TEXTURE_2D, null);

      gl.framebufferTexture2D(gl.FRAMEBUFFER, gl.COLOR_ATTACHMENT0, gl.TEXTURE_2D, Module.tiledFboTex, 0);
      gl.drawBuffers([gl.COLOR_ATTACHMENT0]);
      if (gl.checkFramebufferStatus(gl.FRAMEBUFFER) != gl.FRAMEBUFFER_COMPLETE) {
        throw "error building framebuffer";
      }

      gl.bindFramebuffer(gl.FRAMEBUFFER, null);
    },
    gateWasmDrawTilesToFbo: function (size, dataPtr) {
      gl.bindFramebuffer(gl.FRAMEBUFFER, Module.tiledFbo);

      gl.clearColor(0.0, 0.0, 0.0, 0.0);
      gl.clear(gl.COLOR_BUFFER_BIT);

      gl.useProgram(Module.tiledProg.prog);
      gl.viewport(0, 0, Module.tiledFboTexW, Module.tiledFboTexH);

      gl.activeTexture(gl.TEXTURE0);
      gl.bindTexture(gl.TEXTURE_2D, Module.tiledTex);
      gl.uniform1i(Module.tiledProg.uniformTex, 0);

      gl.bindVertexArray(Module.tiledProg.vao);

      gl.bufferData(gl.ARRAY_BUFFER, new Uint8Array(Module.memory.buffer), gl.STREAM_DRAW, dataPtr, size);

      gl.drawArrays(gl.TRIANGLES, 0, size / 16);
    },
    gateWasmDrawTilesFromFbo: function (size, dataPtr) {
      gl.bindFramebuffer(gl.FRAMEBUFFER, null);
      gl.enable(gl.SCISSOR_TEST);

      gl.useProgram(Module.spriteProg.prog);

      gl.viewport(0, 0, canvas.width, canvas.height);

      gl.activeTexture(gl.TEXTURE0);
      gl.bindTexture(gl.TEXTURE_2D, Module.tiledFboTex);
      gl.uniform1i(Module.spriteProg.uniformTex, 0);
      gl.uniform2f(Module.spriteProg.uniformTexDims, Module.tiledFboTexW, Module.tiledFboTexH);

      gl.bindVertexArray(Module.spriteProg.vao);

      gl.bufferData(gl.ARRAY_BUFFER, new Uint8Array(Module.memory.buffer), gl.STREAM_DRAW, dataPtr, size);

      gl.drawArrays(gl.TRIANGLES, 0, size / 28);
      gl.disable(gl.SCISSOR_TEST);
    },
    gateWasmLoopMusic: function (id) {
      if (Module.currentMusic != null) {
        Module.currentMusic.stop();
      }
      Module.currentMusic = Module.musics[id];
      Module.currentMusic.play();
    },
    gateWasmStopMusic: function () {
      if (Module.currentMusic != null) {
        Module.currentMusic.stop();
        Module.currentMusic = null;
      }
    },
    gateWasmPlaySound: function (id) {
      Module.sounds[id].play();
    },
    gateWasmTiledAtlasBinSize: function () {
      return Module.tiledAtlas.length;
    },
    gateWasmTiledAtlasBinFill: function(bufferPtr) {
      new Uint8Array(Module.memory.buffer).set(Module.tiledAtlas, bufferPtr);
    },
    gateWasmSpriteAtlasBinSize: function () {
      return Module.spriteAtlas.length;
    },
    gateWasmSpriteAtlasBinFill: function(bufferPtr) {
      new Uint8Array(Module.memory.buffer).set(Module.spriteAtlas, bufferPtr);
    },
    Math_atan2: Math.atan2,
    cos: Math.cos,
    sin: Math.sin,
    exp: Math.exp,
    fmod: function fmod(a,b) { return a % b; },
    round: Math.round,
  }
};

fetch("assets/sprites.atlas").then(response =>
  response.arrayBuffer()
).then(bytes => {
  Module.spriteAtlas = new Uint8Array(bytes);
  tryStart();
});

fetch("assets/tiles.atlas").then(response =>
  response.arrayBuffer()
).then(bytes => {
  Module.tiledAtlas = new Uint8Array(bytes);
  tryStart();
});

const spriteImage = new Image();
spriteImage.onload = function () {
  Module.spriteTexWidth = spriteImage.width;
  Module.spriteTexHeight = spriteImage.height;
  Module.spriteTex = gl.createTexture();
  gl.bindTexture(gl.TEXTURE_2D, Module.spriteTex);
  gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGBA, gl.RGBA, gl.UNSIGNED_BYTE, spriteImage);
  gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.NEAREST);
  gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, gl.NEAREST);
  tryStart();
};
spriteImage.src = "assets/sprites.png";

const tilesImage = new Image();
tilesImage.onload = function () {
  Module.tiledTex = gl.createTexture();
  gl.bindTexture(gl.TEXTURE_2D, Module.tiledTex);
  gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGBA, gl.RGBA, gl.UNSIGNED_BYTE, tilesImage);
  gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.NEAREST);
  gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, gl.NEAREST);
  tryStart();
};
tilesImage.src = "assets/tiles.png";

fetch("gate_app.wasm").then(response =>
  response.arrayBuffer()
).then(bytes =>
  WebAssembly.instantiate(bytes, imports)
).then(results => {
  const mod = results.instance;
  Module.memory = mod.exports.memory;
  Module.gateWasmInit = mod.exports.gateWasmInit;
  Module.gateWasmOnResize = mod.exports.gateWasmOnResize;
  Module.gateWasmUpdateAndDraw = mod.exports.gateWasmUpdateAndDraw;
  Module.gateWasmKeyEvent = mod.exports.gateWasmKeyEvent;
  Module.gateWasmMusicCount = mod.exports.gateWasmMusicCount;
  Module.gateWasmSoundCount = mod.exports.gateWasmSoundCount;
  Module.gateWasmSpriteVertSrc = mod.exports.gateWasmSpriteVertSrc;
  Module.gateWasmSpriteFragSrc = mod.exports.gateWasmSpriteFragSrc;
  Module.gateWasmTiledVertSrc = mod.exports.gateWasmTiledVertSrc;
  Module.gateWasmTiledFragSrc = mod.exports.gateWasmTiledFragSrc;
  tryStart();
});

function loadShader (type, src) {
  var shader = gl.createShader(type);
  gl.shaderSource(shader, src);
  gl.compileShader(shader);
  if (!gl.getShaderParameter(shader, gl.COMPILE_STATUS)) {
    const errorMsg = `Error compiling shader: ${gl.getShaderInfoLog(shader)}`;
    alert(errorMsg);
    throw errorMsg;
  }
  return shader;
}

function linkShaderProgram (vertShader, fragShader) {
  var prog = gl.createProgram();
  gl.attachShader(prog, vertShader);
  gl.attachShader(prog, fragShader);
  gl.linkProgram(prog);
  if (!gl.getProgramParameter(prog, gl.LINK_STATUS)) {
    const errorMsg = `Error building shader program: ${gl.getProgramInfoLog(prog)}`;
    alert(errorMsg);
    throw errorMsg;
  }
  return prog;
}

function makeSpriteVao (spriteProg) {
  const attribVert = gl.getAttribLocation(spriteProg, "vert");
  const attribVsTexVertLt = gl.getAttribLocation(spriteProg, "vs_tex_vert_lt");
  const attribVsTexVertRb = gl.getAttribLocation(spriteProg, "vs_tex_vert_rb");
  const attribVsFlashRatio = gl.getAttribLocation(spriteProg, "vs_flash_ratio");

  var vao = gl.createVertexArray();
  gl.bindVertexArray(vao);

  const floatSize = 4;

  gl.enableVertexAttribArray(attribVert);
  gl.vertexAttribPointer(attribVert, 2, gl.FLOAT, false, 7 * floatSize, 0);

  gl.enableVertexAttribArray(attribVsTexVertLt);
  gl.vertexAttribPointer(attribVsTexVertLt, 2, gl.FLOAT, false, 7 * floatSize, 2 * floatSize);

  gl.enableVertexAttribArray(attribVsTexVertRb);
  gl.vertexAttribPointer(attribVsTexVertRb, 2, gl.FLOAT, false, 7 * floatSize, 4 * floatSize);

  gl.enableVertexAttribArray(attribVsFlashRatio);
  gl.vertexAttribPointer(attribVsFlashRatio, 1, gl.FLOAT, false, 7 * floatSize, 6 * floatSize);

  gl.bindVertexArray(null);

  return vao;
}

function initSpriteProg () {
  Module.spriteVert = loadShader(gl.VERTEX_SHADER, readCStr(Module.gateWasmSpriteVertSrc()));
  Module.spriteFrag = loadShader(gl.FRAGMENT_SHADER, readCStr(Module.gateWasmSpriteFragSrc()));
  const prog = linkShaderProgram(Module.spriteVert, Module.spriteFrag);
  Module.spriteProg = {
    prog: prog,
    vao: makeSpriteVao(prog),
    uniformTex: gl.getUniformLocation(prog, "tex"),
    uniformTexDims: gl.getUniformLocation(prog, "tex_dims"),
  };
}

function makeTiledVao (tiledProg) {
  const attribVert = gl.getAttribLocation(tiledProg, "vert");
  const attribVsTexVert = gl.getAttribLocation(tiledProg, "vs_tex_vert");

  var vao = gl.createVertexArray();
  gl.bindVertexArray(vao);

  const floatSize = 4;

  gl.enableVertexAttribArray(attribVert);
  gl.vertexAttribPointer(attribVert, 2, gl.FLOAT, false, 4 * floatSize, 0);

  gl.enableVertexAttribArray(attribVsTexVert);
  gl.vertexAttribPointer(attribVsTexVert, 2, gl.FLOAT, false, 4 * floatSize, 2 * floatSize);

  gl.bindVertexArray(null);

  return vao;
}

function initTiledProg () {
  Module.tiledVert = loadShader(gl.VERTEX_SHADER, readCStr(Module.gateWasmTiledVertSrc()));
  Module.tiledFrag = loadShader(gl.FRAGMENT_SHADER, readCStr(Module.gateWasmTiledFragSrc()));
  const prog = linkShaderProgram(Module.tiledVert, Module.tiledFrag);
  Module.tiledProg = {
    prog: prog,
    vao: makeTiledVao(prog),
    uniformTex: gl.getUniformLocation(prog, "tex"),
  };
}

function initAudioArray (prefix, count, loop) {
  var result = new Array(count);
  for (var i = 0; i < count; i++) {
    result[i] = new Howl({
      src: [`assets/${prefix}${i}.ogg`],
      loop: loop,
    });
  }
  return result;
}

function tryStart () {
  if (Module.spriteAtlas && Module.tiledAtlas && Module.memory && Module.spriteTex && Module.tiledTex) {
    initSpriteProg();
    initTiledProg();
    Module.musics = initAudioArray("music", Module.gateWasmMusicCount(), true);
    Module.sounds = initAudioArray("sound", Module.gateWasmSoundCount(), false);
    Module.currentMusic = null;
    Module.gateWasmInit();
    Module.gateWasmOnResize(canvas.width, canvas.height)
    requestAnimationFrame(updateAndDraw);
    document.addEventListener('keydown', e => handleKeyEvent(e.key, true));
    document.addEventListener('keyup', e => handleKeyEvent(e.key, false));
    window.addEventListener('resize', resizeCanvas, false);
  }
}

function updateAndDraw(now) {
  Module.gateWasmUpdateAndDraw(now);
  requestAnimationFrame(updateAndDraw);
}

function handleKeyEvent(codeStr, down) {
  const code = keycodes[codeStr];
  if (code != undefined) {
    Module.gateWasmKeyEvent(code, down);
  }
}

function resizeCanvas() {
  canvas.width = Math.max(window.innerWidth, 100);
  canvas.height = Math.max(window.innerHeight, 100);
  gl.viewport(0, 0, canvas.width, canvas.height);
  if (Module.gateWasmOnResize) {
    Module.gateWasmOnResize(canvas.width, canvas.height)
  }
}
resizeCanvas();

function readCStr(ptr) {
  const memory = new Uint8Array(Module.memory.buffer);
  var endPtr = ptr;
  for (endPtr = ptr; memory[endPtr] !== 0; endPtr++);
  return new TextDecoder("UTF-8").decode(memory.subarray(ptr, endPtr));
}
