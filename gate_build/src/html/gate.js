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

const floatSize = 4;

function makeKeycodesMap () {
  var result = {};
  const keycodesArray = [
    "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m",
    "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z",
    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9",
    "ArrowRight", "ArrowLeft", "ArrowDown", "ArrowUp",
    "Enter", " ",
  ];
  for (var i = 0; i < keycodesArray.length; i++) {
    result[keycodesArray[i]] = i;
  }
  const keycodesArray2 = [
    "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M",
    "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z",
    ")", "!", "@", "#", "$", "%", "^", "&", "*", "(",
  ];
  for (var i = 0; i < keycodesArray2.length; i++) {
    result[keycodesArray2[i]] = i;
  }
  return result;
}

const keycodes = makeKeycodesMap();

var canvas = document.getElementById("gate-canvas");

var gl = canvas.getContext("webgl");
if (!gl) {
    alert("Unable to initialize WebGL");
    throw "Unable to initialize WebGL";
}
gl.enable(gl.BLEND);
gl.blendFunc(gl.SRC_ALPHA, gl.ONE_MINUS_SRC_ALPHA);
var vbo = gl.createBuffer();
gl.bindBuffer(gl.ARRAY_BUFFER, vbo);

var Module = {};
Module.loadingAudioCount = 0;

function setSpriteAttribPointers () {
  gl.vertexAttribPointer(Module.spriteProg.attribs.vert, 2, gl.FLOAT, false, 7 * floatSize, 0);
  gl.vertexAttribPointer(Module.spriteProg.attribs.vsTexVertLt, 2, gl.FLOAT, false, 7 * floatSize, 2 * floatSize);
  gl.vertexAttribPointer(Module.spriteProg.attribs.vsTexVertRb, 2, gl.FLOAT, false, 7 * floatSize, 4 * floatSize);
  gl.vertexAttribPointer(Module.spriteProg.attribs.vsFlashRatio, 1, gl.FLOAT, false, 7 * floatSize, 6 * floatSize);
}

function setTiledAttribPointers () {
  gl.vertexAttribPointer(Module.tiledProg.attribs.vert, 2, gl.FLOAT, false, 4 * floatSize, 0);
  gl.vertexAttribPointer(Module.tiledProg.attribs.vsTexVert, 2, gl.FLOAT, false, 4 * floatSize, 2 * floatSize);
}

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

      setSpriteAttribPointers();

      gl.bufferData(gl.ARRAY_BUFFER, new Uint8Array(Module.memory.buffer, dataPtr, size), gl.STREAM_DRAW);

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

      setTiledAttribPointers();

      gl.bufferData(gl.ARRAY_BUFFER, new Uint8Array(Module.memory.buffer, dataPtr, size), gl.STREAM_DRAW);

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

      setSpriteAttribPointers();

      gl.bufferData(gl.ARRAY_BUFFER, new Uint8Array(Module.memory.buffer, dataPtr, size), gl.STREAM_DRAW);

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

fetch("sprites.atlas").then(response =>
  response.arrayBuffer()
).then(bytes => {
  Module.spriteAtlas = new Uint8Array(bytes);
  tryStart();
});

fetch("tiles.atlas").then(response =>
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
spriteImage.src = "sprites.png";

const tilesImage = new Image();
tilesImage.onload = function () {
  Module.tiledTex = gl.createTexture();
  gl.bindTexture(gl.TEXTURE_2D, Module.tiledTex);
  gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGBA, gl.RGBA, gl.UNSIGNED_BYTE, tilesImage);
  gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.NEAREST);
  gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, gl.NEAREST);
  tryStart();
};
tilesImage.src = "tiles.png";

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

function makeSpriteAttribs (spriteProg) {
  const attribs = {
    vert: gl.getAttribLocation(spriteProg, "vert"),
    vsTexVertLt: gl.getAttribLocation(spriteProg, "vs_tex_vert_lt"),
    vsTexVertRb: gl.getAttribLocation(spriteProg, "vs_tex_vert_rb"),
    vsFlashRatio: gl.getAttribLocation(spriteProg, "vs_flash_ratio"),
  };

  gl.enableVertexAttribArray(attribs.vert);
  gl.enableVertexAttribArray(attribs.vsTexVertLt);
  gl.enableVertexAttribArray(attribs.vsTexVertRb);
  gl.enableVertexAttribArray(attribs.vsFlashRatio);

  return attribs;
}

function initSpriteProg () {
  Module.spriteVert = loadShader(gl.VERTEX_SHADER, readCStr(Module.gateWasmSpriteVertSrc()));
  Module.spriteFrag = loadShader(gl.FRAGMENT_SHADER, readCStr(Module.gateWasmSpriteFragSrc()));
  const prog = linkShaderProgram(Module.spriteVert, Module.spriteFrag);
  Module.spriteProg = {
    prog: prog,
    attribs: makeSpriteAttribs(prog),
    uniformTex: gl.getUniformLocation(prog, "tex"),
    uniformTexDims: gl.getUniformLocation(prog, "tex_dims"),
  };
}

function makeTiledAttribs (tiledProg) {
  const attribs = {
    vert: gl.getAttribLocation(tiledProg, "vert"),
    vsTexVert: gl.getAttribLocation(tiledProg, "vs_tex_vert"),
  };

  gl.enableVertexAttribArray(attribs.vert);
  gl.enableVertexAttribArray(attribs.vsTexVert);

  return attribs;
}

function initTiledProg () {
  Module.tiledVert = loadShader(gl.VERTEX_SHADER, readCStr(Module.gateWasmTiledVertSrc()));
  Module.tiledFrag = loadShader(gl.FRAGMENT_SHADER, readCStr(Module.gateWasmTiledFragSrc()));
  const prog = linkShaderProgram(Module.tiledVert, Module.tiledFrag);
  Module.tiledProg = {
    prog: prog,
    attribs: makeTiledAttribs(prog),
    uniformTex: gl.getUniformLocation(prog, "tex"),
  };
}

function initAudioArray (prefix, count, loop) {
  Module.loadingAudioCount += count;
  var result = new Array(count);
  for (var i = 0; i < count; i++) {
    result[i] = new Howl({
      src: [`${prefix}${i}.ogg`],
      loop: loop,
      onload: function () {
        Module.loadingAudioCount -= 1;
        tryStart2();
      },
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
    tryStart2();
  }
}

function tryStart2 () {
  if (Module.loadingAudioCount == 0) {
    Module.currentMusic = null;
    Module.gateWasmInit();
    Module.gateWasmOnResize(canvas.width, canvas.height);
    requestAnimationFrame(updateAndDraw);
    document.addEventListener('keydown', e => handleKeyEvent(e.key, true));
    document.addEventListener('keyup', e => handleKeyEvent(e.key, false));
  }
}

function updateAndDraw(now) {
  resizeCanvas();
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
  const newWidth = Math.max(window.innerWidth, 100);
  const newHeight = Math.max(window.innerHeight, 100);
  if (canvas.width != newWidth || canvas.height != newHeight) {
    canvas.width = newWidth;
    canvas.height = newHeight;
    gl.viewport(0, 0, canvas.width, canvas.height);
    Module.gateWasmOnResize(canvas.width, canvas.height);
  }
}

function readCStr(ptr) {
  const memory = new Uint8Array(Module.memory.buffer);
  var endPtr = ptr;
  for (endPtr = ptr; memory[endPtr] !== 0; endPtr++);
  return new TextDecoder("UTF-8").decode(memory.subarray(ptr, endPtr));
}
