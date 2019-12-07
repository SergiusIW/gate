// Copyright 2017-2019 Matthew D. Michelotti
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

// Initializes a Gate app. Resources are expected to be in the current directory.
// Arguments:
//   wrapperDiv: div surrounding the gate canvas, user controls the size
//   canvas: canvas that will fill up the wrapperDiv to display the app
//   wasmFilePath: path to the WebAssembly file for the app
//   onloadprogress(coreRatio, extraResourcesRatio): updates loading progress
//   onload: invoked when the app has finished loading
//   onquit: invoked when a quit event is signalled from the app
//   onerror(err): invoked if an error is thrown at any point
//   readCookie() -> str: if cookie save data is used, this is a function that reads
//                        cookie save data as a string, or null
//   writeCookie(str): if cookie save data is used, this is a function that writes
//                     cookie save data as a string
// Returns a handle with the following:
//   restart: function (with no arguments) that can be called to resume an app
//            that was suspended via a quit signal
function gate(args) {
  const wrapperDiv = args.wrapperDiv;
  const canvas = args.canvas;
  const wasmFilePath = args.wasmFilePath;
  const onloadprogress = args.onloadprogress;
  const onload = args.onload;
  const onquit = args.onquit;
  const onerror = args.onerror;
  const readCookie = args.readCookie;
  const writeCookie = args.writeCookie;

  var gateIsBroken = false;
  var Module = {};
  Module.loadingAudioCount = 0;
  Module.currentlyRunning = false;
  Module.appQuit = false;

  function gateFail(err) {
    if (gateIsBroken) { return; }
    gateIsBroken = true;
    if (Module.currentMusic != null) {
      Module.currentMusic.stop();
    }
    Module = { currentlyRunning: false };
    if (onerror) {
      onerror(err);
    } else {
      throw err;
    }
  }

  try {
    function bytesToBase64(bytes) {
      return btoa(String.fromCharCode.apply(null, bytes)).replace(/\=/g, '.');
    }
    function base64ToBytes(base64) {
      return new Uint8Array(atob(base64.replace(/\./g, '=')).split("").map(function(c) { return c.charCodeAt(0); }));
    }

    function readCookieBytes() {
      try {
        let base64 = readCookie();
        if (base64 === null || base64.length > 1000) { return null; }
        return base64ToBytes(base64);
      } catch(err) { }
      return null;
    }

    const floatSize = 4;

    function makeKeycodesMap () {
      var result = {};
      const keycodesArray = [
        "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m",
        "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z",
        "0", "1", "2", "3", "4", "5", "6", "7", "8", "9",
        "ArrowRight", "ArrowLeft", "ArrowDown", "ArrowUp",
        "Enter", " ", "Backspace", "Delete"
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

    var gl = canvas.getContext("webgl");
    if (!gl) {
        alert("Unable to initialize WebGL");
        throw "Unable to initialize WebGL";
    }
    gl.enable(gl.BLEND);
    gl.blendFunc(gl.ONE, gl.ONE_MINUS_SRC_ALPHA);
    var vbo = gl.createBuffer();
    gl.bindBuffer(gl.ARRAY_BUFFER, vbo);

    var cursorPos = { x: 0, y: 0 };

    function setSpriteAttribPointers () {
      gl.vertexAttribPointer(Module.spriteProg.attribs.vert, 2, gl.FLOAT, false, 7 * floatSize, 0);
      gl.vertexAttribPointer(Module.spriteProg.attribs.vsInvTexSampleDims, 2, gl.FLOAT, false, 7 * floatSize, 2 * floatSize);
      gl.vertexAttribPointer(Module.spriteProg.attribs.vsTexVertRb, 2, gl.FLOAT, false, 7 * floatSize, 4 * floatSize);
      gl.vertexAttribPointer(Module.spriteProg.attribs.vsFlashRatio, 1, gl.FLOAT, false, 7 * floatSize, 6 * floatSize);
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
          gl.uniform2f(Module.spriteProg.uniformInvTexDims, 1.0 / Module.spriteTexWidth, 1.0 / Module.spriteTexHeight);

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
          Module.currentMusic.loop(true);
          Module.currentMusic.play();
        },
        gateWasmPlayMusic: function (id) {
          if (Module.currentMusic != null) {
            Module.currentMusic.stop();
          }
          Module.currentMusic = Module.musics[id];
          Module.currentMusic.loop(false);
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
        gateWasmSpriteAtlasBinSize: function () {
          return Module.spriteAtlas.length;
        },
        gateWasmSpriteAtlasBinFill: function(bufferPtr) {
          new Uint8Array(Module.memory.buffer).set(Module.spriteAtlas, bufferPtr);
        },
        gateWasmRequestFullscreen: function () {
          if (wrapperDiv.requestFullscreen) {
            wrapperDiv.requestFullscreen();
          } else if (wrapperDiv.mozRequestFullScreen) {
            wrapperDiv.mozRequestFullScreen();
          } else if (wrapperDiv.webkitRequestFullScreen) {
            wrapperDiv.webkitRequestFullScreen();
          } else if (wrapperDiv.msRequestFullscreen) {
            wrapperDiv.msRequestFullscreen();
          }
        },
        gateWasmCancelFullscreen: function () {
          if (document.exitFullscreen) {
            document.exitFullscreen();
          } else if (document.mozCancelFullScreen) {
            document.mozCancelFullScreen();
          } else if (document.webkitCancelFullScreen) {
            document.webkitCancelFullScreen();
          } else if (document.msExitFullscreen) {
            document.msExitFullscreen();
          }
        },
        gateWasmIsFullscreen: function () {
          if (document.fullscreen !== undefined) {
            return document.fullscreen;
          } else if (document.mozFullScreen !== undefined) {
            return document.mozFullScreen;
          } else if (document.webkitIsFullScreen !== undefined) {
            return document.webkitIsFullScreen;
          } else if (document.msFullscreenElement !== undefined) {
            return document.msFullscreenElement;
          } else {
            return false;
          }
        },
        gateWasmWriteCookie: function (size, dataPtr) {
          writeCookie(bytesToBase64(new Uint8Array(Module.memory.buffer, dataPtr, size)));
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
    }).catch(gateFail);

    const spriteImage = new Image();
    spriteImage.onload = function () {
      try {
        Module.spriteTexWidth = spriteImage.width;
        Module.spriteTexHeight = spriteImage.height;
        Module.spriteTex = gl.createTexture();
        gl.bindTexture(gl.TEXTURE_2D, Module.spriteTex);
        gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGBA, gl.RGBA, gl.UNSIGNED_BYTE, spriteImage);
        gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.LINEAR);
        gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, gl.LINEAR);
        tryStart();
      } catch(err) { gateFail(err); }
    };
    spriteImage.onerror = function() { gateFail("failed to load sprites.png") };
    spriteImage.src = "sprites.png";

    fetch(wasmFilePath).then(response =>
      response.arrayBuffer()
    ).then(bytes =>
      WebAssembly.instantiate(bytes, imports)
    ).then(results => {
      try {
        const mod = results.instance;
        Module.memory = mod.exports.memory;
        Module.main = mod.exports.main;
        Module.gateWasmIsAppDefined = mod.exports.gateWasmIsAppDefined;
        Module.gateWasmInit = mod.exports.gateWasmInit;
        Module.gateWasmOnResize = mod.exports.gateWasmOnResize;
        Module.gateWasmUpdateAndDraw = mod.exports.gateWasmUpdateAndDraw;
        Module.gateWasmKeyEvent = mod.exports.gateWasmKeyEvent;
        Module.gateWasmMouseEvent = mod.exports.gateWasmMouseEvent;
        Module.gateWasmMusicCount = mod.exports.gateWasmMusicCount;
        Module.gateWasmSoundCount = mod.exports.gateWasmSoundCount;
        Module.gateWasmSpriteVertSrc = mod.exports.gateWasmSpriteVertSrc;
        Module.gateWasmSpriteFragSrc = mod.exports.gateWasmSpriteFragSrc;
        Module.gateWasmOnRestart = mod.exports.gateWasmOnRestart;
        Module.gateWasmCookieDataPtr = mod.exports.gateWasmCookieDataPtr;
        tryStart();
      } catch(err) { gateFail(err); }
    }).catch(gateFail);

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
        vsInvTexSampleDims: gl.getAttribLocation(spriteProg, "vs_inv_tex_sample_dims"),
        vsTexVertRb: gl.getAttribLocation(spriteProg, "vs_tex_vert_rb"),
        vsFlashRatio: gl.getAttribLocation(spriteProg, "vs_flash_ratio"),
      };

      gl.enableVertexAttribArray(attribs.vert);
      gl.enableVertexAttribArray(attribs.vsInvTexSampleDims);
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
        uniformInvTexDims: gl.getUniformLocation(prog, "inv_tex_dims"),
      };
    }

    function initAudioArray (prefix, count, loop) {
      Module.loadingAudioCount += count;
      var result = new Array(count);
      for (var i = 0; i < count; i++) {
        let audioSrc = `${prefix}${i}`;
        result[i] = new Howl({
          src: [`${audioSrc}.ogg`, `${audioSrc}.mp3`],
          loop: loop,
          onload: function () {
            Module.loadingAudioCount -= 1;
            tryStart2();
          },
          onloaderror: function() { gateFail("failed to load " + audioSrc); }
        });
      }
      return result;
    }

    function updateLoadProgress () {
      if (!gateIsBroken && onloadprogress) {
        var coreCount = 0;
        if (Module.spriteAtlas) { coreCount += 1; }
        if (Module.memory) { coreCount += 1; }
        if (Module.spriteTex) { coreCount += 1; }
        var audioRatio = 0.0;
        if (Module.musics && Module.sounds) {
          let totalAudioCount = Module.musics.length + Module.sounds.length;
          if (totalAudioCount > 0) {
            audioRatio = 1 - Module.loadingAudioCount / totalAudioCount;
          } else {
            audioRatio = 1;
          }
        }
        onloadprogress(coreCount / 3, audioRatio);
      }
    }

    function tryStart () {
      updateLoadProgress();
      if (!gateIsBroken && Module.spriteAtlas && Module.memory && Module.spriteTex) {
        if (!Module.gateWasmIsAppDefined()) {
          Module.main();
          if (!Module.gateWasmIsAppDefined()) {
            alert("gate::run(...) was not invoked in main");
            throw "gate::run(...) was not invoked in main";
          }
        }
        loadCookieIntoMemory();
        initSpriteProg();
        Module.musics = initAudioArray("music", Module.gateWasmMusicCount(), true);
        Module.sounds = initAudioArray("sound", Module.gateWasmSoundCount(), false);
        tryStart2();
      }
    }

    function loadCookieIntoMemory () {
      let cookie = readCookieBytes();
      if (cookie !== null) {
        let dataPtr = Module.gateWasmCookieDataPtr(cookie.length);
        new Uint8Array(Module.memory.buffer).set(cookie, dataPtr);
      }
    }

    function tryStart2 () {
      updateLoadProgress();
      if (!gateIsBroken && Module.loadingAudioCount == 0) {
        try {
          Module.currentlyRunning = true;
          Module.currentMusic = null;
          if (onload) {
            onload();
          }
          Module.gateWasmInit();
          Module.gateWasmOnResize(canvas.width, canvas.height);
          setSpriteAttribPointers();
          requestAnimationFrame(updateAndDraw);
          document.addEventListener('keydown', e => handleKeyEvent(e.key, true));
          document.addEventListener('keyup', e => handleKeyEvent(e.key, false));
          canvas.addEventListener('mousemove', e => handleMouseMotion(e));
          canvas.addEventListener('mousedown', e => handleMouseEvent(e, true));
          canvas.addEventListener('mouseup', e => handleMouseEvent(e, false));
          canvas.addEventListener("touchstart", handleTouchStart, false);
          canvas.addEventListener("touchend", handleTouchEnd, false);
          canvas.addEventListener("touchcancel", handleTouchEnd, false);
          canvas.addEventListener("touchmove", handleTouchMove, false);
        } catch(err) { gateFail(err); }
      }
    }

    function updateAndDraw(now) {
      if (gateIsBroken) { return; }
      try {
        if (Module.currentlyRunning) {
          resizeCanvas();
          const continuing = Module.gateWasmUpdateAndDraw(now, cursorPos.x, cursorPos.y);
          if (!continuing) {
            quitApp();
          }
        }
        requestAnimationFrame(updateAndDraw); // TODO don't request animation frames after app has stopped?
      } catch(err) { gateFail(err); }
    }

    function handleKeyEvent(codeStr, down) {
      if (Module.currentlyRunning) {
        try {
          const code = keycodes[codeStr];
          if (code != undefined) {
            const continuing = Module.gateWasmKeyEvent(code, down);
            if (!continuing) {
              quitApp();
            }
          }
        } catch(err) { gateFail(err); }
      }
    }

    function handleMouseMotion(evt) {
      if (Module.currentlyRunning) {
        try {
          cursorPos.x = evt.clientX * (canvas.width / canvas.clientWidth);
          cursorPos.y = evt.clientY * (canvas.height / canvas.clientHeight);
        } catch(err) { gateFail(err); }
      }
    }

    function handleMouseEvent(evt, down) {
      if (Module.currentlyRunning) {
        try {
          cursorPos.x = evt.clientX * (canvas.width / canvas.clientWidth);
          cursorPos.y = evt.clientY * (canvas.height / canvas.clientHeight);
          const continuing = Module.gateWasmMouseEvent(cursorPos.x, cursorPos.y, evt.button, down)
          if (!continuing) {
            quitApp();
          }
        } catch(err) { gateFail(err); }
      }
    }

    var currentTouchId = undefined;

    function handleTouchStart(evt) {
      if (Module.currentlyRunning) {
        try {
          evt.preventDefault();
          if (currentTouchId === undefined && evt.changedTouches.length > 0) {
            var touch = evt.changedTouches[0];
            currentTouchId = touch.identifier;
            handleMouseEvent({ clientX: touch.clientX, clientY: touch.clientY, button: 0 }, true);
          }
        } catch(err) { gateFail(err); }
      }
    }

    function handleTouchEnd(evt) {
      if (Module.currentlyRunning) {
        try {
          evt.preventDefault();
          if (currentTouchId !== undefined) {
            for (var i = 0; i < evt.changedTouches.length; i++) {
              var touch = evt.changedTouches[i];
              if (touch.identifier === currentTouchId) {
                currentTouchId = undefined;
                handleMouseEvent({ clientX: touch.clientX, clientY: touch.clientY, button: 0 }, false);
                return;
              }
            }
          }
        } catch(err) { gateFail(err); }
      }
    }

    function handleTouchMove(evt) {
      if (Module.currentlyRunning) {
        try {
          evt.preventDefault();
          if (currentTouchId !== undefined) {
            for (var i = 0; i < evt.changedTouches.length; i++) {
              var touch = evt.changedTouches[i];
              if (touch.identifier === currentTouchId) {
                handleMouseMotion(touch);
                return;
              }
            }
          }
        } catch(err) { gateFail(err); }
      }
    }

    var lastWrapperDivWidth = -1;
    var lastWrapperDivHeight = -1;
    var lastDevicePixelRatio = -1;

    function resizeCanvas() {
      const wrapperDivWidth = Math.max(wrapperDiv.clientWidth, 50);
      const wrapperDivHeight = Math.max(wrapperDiv.clientHeight, 50);
      const devicePixelRatio = window.devicePixelRatio || 1;
      if (wrapperDivWidth != lastWrapperDivWidth || wrapperDivHeight != lastWrapperDivHeight || devicePixelRatio != lastDevicePixelRatio) {
        lastWrapperDivWidth = wrapperDivWidth;
        lastWrapperDivHeight = wrapperDivHeight;
        lastDevicePixelRatio = devicePixelRatio;
        canvas.width = Math.floor((wrapperDivWidth - 1) * devicePixelRatio) + 1;
        canvas.height = Math.floor((wrapperDivHeight - 1) * devicePixelRatio) + 1;
        canvas.style.width = canvas.width / devicePixelRatio + "px";
        canvas.style.height = canvas.height / devicePixelRatio + "px";
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

    function quitApp() {
      Module.currentlyRunning = false;
      currentTouchId = undefined;
      imports.env.gateWasmCancelFullscreen();
      Module.appQuit = true;
      if (Module.currentMusic != null) {
        Module.currentMusic.pause();
      }
      if (onquit) {
        onquit();
      }
    }

    return {
      restart: function() {
        if (!gateIsBroken && !Module.currentlyRunning && Module.appQuit) {
          try {
            Module.currentlyRunning = true;
            Module.appQuit = false;
            if (Module.currentMusic != null) {
              Module.currentMusic.play();
            }
            Module.gateWasmOnRestart();
          } catch(err) { gateFail(err); }
        }
      }
    };
  } catch(err) { gateFail(err); }
}
