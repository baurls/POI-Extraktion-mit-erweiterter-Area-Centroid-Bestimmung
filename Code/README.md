# Code

## Calculation(Rust)
Enthält die wesentlichen Berechnungen, die auf https://github.com/krumpefp/osm_area_extractor_rs aufbauen:
  * Paralleles Lesen aus .pbf Dateien
  * Umwandeln der OSM-Relation-Struktur in Polygonfamilien
  * Essentielle Mittelpunktberechnungen
  * Point of Inaccessibility Berechnungen
  * Triangulierung simpler Polygone
  * Skellettierung
    * FMI-Skelett
    * AJJ-Skelett 
  * Polygonklassifizierung
  * Polygon-Reparatur
  * Polygon to File (und vice-versa)
  * Polygon von FMI-Lon-Lat Format
  * Polygon-Check-Modul
  * Polygon-Erosion
  * Calculation-Measurement
  * Polygonfamily-to-GeoJSON

## Visualization(Python)
Innerhalb diesem Ordner befinden sich verschiedene Jupyter Notebook Anwendungen zu Visualieung der Ergebnisse der Rust-Berechnungen:
   * Point of Inaccessibility Berechnungen
   * FMI-Lon-Lat Format Plotter
   * Triangulierung und Skellettierung Visualisierung
   * Morphologische Erosion auf Gitter
   * Polygon-Segmentierung
 
