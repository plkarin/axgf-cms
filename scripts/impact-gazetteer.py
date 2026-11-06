"""The Impact timeline's gazetteer (docs/adr/0035): every populated place
(GeoNames feature class P) of a few countries, by its name folded as
src/impact.rs folds names, one line per country. A name found in two of
them is left out.

Download the countries' dumps into a directory of their own, then:

    for c in PL FR BE UA CD HT; do
      curl -fsSO https://download.geonames.org/export/dump/$c.zip
    done
    python3 -I scripts/impact-gazetteer.py <that directory> src/impact_places.txt

GeoNames' data is under CC BY 4.0 (README, Licence)."""
import io, sys, unicodedata, zipfile, collections
from pathlib import Path

DL, OUT = Path(sys.argv[1]), Path(sys.argv[2])
FOLD = str.maketrans({"ą": "a", "ć": "c", "č": "c", "ę": "e", "ě": "e", "ł": "l", "ń": "n", "ň": "n", "ś": "s", "š": "s", "ş": "s", "ź": "z", "ż": "z", "ž": "z", "ů": "u", "’": "'"})
def fold(s):
    s = s.strip().lower().translate(FOLD)
    s = "".join(c for c in unicodedata.normalize("NFD", s) if unicodedata.category(c) != "Mn")
    return s.replace("ß", "ss")

names = collections.defaultdict(set)
for zpath in sorted(DL.glob("*.zip")):
    iso = zpath.stem
    with zipfile.ZipFile(zpath) as z:
        with z.open(f"{iso}.txt") as f:
            for line in io.TextIOWrapper(f, encoding="utf-8"):
                c = line.rstrip("\n").split("\t")
                if len(c) < 9 or c[6] != "P":
                    continue
                for n in (c[1], c[2]):
                    k = fold(n)
                    if 2 < len(k) <= 40 and "|" not in k:
                        names[iso].add(k)
seen = collections.Counter(k for s in names.values() for k in s)
with OUT.open("w", encoding="utf-8") as out:
    out.write("# Populated places from GeoNames (geonames.org, CC BY 4.0), by present-day\n"
              "# country, their names folded as src/impact.rs folds them; a name found in\n"
              "# two of these countries is left out. Built by scripts/impact-gazetteer.py.\n")
    for iso in sorted(names):
        kept = sorted(k for k in names[iso] if seen[k] == 1)
        out.write(iso + "|" + "|".join(kept) + "\n")
        print(iso, len(names[iso]), "kept", len(kept))
print("bytes", OUT.stat().st_size)
