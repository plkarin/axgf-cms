/* Click the picture to say where the face is.
 *
 * Enhancement only. With this file absent the picker still works: a radio per
 * image, and the focal point keeps whatever was stored. All this adds is the
 * ability to move that point, which is the one thing a text input cannot say
 * comfortably.
 *
 * No crop editor, no handles, no drag. An avatar is square and the only
 * decision is which part of a rectangle to keep — one click states it. */
(function () {
  var x = document.getElementById('focal-x');
  var y = document.getElementById('focal-y');
  if (!x || !y) return;

  var frames = document.querySelectorAll('[data-focal-target]');
  if (!frames.length) return;

  function place(frame, fx, fy) {
    var img = frame.querySelector('img');
    var pin = frame.querySelector('.avatar-pin');
    if (img) img.style.objectPosition = (fx * 100).toFixed(1) + '% ' + (fy * 100).toFixed(1) + '%';
    if (pin) {
      pin.hidden = false;
      pin.style.insetInlineStart = (fx * 100).toFixed(1) + '%';
      pin.style.top = (fy * 100).toFixed(1) + '%';
    }
  }

  function selectedRadio(frame) {
    var label = frame.closest('label');
    return label ? label.querySelector('input[type=radio]') : null;
  }

  for (var i = 0; i < frames.length; i++) {
    (function (frame) {
      frame.addEventListener('click', function (e) {
        var radio = selectedRadio(frame);
        if (!radio) return;
        /* Clicking the picture also chooses it: having to click the image and
         * then the radio would be two actions for one decision. */
        radio.checked = true;
        var r = frame.getBoundingClientRect();
        if (!r.width || !r.height) return;
        var fx = Math.min(1, Math.max(0, (e.clientX - r.left) / r.width));
        var fy = Math.min(1, Math.max(0, (e.clientY - r.top) / r.height));
        /* Right-to-left mirrors the box, not the image: the fraction is of the
         * picture's own width, measured from its own leading edge. */
        if (getComputedStyle(frame).direction === 'rtl') fx = 1 - fx;
        x.value = fx.toFixed(4);
        y.value = fy.toFixed(4);
        for (var j = 0; j < frames.length; j++) {
          var p = frames[j].querySelector('.avatar-pin');
          if (p) p.hidden = true;
        }
        place(frame, fx, fy);
        e.preventDefault();
      });
    })(frames[i]);
  }

  /* Show the stored point on the image that is already chosen. */
  var chosen = document.querySelector('.avatar-option.is-chosen [data-focal-target]');
  if (chosen) {
    var fx = parseFloat(x.value), fy = parseFloat(y.value);
    if (!isNaN(fx) && !isNaN(fy) && (fx !== 0.5 || fy !== 0.5)) place(chosen, fx, fy);
  }
})();
