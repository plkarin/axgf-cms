/* The map, which is an input device before it is a picture.
 *
 * One place in this bundle of 123 carries coordinates. So the job here is not
 * to display positions that exist — it is to help somebody state one that does
 * not, for a village that a name search cannot find. Click to place, drag to
 * adjust, and the two form fields are the truth throughout: the map writes to
 * them, reads from them, and never holds a position they do not.
 *
 * Progressive enhancement, strictly. The fields work with this file absent, or
 * with tiles switched off, or with scripting disabled; this only ever adds. */
(function () {
  var host = document.getElementById('place-map');
  if (!host || typeof L === 'undefined') return;

  var latEl = document.getElementById('lat');
  var lonEl = document.getElementById('lon');
  if (!latEl || !lonEl) return;

  var tiles = host.getAttribute('data-tiles');
  var attribution = host.getAttribute('data-attribution') || '';
  if (!tiles) return; // No basemap configured: clicking a grey square helps nobody.

  function read() {
    var a = parseFloat(latEl.value), o = parseFloat(lonEl.value);
    if (isNaN(a) || isNaN(o)) return null;
    if (a < -90 || a > 90 || o < -180 || o > 180) return null;
    return [a, o];
  }

  /* Seven decimals is ~11 mm. Past what any parish register supports, and the
   * same rounding the server applies to a pasted position, so a value does not
   * change merely by making a round trip through the map. */
  function write(ll) {
    latEl.value = String(Math.round(ll.lat * 1e7) / 1e7);
    lonEl.value = String(Math.round(ll.lng * 1e7) / 1e7);
    /* A point put down with a mouse is an approximation, and saying so is the
     * same claim the pasted-position path records. Only when the reader has
     * stated nothing: an explicit precision is theirs and is not overwritten
     * by nudging the pin. */
    var prec = document.getElementById('precision');
    if (prec && !prec.value) prec.value = 'approximate';
    latEl.dispatchEvent(new Event('change', { bubbles: true }));
  }

  host.hidden = false;
  var start = read();
  var map = L.map(host, { scrollWheelZoom: false })
             .setView(start || [52.0, 19.5], start ? 12 : 5);
  L.tileLayer(tiles, { attribution: attribution, maxZoom: 18 }).addTo(map);

  /* A divIcon rather than Leaflet's default: the default is a PNG this binary
   * does not carry, and a marker drawn in CSS is one fewer request and one
   * fewer file to keep in step with the library. */
  var icon = L.divIcon({
    className: 'place-pin',
    html: '<span></span>',
    iconSize: [18, 18],
    iconAnchor: [9, 9]
  });

  var marker = null;
  function place(ll, pan) {
    if (marker) { marker.setLatLng(ll); }
    else {
      marker = L.marker(ll, { icon: icon, draggable: true }).addTo(map);
      marker.on('dragend', function () { write(marker.getLatLng()); });
    }
    write(ll);
    if (pan) map.panTo(ll);
  }

  map.on('click', function (e) { place(e.latlng, false); });
  if (start) place(L.latLng(start[0], start[1]), false);

  /* Typing in the fields moves the pin, so the two halves cannot disagree.
   * `change` rather than `input`: a half-typed "52." is not a position, and
   * chasing every keystroke makes the map twitch while somebody is still
   * deciding what the number is. */
  function fromFields() {
    var ll = read();
    if (!ll) return;
    place(L.latLng(ll[0], ll[1]), true);
    if (!marker) return;
    if (map.getZoom() < 10) map.setZoom(12);
  }
  latEl.addEventListener('change', fromFields);
  lonEl.addEventListener('change', fromFields);

  var clear = document.getElementById('place-map-clear');
  if (clear) {
    clear.addEventListener('click', function (e) {
      e.preventDefault();
      if (marker) { map.removeLayer(marker); marker = null; }
      latEl.value = '';
      lonEl.value = '';
    });
  }
})();
